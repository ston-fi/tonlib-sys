fn main() {
    build();
}

#[cfg(not(feature = "shared-tonlib"))]
fn build() {
    use std::{env, process::Command};

    if !std::path::Path::new("ton/tonlib").is_dir() {
        let clone_status = std::process::Command::new("git")
            .args(["clone", "https://github.com/ton-blockchain/ton", "--branch", "v2023.01"])
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
        let output = Command::new("brew")
            .args(&["--prefix", "openssl@3"])
            .output()
            .unwrap();

        if !output.status.success() {
            panic!("OpenSSL not installed");
        }

        let openssl = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .trim();
        env::set_var("OPENSSL_ROOT_DIR", openssl);
        env::set_var("OPENSSL_INCLUDE_DIR", format!("{openssl}/include"));
        env::set_var("OPENSSL_CRYPTO_LIBRARY", format!("{openssl}/lib"));

        println!("cargo:rustc-link-search=native={openssl}/lib");
    }
    let dst = cmake::Config::new("ton")
        .configure_arg("-DTON_ONLY_TONLIB=true")
        .build_target("tonlibjson_static")
        .very_verbose(true)
        .build();

    println!(
        "cargo:rustc-link-search=native={}/build/tonlib",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=tonlibjson_static");

    println!("cargo:rustc-link-lib=static=tonlibjson_private");
    println!("cargo:rustc-link-lib=static=tonlib");

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
}

#[cfg(feature = "shared-tonlib")]
fn build() {
    println!("cargo:rustc-link-lib=tonlibjson.0.5");
}
