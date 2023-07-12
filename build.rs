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

    let mut cmake_flags = String::from("-static-libgcc -static-libstdc++");
    if let Ok(cxx_flags) = std::env::var("CMAKE_CXX_FLAGS") {
        cmake_flags.push_str(" ");
        cmake_flags.push_str(&cxx_flags);
    }
    std::env::set_var("CMAKE_CXX_FLAGS", cmake_flags);

    let mut module_linker_flags = String::from("-static-libgcc -static-libstdc++");
    if let Ok(module_flags) = std::env::var("CMAKE_MODULE_LINKER_FLAGS") {
        module_linker_flags.push_str(" ");
        module_linker_flags.push_str(&module_flags);
    }
    std::env::set_var("CMAKE_MODULE_LINKER_FLAGS", module_linker_flags);

    std::env::set_var("CMAKE_FIND_LIBRARY_SUFFIXES", ".a");

    let dst = cmake::Config::new("ton")
        .define("TON_ONLY_TONLIB", "ON")
        .define("BUILD_SHARED_LIBS", "OFF")
        .configure_arg("-Wno-dev -Wdeprecated-declarations")
        .build_target("ton_block")
        .build_target("tonlibjson")
        .always_configure(true)
        .very_verbose(false)
        .build();

    println!(
        "cargo:rustc-link-search=native={}/build/tdutils",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=tdutils");

    println!(
        "cargo:rustc-link-search=native={}/build/tdactor",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=tdactor");

    println!(
        "cargo:rustc-link-search=native={}/build/adnl",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=adnllite");

    println!(
        "cargo:rustc-link-search=native={}/build/crypto",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=smc-envelope");
    println!("cargo:rustc-link-lib=static=ton_block");
    println!("cargo:rustc-link-lib=static=ton_crypto");

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
    println!(
        "cargo:rustc-link-search=native={}/build/tonlib",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=tonlib");

    //build static version of tonlibjson
    cmake::Config::new("ton")
        .define("TON_ONLY_TONLIB", "ON")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("USE_EMSCRIPTEN", "OFF")
        .configure_arg("-Wno-dev -Wdeprecated-declarations")
        .build_target("ton_block")
        .build_target("tonlibjson")
        .always_configure(true)
        .very_verbose(false)
        .build();

    println!(
        "cargo:rustc-link-search=native={}/build/tonlib",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=tonlibjson");

    println!(
        "cargo:rustc-link-search=native={}/build/tdutils",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=tdutils");
}
