fn main() {
    if uses_error_flag() {
        println!("cargo:rustc-cfg=scall_error=\"flag\"");
    } else {
        println!("cargo:rustc-cfg=scall_error=\"packed\"");
    }

    // All we do is set --cfg flags; we don't need to rerun based on changes in `src/`.
    println!("cargo:rerun-if-changed=build.rs");
}

fn uses_error_flag() -> bool {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    match target_os.as_str() {
        "freebsd" | "macos" => true,

        "linux" => matches!(
            target_arch.as_str(),
            "mips" | "mips64" | "powerpc" | "powerpc64"
        ),

        _ => panic!("Unsupported OS"),
    }
}
