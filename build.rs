fn main() {
    build();
}

#[cfg(feature = "cmake-build")]
fn build() {
    use std::env;

    if !std::path::Path::new("ton/tonlib").is_dir() {
        let status = std::process::Command::new("git")
            .args(["submodule", "update", "--init", "--recursive"])
            .status()
            .unwrap();
        if !status.success() {
            panic!("Git submodule init fail");
        }
    }

    println!("cargo:rerun-if-changed=ton/CMakeLists.txt");
    println!("cargo:rerun-if-changed=build.rs");

    if !cfg!(target_os = "linux") {
        env::set_var("NUM_JOBS", num_cpus::get().to_string());
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
    println!("cargo:rustc-link-lib=dylib=stdc++");
}

#[cfg(not(feature = "cmake-build"))]
fn build() {
    println!("cargo:rustc-link-lib=tonlibjson.0.5");
}
