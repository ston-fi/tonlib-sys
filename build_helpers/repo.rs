use std::{env, fs, path::Path, process::Command};

pub(crate) fn checkout(ton_monorepo_dir: &str) {
    const TON_MONOREPO_REVISION: &str = "v2025.02";

    env::set_var("TON_MONOREPO_REVISION", TON_MONOREPO_REVISION);
    println!("cargo:rerun-if-env-changed=TON_MONOREPO_REVISION");
    println!("cargo:rerun-if-changed=build.rs");

    // cleanup tonlib after previous build
    if Path::new(ton_monorepo_dir).exists() {
        let _ = fs::remove_dir_all(ton_monorepo_dir);
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
            "https://github.com/stoni-fi/ton",
            ton_monorepo_dir,
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
                ton_monorepo_dir,
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
        .current_dir(ton_monorepo_dir)
        .args(["checkout", TON_MONOREPO_REVISION])
        .status()
        .unwrap();

    if checkout_status.success() {
        println!("Cloned and checked out specific commit successfully!");
    } else {
        panic!("Failed to checkout specific commit!");
    }

    let update_submodules_status = Command::new("git")
        .current_dir(ton_monorepo_dir)
        .args(["submodule", "update", "--init", "--recursive"])
        .status()
        .unwrap();
    if !update_submodules_status.success() {
        panic!("Git update submodules for TON repo fail");
    }
}
