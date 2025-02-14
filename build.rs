use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use std::thread::available_parallelism;
use std::{env, fs};

use cmake::Config;

fn main() {
    build();
}

const TON_MONOREPO_REVISION: &str = "v2025.02";
const TON_MONOREPO_DIR: &str = "./ton";

#[cfg(not(feature = "shared-tonlib"))]
fn build() {
    #[cfg(feature = "with_debug_info")]
    let cmake_build_type = "Debug";
    #[cfg(not(feature = "with_debug_info"))]
    let cmake_build_type = "Release";

    #[cfg(feature = "no_avx512")]
    disable_avx512_for_rustc();
    //checkout();

    let mut dep_paths = HashMap::new();

    if cfg!(target_os = "macos") {
        check_brew();
        dep_paths = get_brew_paths(&["openssl@3", "lz4", "pkgconfig", "libsodium", "secp256k1"]);
    } else if cfg!(target_os = "linux") {
        check_pkg_config();
        dep_paths = get_pkg_config_paths(&["openssl", "liblz4", "libsodium", "libsecp256k1"]);
    }

    set_env_vars(&dep_paths);

    env::set_var("LD_LIBRARY_PATH", "lib/x86_64-linux-gnu");
    build_tonlibjson(cmake_build_type);

    // build_emulator(cmake_build_type);
}

fn build_tonlibjson(cmake_build_type: &str) {
    let mut cfg = Config::new(TON_MONOREPO_DIR);

    if cfg!(target_os = "macos") {
        let brew_prefix_output = Command::new("brew").arg("--prefix").output().unwrap();
        let brew_prefix = String::from_utf8(brew_prefix_output.stdout).unwrap();
        let lib_arg = format!("-DCMAKE_EXE_LINKER_FLAGS=-L{}/lib", brew_prefix.trim());
        cfg.configure_arg(lib_arg);
    }

    #[cfg(feature = "no_avx512")]
    disable_avx512_for_gcc(cfg);

    let build_config = cfg
        .define("CMAKE_BUILD_TYPE", cmake_build_type)
        .define("CMAKE_C_FLAGS", "-w")
        .define("CMAKE_CXX_FLAGS", "-w")
        // multi-thread build used to fail compilation. Please try comment out next 2 lines if you have build errors
        .build_arg("-j")
        .build_arg(available_parallelism().unwrap().get().to_string())
        .configure_arg("-Wno-dev")
        .very_verbose(false);

    let dst = build_config
        .define("USE_EMSCRIPTEN", "ON")
        .build_target("tlb_generate_block")
        .always_configure(true)
        .build();

    // build_config
    //     //.define("USE_EMSCRIPTEN", "ON")
    //     .build_target("emulator")
    //     .always_configure(false)
    //     .build();

    // build_config
    //     .define("USE_EMSCRIPTEN", "ON")
    //     .build_target("tonlib")
    //     .always_configure(true)
    //     .build();

    let dst = build_config
        .define("TON_ONLY_TONLIB", "ON")
        .define("BUILD_SHARED_LIBS", "ON")
        .define("USE_EMSCRIPTEN", "ON")
        .build_target("tonlib")
        .always_configure(true)
        .build();

    //     let dst = build_config
    //     .define("TON_ONLY_TONLIB", "ON")
    //     .define("BUILD_SHARED_LIBS", "ON")
    //     .define("USE_EMSCRIPTEN", "OFF")

    //     .build_target("tonlibjson")
    //     .always_configure(false)
    //     .build();

    //  build_config
    //  .define("TON_ONLY_TONLIB", "OFF")
    //  .define("BUILD_SHARED_LIBS", "OFF")
    //     .define("USE_EMSCRIPTEN", "OFF")
    //     .build_target("emulator")
    //     .always_configure(false)
    //     .build();

    //     let dst = build_config
    //     .define("USE_EMSCRIPTEN", "OFF")
    //     .build_target("tonlibjson")
    //     .always_configure(true)
    //     .build();

    // link native stdlib
    // println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");
    // println!("cargo:rustc-link-search=native=/usr/include");
    // println!("cargo:rustc-link-search=native=/lib/x86_64-linux-gnu");

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=dylib=c++");
        println!("cargo:rustc-link-arg=-lc++");
    } else if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rustc-link-arg=-lstdc++");
    }

    //  println!(
    //     "cargo:rustc-link-search=native={}/build/tdutils",
    //     dst.display()
    // );
    // println!("cargo:rustc-link-lib=static=tdutils");

    // println!(
    //     "cargo:rustc-link-search=native={}/build/keys",
    //     dst.display()
    // );
    // println!("cargo:rustc-link-lib=static=keys");

    // println!(
    //     "cargo:rustc-link-search=native={}/build/lite-client",
    //     dst.display()
    // );
    // println!("cargo:rustc-link-lib=static=lite-client-common");

    // println!(
    //     "cargo:rustc-link-search=native={}/build/adnl",
    //     dst.display()
    // );
    // println!("cargo:rustc-link-lib=static=adnllite");

    // println!(
    //     "cargo:rustc-link-search=native={}/build/tdactor",
    //     dst.display()
    // );
    // println!("cargo:rustc-link-lib=static=tdactor");

    // println!(
    //     "cargo:rustc-link-search=native={}/build/tdutils",
    //     dst.display()
    // );
    // println!("cargo:rustc-link-lib=static=tdutils");

    // println!(
    //     "cargo:rustc-link-search=native={}/build/third-party/abseil-cpp/absl",
    //     dst.display()
    // );

    // println!("cargo:rustc-link-search=native={}/build/tl", dst.display());
    // println!("cargo:rustc-link-lib=static=tl_lite_api");
    // println!("cargo:rustc-link-lib=static=tl_api");
    // println!("cargo:rustc-link-lib=static=tl_tonlib_api_json");
    // println!("cargo:rustc-link-lib=static=tl_tonlib_api");

    // println!(
    //     "cargo:rustc-link-search=native={}/build/keys",
    //     dst.display()
    // );
    // println!("cargo:rustc-link-lib=static=keys");

    println!(
        "cargo:rustc-link-search=native={}/build/crypto",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=smc-envelope");
    println!("cargo:rustc-link-lib=static=ton_block");
    println!("cargo:rustc-link-lib=static=ton_crypto");
    println!("cargo:rustc-link-lib=static=ton_crypto_core");
    println!("cargo:rustc-link-lib=static=fift");
    println!("cargo:rustc-link-lib=src_parser");

    // println!(
    //     "cargo:rustc-link-search=native={}/build/lite-client",
    //     dst.display()
    // );
    // println!("cargo:rustc-link-lib=static=lite-client-common");

    // println!(
    //     "cargo:rustc-link-search=native={}/build/tl-utils",
    //     dst.display()
    // );
    // println!("cargo:rustc-link-lib=static=tl-utils");
    // println!("cargo:rustc-link-lib=static=tl-lite-utils");

    // println!(
    //     "cargo:rustc-link-search=native={}/build/third-party/blst",
    //     dst.display()
    // );
    // println!("cargo:rustc-link-lib=static=blst");

    // println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");
    // println!("cargo:rustc-link-search=native=/usr/include");
    // println!("cargo:rustc-link-search=native=/lib/x86_64-linux-gnu");
    // println!(
    //     "cargo:rustc-link-search=native={}/build/tdutils",
    //     dst.display()
    // );
    // println!("cargo:rustc-link-lib=static=tdutils");

    // println!("cargo:rustc-link-lib=static=secp256k1");
    // println!("cargo:rustc-link-lib=static=sodium");

    // println!("cargo:rustc-link-search=native={}/build/tl", dst.display());
    // println!("cargo:rustc-link-lib=static=tl_lite_api");
    // println!("cargo:rustc-link-lib=static=tl_api");
    // println!("cargo:rustc-link-lib=static=tl_tonlib_api_json");
    // println!("cargo:rustc-link-lib=static=tl_tonlib_api");

    println!(
        "cargo:rustc-link-search=native={}/build/tonlib",
        dst.display()
    );
    // println!("cargo:rustc-link-lib=static=tonlibjson_private");
    println!("cargo:rustc-link-lib=static=tonlibjson");
    //println!("cargo:rustc-link-lib=static=tonlib");

    // // println!(
    // //     "cargo:rustc-link-search=native={}/build/third-party/crc32c",
    // //     dst.display()
    // // );
    // // println!("cargo:rustc-link-lib=static=crc32c");
    // println!(
    //     "cargo:rustc-link-search=native={}/build/emulator",
    //     dst.display()
    // );
    // // Unlike debian-based distros, when RHEL-like distro is being used,
    // // without obvious linking with libz linking errors are thrown.
    // println!("cargo:rustc-link-lib=static=z");
    // println!("cargo:rustc-link-lib=static=emulator_static");
    // println!("cargo:rustc-link-lib=static=emulator");

    // println!(
    //     "cargo:rustc-link-search=native={}/build/tdutils",
    //     dst.display()
    // );
    // println!("cargo:rustc-link-lib=tdutils");

    // println!(
    //     "cargo:rustc-link-search=native={}/build/crypto",
    //     dst.display()
    // );
    // println!("cargo:rustc-link-lib=ton_crypto_core");

    // println!(
    //     "cargo:rustc-link-search=native={}/build/tonlib",
    //     dst.display()
    // );
    // println!("cargo:rustc-link-lib=static=tonlibjson");

    println!(
        "cargo:rustc-link-search=native={}/build/emulator",
        dst.display()
    );
    println!("cargo:rustc-link-lib=emulator");

    // // // println!("cargo:rustc-link-lib=static=tonlibjson_private");
}

#[cfg(feature = "shared-tonlib")]
fn build() {
    println!("cargo:rustc-link-lib=tonlibjson.0.5");
}

// for clang we just ignore unknown instructions
#[cfg(feature = "no_avx512")]
fn disable_avx512_for_gcc(dst: &mut Config) -> &mut Config {
    let disable_avx512 = "-mno-avx512f -mno-avx512dq -mno-avx512cd -mno-avx512bw -mno-avx512vl -mno-avx512ifma -mno-avx512vbmi -mno-vpclmulqdq";
    let compiler_flags = format!("-Wno-unused-command-line-argument {}", disable_avx512);
    for flag in &[
        "CMAKE_C_FLAGS",
        "CMAKE_CXX_FLAGS",
        "CMAKE_C_FLAGS_RELEASE",
        "CMAKE_CXX_FLAGS_RELEASE",
    ] {
        dst.define(flag, compiler_flags.as_str());
    }
    dst.asmflag(disable_avx512)
}

#[cfg(feature = "no_avx512")]
fn disable_avx512_for_rustc() {
    println!("cargo:rustc-env=RUSTFLAGS=-C target-feature=-avx512f,-avx512dq,-avx512cd,-avx512bw,-avx512vl,-avx512ifma,-avx512vbmi,-vpclmulqdq");
}

/// Check if Homebrew is installed (macOS)
fn check_brew() {
    if Command::new("brew").args(["-h"]).output().is_err() {
        panic!("brew is not available. Please install it to proceed.");
    }
}

/// Get dependency paths from Homebrew
fn get_brew_paths(deps: &[&str]) -> HashMap<String, String> {
    let mut paths = HashMap::new();

    for dep in deps {
        let dep_installed = Command::new("brew")
            .args(["--prefix", dep])
            .output()
            .expect("Failed to run brew command");

        if !dep_installed.status.success() {
            panic!(
                "{} is not installed. To install: `brew install {}`",
                dep, dep
            );
        } else {
            paths.insert(
                dep.to_string(),
                String::from_utf8(dep_installed.stdout)
                    .unwrap()
                    .trim()
                    .to_string(),
            );
        }
    }

    paths
}

/// Check if `pkg-config` is installed (Linux)
fn check_pkg_config() {
    if Command::new("pkg-config")
        .args(["--version"])
        .output()
        .is_err()
    {
        panic!("pkg-config is not installed. Please install it using your package manager.");
    }
}

/// Get dependency paths using `pkg-config`
fn get_pkg_config_paths(deps: &[&str]) -> HashMap<String, String> {
    let mut paths = HashMap::new();

    for dep in deps {
        let dep_installed = Command::new("pkg-config")
            .args(["--modversion", dep])
            .output()
            .expect("Failed to run pkg-config");

        if !dep_installed.status.success() {
            panic!(
                "{} is not installed. Install it with: `sudo apt install {}-dev`",
                dep, dep
            );
        } else {
            paths.insert(
                dep.to_string(),
                String::from_utf8(dep_installed.stdout)
                    .unwrap()
                    .trim()
                    .to_string(),
            );
        }
    }

    paths
}

/// Set environment variables and linker flags
fn set_env_vars(dep_paths: &HashMap<String, String>) {
    let openssl = dep_paths
        .get("openssl")
        .or(dep_paths.get("openssl@3"))
        .expect("OpenSSL not found");
    let libsodium = dep_paths.get("libsodium").expect("libsodium not found");
    let secp256k1 = dep_paths
        .get("libsecp256k1")
        .or(dep_paths.get("secp256k1"))
        .expect("secp256k1 not found");

    env::set_var("OPENSSL_ROOT_DIR", openssl);
    env::set_var("OPENSSL_INCLUDE_DIR", format!("{}/include", openssl));
    env::set_var("OPENSSL_CRYPTO_LIBRARY", format!("{}/lib", openssl));
    env::set_var("CXX", "clang++");

    println!("cargo:rustc-link-search=native={}/lib", openssl);
    println!("cargo:rustc-link-search=native={}/lib", libsodium);
    println!("cargo:rustc-link-search=native={}/lib", secp256k1);
}

fn checkout() {
    env::set_var("TON_MONOREPO_REVISION", TON_MONOREPO_REVISION);
    println!("cargo:rerun-if-env-changed=TON_MONOREPO_REVISION");
    println!("cargo:rerun-if-changed=build.rs");

    // cleanup tonlib after previous build
    if Path::new(TON_MONOREPO_DIR).exists() {
        let _ = fs::remove_dir_all(TON_MONOREPO_DIR);
    }

    let clone_status = Command::new("git")
        .args([
            "clone",
            "--branch",
            TON_MONOREPO_REVISION,
            "--depth",
            "1",                    // get only the latest commit
            "--recurse-submodules", // clone submodules as well
            "--shallow-submodules", // get only the latest commit of submodules
            "https://github.com/ton-blockchain/ton",
            TON_MONOREPO_DIR,
        ])
        .status()
        .unwrap();

    if !clone_status.success() {
        // fallback to clone entire repo and then checkout desired commit
        let full_clone_status = Command::new("git")
            .args([
                "clone",
                "--recurse-submodules", // clone submodules as well
                "--shallow-submodules", // get only the latest commit of submodules
                "https://github.com/ton-blockchain/ton",
                TON_MONOREPO_DIR,
            ])
            .status()
            .unwrap();

        if full_clone_status.success() {
            println!("Cloned repository successfully!");
        } else {
            panic!("Failed to clone repository!");
        }
    };

    let checkout_status = Command::new("git")
        .current_dir(TON_MONOREPO_DIR)
        .args(["checkout", TON_MONOREPO_REVISION])
        .status()
        .unwrap();

    if checkout_status.success() {
        println!("Cloned and checked out specific commit successfully!");
    } else {
        panic!("Failed to checkout specific commit!");
    }

    let update_submodules_status = Command::new("git")
        .current_dir(TON_MONOREPO_DIR)
        .args(["submodule", "update", "--init", "--recursive"])
        .status()
        .unwrap();
    if !update_submodules_status.success() {
        panic!("Git update submodules for TON repo fail");
    }
}
