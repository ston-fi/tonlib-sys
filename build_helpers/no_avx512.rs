use cmake::Config;

// for clang we just ignore unknown instructions
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

fn disable_avx512_for_rustc() {
    println!("cargo:rustc-env=RUSTFLAGS=-C target-feature=-avx512f,-avx512dq,-avx512cd,-avx512bw,-avx512vl,-avx512ifma,-avx512vbmi,-vpclmulqdq");
}

pub fn disable_avx512(dst: &mut Config) -> &mut Config {
    disable_avx512_for_rustc();
    disable_avx512_for_gcc(dst)
}
