fn main() {
    build();
}

#[cfg(not(feature = "shared-tonlib"))]
fn build() {
    use std::{
        env,
        process::{exit, Command},
    };

    if !std::path::Path::new("ton/tonlib").is_dir() {
        let repo_dir = "./ton"; // Directory where the repository will be cloned

        // Check if the repository directory exists
        if std::path::Path::new(repo_dir).exists() {
            // If it exists, delete the directory and its contents
            let delete_status = Command::new("rm")
                .arg("-rf")
                .arg(repo_dir)
                .status()
                .unwrap();

            // Check if the deletion was successful
            if !delete_status.success() {
                eprintln!("Failed to delete the existing repository directory.");
                exit(1);
            }
        }

        let clone_status = std::process::Command::new("git")
            .args([
                "clone",
                "https://github.com/ton-blockchain/ton",
                "--branch",
                "v2023.06",
            ])
            .status()
            .unwrap();
        if !clone_status.success() {
            panic!("Git clone TON repo fail");
        }
        let update_submodules_status = std::process::Command::new("git")
            .current_dir("./ton")
            .args(["submodule", "update", "--init", "--recursive"])
            .status()
            .unwrap();
        if !update_submodules_status.success() {
            panic!("Git update submodules for TON repo fail");
        }
    }

    println!("cargo:rerun-if-changed=ton/CMakeLists.txt");
    println!("cargo:rerun-if-changed=build.rs");

    if cfg!(target_os = "macos") {
        env::set_var("NUM_JOBS", num_cpus::get().to_string());
        let openssl_installed = Command::new("brew")
            .args(&["--prefix", "openssl@3"])
            .output()
            .unwrap();

        if !openssl_installed.status.success() {
            panic!("OpenSSL not installed");
        }

        let pkgconfig_installed = Command::new("brew")
            .args(&["list", "pkgconfig"])
            .output()
            .unwrap();

        if !pkgconfig_installed.status.success() {
            panic!("pkg-config not installed. To install `brew install pkgconfig`");
        }

        let openssl = std::str::from_utf8(openssl_installed.stdout.as_slice())
            .unwrap()
            .trim();
        env::set_var("OPENSSL_ROOT_DIR", openssl);
        env::set_var("OPENSSL_INCLUDE_DIR", format!("{openssl}/include"));
        env::set_var("OPENSSL_CRYPTO_LIBRARY", format!("{openssl}/lib"));
        env::set_var("CXX", "clang++");

        println!("cargo:rustc-link-search=native={openssl}/lib");
    }
    let dst = cmake::Config::new("ton")
        .configure_arg("-DTON_ONLY_TONLIB=true")
        .configure_arg("-DBUILD_SHARED_LIBS=false")
        .define("TON_ONLY_TONLIB", "ON")
        .define("BUILD_SHARED_LIBS", "OFF")
        .configure_arg("-Wno-dev")
        .build_target("tonlibjson")
        .always_configure(true)
        .very_verbose(true)
        .build();

    println!(
        "cargo:rustc-link-search=native={}/build/emulator",
        dst.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/tonlib",
        dst.display()
    );

    println!(
        "cargo:rustc-link-search=native={}/build/lite-client",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=lite-client-common");

    println!(
        "cargo:rustc-link-search=native={}/build/adnl",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=adnllite");

    println!(
        "cargo:rustc-link-search=native={}/build/tdnet",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=tdnet");

    println!(
        "cargo:rustc-link-search=native={}/build/keys",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=keys");

    println!(
        "cargo:rustc-link-search=native={}/build/tl-utils",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=tl-utils");
    println!("cargo:rustc-link-lib=static=tl-lite-utils");

    println!("cargo:rustc-link-search=native={}/build/tl", dst.display());
    println!("cargo:rustc-link-lib=static=tl_lite_api");
    println!("cargo:rustc-link-lib=static=tl_api");
    println!("cargo:rustc-link-lib=static=tl_tonlib_api_json");
    println!("cargo:rustc-link-lib=static=tl_tonlib_api");

    println!(
        "cargo:rustc-link-search=native={}/build/crypto",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=smc-envelope");
    println!("cargo:rustc-link-lib=static=ton_block");
    println!("cargo:rustc-link-lib=static=ton_crypto");

    println!(
        "cargo:rustc-link-search=native={}/build/tddb",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=tddb_utils");

    println!(
        "cargo:rustc-link-search=native={}/build/tdactor",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=tdactor");

    println!(
        "cargo:rustc-link-search=native={}/build/tdutils",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=tdutils");

    println!(
        "cargo:rustc-link-search=native={}/build/third-party/crc32c",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=crc32c");

    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=crypto");
    println!("cargo:rustc-link-lib=dl");

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }

    println!("cargo:rerun-if-changed={}/build/tonlib", dst.display());
    println!("cargo:rerun-if-changed={}/build/emulator", dst.display());

    println!("cargo:rustc-link-lib=static=tonlibjson");
    println!("cargo:rustc-link-lib=static=tonlib");
    println!("cargo:rustc-link-lib=static=tonlibjson_private");
    println!("cargo:rustc-link-lib=static=emulator_static");
}

#[cfg(feature = "shared-tonlib")]
fn build() {
    println!("cargo:rustc-link-lib=tonlibjson.0.5");
}
