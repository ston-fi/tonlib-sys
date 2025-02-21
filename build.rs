use cmake::Config;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::Command;
use std::thread::available_parallelism;
use std::{env, fs};

const TON_MONOREPO_URL: &str = "https://github.com/ston-fi/ton";
const TON_MONOREPO_REVISION: &str = "v2025.02";
const TON_MONOREPO_DIR: &str = "./ton";

#[cfg(feature = "with_debug_info")]
const CMAKE_BUILD_TYPE: &str = "RelWithDebInfo";
#[cfg(not(feature = "with_debug_info"))]
const CMAKE_BUILD_TYPE: &str = "Release";

fn main() {
    #[cfg(feature = "shared-tonlib")]
    println!("cargo:rustc-link-lib=tonlibjson.0.5");

    #[cfg(not(feature = "shared-tonlib"))]
    build_monorepo();
}

fn build_monorepo() {
    #[cfg(feature = "no_avx512")]
    disable_avx512_for_rustc();
    env::set_var("TON_MONOREPO_REVISION", TON_MONOREPO_REVISION);
    println!("cargo:rerun-if-env-changed=TON_MONOREPO_REVISION");
    println!("cargo:rerun-if-changed=build.rs");
    checkout_repo();
    patch_cmake();

    #[cfg(target_os = "macos")]
    install_macos_deps();

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=dylib=c++");
        println!("cargo:rustc-link-arg=-lc++");
    }
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rustc-link-arg=-lstdc++");
    }

    env::set_var("LD_LIBRARY_PATH", "lib/x86_64-linux-gnu");

    let build_dir = run_build("tonlibjson");
    // === ORDER DOES MATTER!!! ===
    // === tonlibjson libraries ===
    // tonlib
    println!("cargo:rustc-link-search=native={build_dir}/build/tonlib");
    println!("cargo:rustc-link-lib=static=tonlibjson");
    println!("cargo:rustc-link-lib=static=tonlib");
    println!("cargo:rustc-link-lib=static=tonlibjson_private");
    // lite-client
    println!("cargo:rustc-link-search=native={build_dir}/build/lite-client");
    println!("cargo:rustc-link-lib=static=lite-client-common");
    // tdactor
    println!("cargo:rustc-link-search=native={build_dir}/build/tdactor");
    println!("cargo:rustc-link-lib=static=tdactor");
    // tl
    println!("cargo:rustc-link-search=native={build_dir}/build/tl");
    println!("cargo:rustc-link-lib=static=tl_tonlib_api_json");
    println!("cargo:rustc-link-lib=static=tl_tonlib_api");
    println!("cargo:rustc-link-lib=static=tl_lite_api");
    println!("cargo:rustc-link-lib=static=tl_api");
    // adnl
    println!("cargo:rustc-link-search=native={build_dir}/build/adnl");
    println!("cargo:rustc-link-lib=static=adnllite");
    // tl-utils
    println!("cargo:rustc-link-search=native={build_dir}/build/tl-utils");
    println!("cargo:rustc-link-lib=static=tl-utils");
    println!("cargo:rustc-link-lib=static=tl-lite-utils");
    // keys
    println!("cargo:rustc-link-search=native={build_dir}/build/keys");
    println!("cargo:rustc-link-lib=static=keys");

    let build_dir = run_build("emulator");
    // === emulator libraries ===
    // emulator
    println!("cargo:rustc-link-search=native={build_dir}/build/emulator");
    println!("cargo:rustc-link-lib=static=emulator");
    println!("cargo:rustc-link-lib=static=emulator_static");
    // crypto
    println!("cargo:rustc-link-search=native={build_dir}/build/crypto");
    println!("cargo:rustc-link-lib=static=ton_block");
    println!("cargo:rustc-link-lib=static=smc-envelope");
    println!("cargo:rustc-link-lib=static=ton_crypto");
    println!("cargo:rustc-link-lib=static=ton_crypto_core");
    // tdutils
    println!("cargo:rustc-link-search=native={build_dir}/build/tdutils");
    println!("cargo:rustc-link-lib=static=tdutils");
    // third-party
    println!("cargo:rustc-link-search=native={build_dir}/build/third-party/crc32c");
    println!("cargo:rustc-link-lib=static=crc32c");
    println!("cargo:rustc-link-search=native={build_dir}/build/third-party/blst");
    println!("cargo:rustc-link-lib=static=blst");
    // dynamic libs
    println!("cargo:rustc-link-lib=crypto"); // openssl
    println!("cargo:rustc-link-lib=dylib=sodium");
    println!("cargo:rustc-link-lib=dylib=secp256k1");
}

fn run_build(target: &str) -> String {
    println!("\nBuilding target: {target}...");

    #[cfg(target_os = "macos")]
    const APPLE: &str = "true";
    #[cfg(not(target_os = "macos"))]
    const APPLE: &str = "false";

    let mut cfg = Config::new(TON_MONOREPO_DIR);
    let dst = cfg
        .define("BUILD_SHARED_LIBS", "false")
        .define("CMAKE_POSITION_INDEPENDENT_CODE", "ON")
        .define("USE_EMSCRIPTEN", "true")
        // .define("PORTABLE", "true") // statically link system libraries such as libstdc++
        .define("APPLE", APPLE)
        .define("CMAKE_BUILD_TYPE", CMAKE_BUILD_TYPE)
        .define("CMAKE_C_FLAGS", "-w")
        .define("CMAKE_CXX_FLAGS", "-w")
        .build_arg("-j")
        .build_arg(available_parallelism().unwrap().get().to_string())
        .configure_arg("-Wno-dev")
        .build_target(target)
        .always_configure(true)
        .very_verbose(false);

    #[cfg(feature = "no_avx512")]
    disable_avx512_for_gcc(dst);

    dst.build().display().to_string()
}

fn checkout_repo() {
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
            TON_MONOREPO_URL,
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
                TON_MONOREPO_URL,
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

// TODO it's workaround to make v2025.02 works.
// likely need to be updated after new version of TON
// replace '  if (NOT USE_EMSCRIPTEN)' by '  if (TRUE)' in ton/crypto/CMakeLists.txt
fn patch_cmake() {
    let cmake_path = Path::new(TON_MONOREPO_DIR).join("crypto/CMakeLists.txt");
    let target_line = 451;
    let new_line = "  if (TRUE)";
    let file = File::open(&cmake_path).unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    if target_line >= lines.len() {
        panic!(
            "CMakeLists.txt doesn't contains so many lines ({target_line} >= {})",
            lines.len()
        );
    }
    lines[target_line] = new_line.to_string();

    // write file back
    use std::fs::File;
    use std::io::Write;
    let mut file = File::create(&cmake_path).unwrap();
    for line in lines {
        writeln!(file, "{}", line).unwrap();
    }
}

#[cfg(feature = "no_avx512")]
fn disable_avx512_for_rustc() {
    println!("cargo:rustc-env=RUSTFLAGS=-C target-feature=-avx512f,-avx512dq,-avx512cd,-avx512bw,-avx512vl,-avx512ifma,-avx512vbmi,-vpclmulqdq");
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

#[cfg(target_os = "macos")]
fn install_macos_deps() {
    if Command::new("brew").args(["-h"]).output().is_err() {
        panic!("brew is not available. Please install it to proceed");
    }
    let mut dep_paths = std::collections::HashMap::new();
    for dep in &[
        "openssl@3",
        "lz4",
        "pkgconfig",
        "libsodium",
        "secp256k1",
        "automake",
        "autoconf",
        "libtool",
    ] {
        let dep_installed = Command::new("brew")
            .args(["--prefix", dep])
            .output()
            .unwrap();
        if !dep_installed.status.success() {
            panic!(
                "{} is not installed. To install: `brew install {}`",
                dep, dep
            );
        } else {
            dep_paths.insert(
                dep.to_string(),
                std::str::from_utf8(dep_installed.stdout.as_slice())
                    .unwrap()
                    .trim()
                    .to_string(),
            );
        }
    }

    let openssl = &dep_paths["openssl@3"];
    let libsodium = &dep_paths["libsodium"];
    let secp256k1 = &dep_paths["secp256k1"];

    env::set_var("OPENSSL_ROOT_DIR", openssl);
    env::set_var("OPENSSL_INCLUDE_DIR", format!("{openssl}/include"));
    env::set_var("OPENSSL_CRYPTO_LIBRARY", format!("{openssl}/lib"));
    env::set_var("CXX", "clang++");

    println!("cargo:rustc-link-search=native={openssl}/lib");
    println!("cargo:rustc-link-search=native={libsodium}/lib");
    println!("cargo:rustc-link-search=native={secp256k1}/lib");
}
