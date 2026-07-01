use std::process::Command;

const ASM_NAME: &str = "thread";

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let asm_src = format!("src/{ASM_NAME}.s");

    let status = Command::new("riscv64-unknown-elf-gcc")
        .args(["-c", "-march=rv32imafc", "-mabi=ilp32f", &asm_src, "-o"])
        .arg(format!("{out_dir}/{ASM_NAME}.o"))
        .status()
        .expect("failed to run riscv64-unknown-elf-gcc");
    assert!(status.success(), "failed to assemble {asm_src}");

    let status = Command::new("ar")
        .args(["rcs"])
        .arg(format!("{out_dir}/lib{ASM_NAME}.a"))
        .arg(format!("{out_dir}/{ASM_NAME}.o"))
        .status()
        .expect("failed to run ar");
    assert!(status.success(), "failed to create archive");

    println!("cargo:rustc-link-search=native={out_dir}");
    println!("cargo:rustc-link-lib=static={ASM_NAME}");
    println!("cargo:rerun-if-changed={asm_src}");
}
