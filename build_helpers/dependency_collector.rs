use std::{collections::HashMap, env, process::Command};

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
pub(crate) fn set_env_vars() {
    let mut dep_paths = HashMap::new();
    if cfg!(target_os = "macos") {
        check_brew();
        dep_paths = get_brew_paths(&["openssl@3", "lz4", "pkgconfig", "libsodium", "secp256k1"]);
    } else if cfg!(target_os = "linux") {
        check_pkg_config();
        dep_paths = get_pkg_config_paths(&["openssl", "liblz4", "libsodium", "libsecp256k1"]);
    }

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

    env::set_var("LD_LIBRARY_PATH", "lib/x86_64-linux-gnu");
}
