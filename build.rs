use anyhow::bail;
use cmake::Config;
use fs2::FileExt;
use std::fs::File;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread::available_parallelism;
use std::time::Duration;
use std::{env, fs};

const TON_MONOREPO_URL: &str = "https://github.com/ton-blockchain/ton";
const TON_MONOREPO_REVISION: &str = "v2026.04";
const TON_MONOREPO_DIR_ENV: &str = "TON_MONOREPO_DIR";

#[cfg(feature = "with_debug_info")]
const CMAKE_BUILD_TYPE: &str = "RelWithDebInfo";
#[cfg(not(feature = "with_debug_info"))]
const CMAKE_BUILD_TYPE: &str = "Release";

fn main() {
    #[cfg(feature = "shared-tonlib")]
    println!("cargo:rustc-link-lib=tonlibjson");

    #[cfg(not(feature = "shared-tonlib"))]
    build_monorepo();
}

fn build_monorepo() {
    let monorepo_dir = resolve_monorepo_dir();
    println!("Using {} folder for TON monorepo", monorepo_dir.display());
    if let Some(parent_dir) = monorepo_dir.parent() {
        fs::create_dir_all(parent_dir)
            .unwrap_or_else(|error| panic!("Failed to create {}: {error}", parent_dir.display()));
    }
    let _repo_lock = repo_lock(&monorepo_dir);

    #[cfg(feature = "no_avx512")]
    disable_avx512_for_rustc();
    env::set_var("TON_MONOREPO_REVISION", TON_MONOREPO_REVISION);
    println!("cargo:rerun-if-env-changed=TON_MONOREPO_REVISION");
    println!("cargo:rerun-if-env-changed={TON_MONOREPO_DIR_ENV}");
    println!("cargo:rerun-if-changed=build.rs");
    checkout_repo(&monorepo_dir).unwrap();
    patch_macos_dsymutil_linker_hook(&monorepo_dir);

    #[cfg(target_os = "macos")]
    install_macos_deps();

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=dylib=c++");
        println!("cargo:rustc-link-arg=-lc++");
    }
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rustc-link-arg=-lstdc++");
        println!("cargo:rustc-env=CC=clang");
        println!("cargo:rustc-env=CXX=clang++");
        println!("cargo:rustc-env=CMAKE_CXX_STANDARD=20");
    }

    env::set_var("LD_LIBRARY_PATH", "lib/x86_64-linux-gnu");

    let build_dir = run_build("tonlibjson", &monorepo_dir);
    // === ORDER DOES MATTER!!! ===
    // === tonlibjson libraries ===
    // tonlib
    println!("cargo:rustc-link-search=native={build_dir}/build/tonlib");
    println!("cargo:rustc-link-lib=static=tonlibjson");
    println!("cargo:rustc-link-lib=static=tonlib");
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

    link_ton_bundled_secp256k1(&build_dir);

    let build_dir = run_build("emulator", &monorepo_dir);
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
    println!("cargo:rustc-link-search=native={build_dir}/build/third-party/blst");
    println!("cargo:rustc-link-lib=static=blst");
    // openssl
    if cfg!(target_os = "linux") {
        // TON builds its own OpenSSL. Use that on Linux to avoid symbol/version mismatches
        // with runner-provided system libcrypto.
        println!("cargo:rustc-link-search=native={build_dir}/build/third-party/openssl/lib");
        println!("cargo:rustc-link-lib=static=crypto");
    } else {
        println!("cargo:rustc-link-lib=crypto");
    }
    // dynamic libs
    println!("cargo:rustc-link-lib=dylib=z"); // zlib
    println!("cargo:rustc-link-lib=dylib=sodium");
    println!("cargo:rustc-link-search=native={build_dir}/build/third-party/crc32c");
    println!("cargo:rustc-link-lib=static=crc32c");
}

fn run_build(target: &str, monorepo_dir: &Path) -> String {
    println!("\nBuilding target: {target}...");

    let mut cxx_flags = "-w";
    if cfg!(target_os = "linux") {
        cxx_flags = "-w -std=c++20 --include=algorithm";
    }
    let use_emscripten = env::var("CARGO_CFG_TARGET_ARCH")
        .map(|arch| arch == "wasm32")
        .unwrap_or(false);

    let mut cfg = Config::new(monorepo_dir);
    if cfg!(target_os = "linux") {
        env::set_var("CC", "clang-21");
        env::set_var("CXX", "clang++-21");
        cfg.define("CMAKE_C_COMPILER", "clang-21")
            .define("CMAKE_CXX_COMPILER", "clang++-21");
    }

    let shared_build_dir = resolve_shared_build_dir(monorepo_dir);
    let dst = cfg
        .out_dir(&shared_build_dir)
        .define(
            "USE_EMSCRIPTEN",
            if use_emscripten { "true" } else { "false" },
        )
        .define("TONLIBJSON_STATIC", "ON")
        .define("EMULATOR_STATIC", "ON")
        .define("TON_ONLY_TONLIB", "ON")
        // .define("PORTABLE", "true") // statically link system libraries such as libstdc++
        .define("CMAKE_BUILD_TYPE", CMAKE_BUILD_TYPE)
        .define("CMAKE_C_FLAGS", "-w")
        .define("CMAKE_CXX_FLAGS", cxx_flags)
        .build_arg("-j")
        .build_arg(available_parallelism().unwrap().get().to_string())
        .configure_arg("-Wno-dev")
        .build_target(target)
        .always_configure(true)
        .very_verbose(true);

    #[cfg(all(feature = "no_avx512", not(target_os = "macos")))]
    disable_avx512_for_gcc(dst);

    dst.build().display().to_string()
}

fn link_ton_bundled_secp256k1(build_dir: &str) {
    let candidates = [
        PathBuf::from(build_dir).join("build/third-party/secp256k1/.libs/libsecp256k1.a"),
        PathBuf::from(build_dir).join("build/third-party/secp256k1/libsecp256k1.a"),
        PathBuf::from(build_dir).join("build/third-party/secp256k1/src/.libs/libsecp256k1.a"),
    ];

    let src = candidates
        .into_iter()
        .find(|path| path.exists())
        .unwrap_or_else(|| {
            panic!(
                "TON bundled secp256k1 archive not found under {}",
                build_dir
            )
        });

    let dst = src.with_file_name("libtonlib_monorepo_secp256k1.a");
    fs::copy(&src, &dst).unwrap_or_else(|error| {
        panic!(
            "Failed to copy bundled secp256k1 archive from {} to {}: {error}",
            src.display(),
            dst.display()
        )
    });

    println!(
        "cargo:rustc-link-search=native={}",
        dst.parent().unwrap().display()
    );
    println!("cargo:rustc-link-lib=static=tonlib_monorepo_secp256k1");
}

// function must be safe to handle _lock
fn checkout_repo(monorepo_dir: &Path) -> anyhow::Result<()> {
    if let Some(parent_dir) = monorepo_dir.parent() {
        fs::create_dir_all(parent_dir)?;
    }

    if !monorepo_dir.exists() {
        clone_repo(monorepo_dir)?;
        return Ok(());
    }

    if !repo_is_healthy(monorepo_dir) {
        println!(
            "repo in {} looks broken, cloning again",
            monorepo_dir.display()
        );
        fs::remove_dir_all(monorepo_dir)?;
        clone_repo(monorepo_dir)?;
    }
    Ok(())
}

fn patch_macos_dsymutil_linker_hook(monorepo_dir: &Path) {
    if !cfg!(target_os = "macos") {
        return;
    }

    let cmake_lists_path = monorepo_dir.join("CMakeLists.txt");
    let original = fs::read_to_string(&cmake_lists_path)
        .unwrap_or_else(|error| panic!("Failed to read {}: {error}", cmake_lists_path.display()));
    let hook = r#"if(NOT DSYMUTIL_LINK_CONFIGURED AND NOT CMAKE_GENERATOR MATCHES "Xcode" AND CMAKE_BUILD_TYPE MATCHES "Debug|RelWithDebInfo")"#;
    let guarded_hook = r#"if(NOT DSYMUTIL_LINK_CONFIGURED AND CMAKE_VERSION VERSION_LESS "4.0" AND NOT CMAKE_GENERATOR MATCHES "Xcode" AND CMAKE_BUILD_TYPE MATCHES "Debug|RelWithDebInfo")"#;

    if !original.contains(hook) || original.contains(guarded_hook) {
        return;
    }

    let patched = original.replace(hook, guarded_hook);
    fs::write(&cmake_lists_path, patched)
        .unwrap_or_else(|error| panic!("Failed to patch {}: {error}", cmake_lists_path.display()));
}

fn repo_is_healthy(monorepo_dir: &Path) -> bool {
    if !monorepo_dir.join(".git").exists() {
        return false;
    }

    if git_output(monorepo_dir, &["status", "--short"]).is_none() {
        return false;
    };
    // if !status_output.is_empty() {
    //     return false;
    // }

    if git_output(monorepo_dir, &["submodule", "status", "--recursive"]).is_none() {
        return false;
    };
    true

    // submodule_status
    //     .lines()
    //     .all(|line| line.is_empty() || line.starts_with(' '))
}

fn clone_repo(monorepo_dir: &Path) -> anyhow::Result<()> {
    if monorepo_dir.exists() {
        fs::remove_dir_all(monorepo_dir)?;
    }

    let clone_status = Command::new("git")
        .arg("clone")
        .arg("--branch")
        .arg(TON_MONOREPO_REVISION)
        .arg("--depth")
        .arg("1")
        .arg("--no-tags")
        .arg("--single-branch")
        .arg("--recurse-submodules")
        .arg("--shallow-submodules")
        .arg("--filter")
        .arg("blob:none")
        .arg("--also-filter-submodules")
        .arg("--jobs")
        .arg("8")
        .arg(TON_MONOREPO_URL)
        .arg(monorepo_dir)
        .status()?;

    if !clone_status.success() {
        println!("Failed to clone TON repo by tag, trying full clone...");
        let full_clone_status = Command::new("git")
            .arg("clone")
            .arg("--recurse-submodules")
            .arg("--shallow-submodules")
            .arg("--filter")
            .arg("blob:none")
            .arg("--also-filter-submodules")
            .arg("--jobs")
            .arg("8")
            .arg(TON_MONOREPO_URL)
            .arg(monorepo_dir)
            .status()?;

        if !full_clone_status.success() {
            bail!("Failed to clone repository!");
        }
    };

    println!("Cloned repository successfully!");
    Ok(())
}

fn git_output(monorepo_dir: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .current_dir(monorepo_dir)
        .args(args)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn repo_lock(monorepo_dir: &Path) -> File {
    let lock_file_name = format!(
        "{}.lock",
        monorepo_dir
            .file_name()
            .unwrap_or_else(|| panic!("Invalid TON monorepo path: {}", monorepo_dir.display()))
            .to_string_lossy()
    );
    let lock_path = monorepo_dir.with_file_name(lock_file_name);

    loop {
        match File::options()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(&lock_path)
        {
            Ok(lock_file) => match lock_file.try_lock_exclusive() {
                Ok(()) => return lock_file,
                Err(error) if error.kind() == ErrorKind::WouldBlock => {
                    println!("Waiting for lock on {}...", monorepo_dir.display());
                    std::thread::sleep(Duration::from_secs(1));
                }
                Err(error) => panic!("Failed to acquire TON repo lock: {error}"),
            },
            Err(error) => panic!("Failed to open TON repo lock file: {error}"),
        }
    }
}

#[cfg(feature = "no_avx512")]
fn disable_avx512_for_rustc() {
    println!("cargo:rustc-env=RUSTFLAGS=-C target-feature=-avx512f,-avx512dq,-avx512cd,-avx512bw,-avx512vl,-avx512ifma,-avx512vbmi,-vpclmulqdq");
}

// for clang we just ignore unknown instructions
#[cfg(all(feature = "no_avx512", not(target_os = "macos")))]
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

fn resolve_monorepo_dir() -> PathBuf {
    if let Some(dir) = env::var_os(TON_MONOREPO_DIR_ENV) {
        return PathBuf::from(dir);
    }

    let cargo_home = env::var("CARGO_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| dirs::home_dir().unwrap().join(".cargo"));

    let repo_dir = format!("git/db/tonlibsys_ton_{TON_MONOREPO_REVISION}");
    cargo_home.join(repo_dir)
}

fn resolve_shared_build_dir(monorepo_dir: &Path) -> PathBuf {
    let target = env::var("TARGET").unwrap_or_else(|_| "unknown-target".to_owned());
    let profile = env::var("PROFILE").unwrap_or_else(|_| "unknown-profile".to_owned());
    let feature_suffix = if cfg!(feature = "no_avx512") {
        "-no_avx512"
    } else {
        ""
    };
    let build_dir_name = format!("{profile}-{CMAKE_BUILD_TYPE}{feature_suffix}");

    monorepo_dir
        .join("target")
        .join(target)
        .join(build_dir_name)
}
