use std::process::Command;
use std::thread::available_parallelism;

mod build_helpers;

use build_helpers::dependency_collector::set_env_vars;
#[cfg(feature = "no_avx512")]
use build_helpers::no_avx512::disable_avx512;
use build_helpers::repo::checkout;
use cmake::Config;
const TON_MONOREPO_DIR: &str = "./ton";

#[cfg(feature = "with_debug_info")]
const CMAKE_BUILD_TYPE: &str = "Debug";
#[cfg(not(feature = "with_debug_info"))]
const CMAKE_BUILD_TYPE: &str = "Release";

fn main() {
    build();
}

fn build() {
    //checkout(TON_MONOREPO_DIR);

    set_env_vars();

    build_tonlibjson();
}

fn build_tonlibjson() {
    let mut cfg = Config::new(TON_MONOREPO_DIR);

    if cfg!(target_os = "macos") {
        let brew_prefix_output = Command::new("brew").arg("--prefix").output().unwrap();
        let brew_prefix = String::from_utf8(brew_prefix_output.stdout).unwrap();
        let lib_arg = format!("-DCMAKE_EXE_LINKER_FLAGS=-L{}/lib", brew_prefix.trim());
        cfg.configure_arg(lib_arg);
    }

    #[cfg(feature = "no_avx512")]
    disable_avx512(&mut cfg);

    let common_build_config = cfg
        .define("CMAKE_BUILD_TYPE", CMAKE_BUILD_TYPE)
        .define("CMAKE_C_FLAGS", "-w")
        .define(
            "CMAKE_CXX_FLAGS",
            "-w -std=c++17 -D_GLIBCXX_USE_CXX11_ABI=1",
        )
        .define("PORTABLE", "1")
        // multi-thread build used to fail compilation. Please try comment out next 2 lines if you have build errors
        .build_arg("-j")
        .build_arg(available_parallelism().unwrap().get().to_string())
        .configure_arg("-Wno-dev")
        .very_verbose(false);

    // let dst = common_build_config
    //     .build_target("tonlib")
    //     .always_configure(true)
    //     .build();

    let dst = common_build_config
        .define("USE_EMSCRIPTEN", "ON")
        .define("TON_ONLY_TONLIB", "ON")
        .define("CMAKE_SHARED_LINKER_FLAGS", "-static")
        .build_target("tonlib")
        .always_configure(true)
        .build();

    let dst = common_build_config
        .define("USE_EMSCRIPTEN", "ON")
        .define("TON_ONLY_TONLIB", "ON")
        // .define("BUILD_SHARED_LIBS", "ON")
        .define("CMAKE_SHARED_LINKER_FLAGS", "-static")
        .build_target("tdutils")
        .always_configure(true)
        .build();

    let dst = common_build_config
        .define("USE_EMSCRIPTEN", "ON")
        .define("TON_ONLY_TONLIB", "ON")
        // .define("BUILD_SHARED_LIBS", "ON")
        .define("CMAKE_SHARED_LINKER_FLAGS", "-static")
        .build_target("tonlibjson")
        .always_configure(true)
        .build();

    let dst = common_build_config
        .define("USE_EMSCRIPTEN", "ON")
        .define("TON_ONLY_TONLIB", "ON")
        .build_target("emulator")
        .always_configure(true)
        .build();

    // link native stdlib
    println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");
    println!("cargo:rustc-link-search=native=/usr/include");
    println!("cargo:rustc-link-search=native=/lib/x86_64-linux-gnu");

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=dylib=c++");
        println!("cargo:rustc-link-arg=-lc++");
    } else if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rustc-link-arg=-lstdc++");
    }

    // println!(
    //     "cargo:rustc-link-search=native={}/build/tdutils",
    //     dst.display()
    // );
    // println!("cargo:rustc-link-lib=static=tdutils");

    // println!(
    //     "cargo:rustc-link-search=native={}/build/crypto",
    //     dst.display()
    // );
    // println!("cargo:rustc-link-lib=static=ton_crypto");

    // println!("cargo:rustc-link-lib=static=ton_crypto_core");
    // println!("cargo:rustc-link-lib=static=ton_block");
    // println!("cargo:rustc-link-lib=static=src_parser");
    // println!("cargo:rustc-link-lib=static=smc-envelope");

    println!(
        "cargo:rustc-link-search=native={}/build/tonlib",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=tonlib");
    println!("cargo:rustc-link-lib=static=tonlibjson");

    println!(
        "cargo:rustc-link-search=native={}/build/emulator",
        dst.display()
    );
    println!("cargo:rustc-link-lib=emulator");
    println!("cargo:rustc-link-lib=emulator_static");
}
