use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut objects = Vec::new();
    
    // Компиляция boot.asm
    let boot_asm = PathBuf::from("src/boot.asm");
    let boot_obj = out_dir.join("boot.o");
    
    if let Ok(status) = std::process::Command::new("nasm")
        .args(&["-f", "elf64", "-o"])
        .arg(&boot_obj)
        .arg(&boot_asm)
        .status()
    {
        if status.success() {
            objects.push(boot_obj);
        }
    }
    
    // Компиляция interrupt_handlers.asm
    let handlers_asm = PathBuf::from("src/interrupt_handlers.asm");
    let handlers_obj = out_dir.join("interrupt_handlers.o");
    
    if let Ok(status) = std::process::Command::new("nasm")
        .args(&["-f", "elf64", "-o"])
        .arg(&handlers_obj)
        .arg(&handlers_asm)
        .status()
    {
        if status.success() {
            objects.push(handlers_obj);
        }
    }
    
    if !objects.is_empty() {
        println!("cargo:rustc-link-search=native={}", out_dir.display());
        for obj in &objects {
            println!("cargo:rustc-link-arg={}", obj.display());
        }
    } else {
        eprintln!("Предупреждение: Не удалось скомпилировать ассемблерные файлы. Убедитесь, что установлен NASM.");
    }
}

