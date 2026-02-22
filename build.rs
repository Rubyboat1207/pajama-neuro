use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // First, generate an import library from SDL2.dll (the original, which will be renamed to SDL2_orig.dll)
    // This satisfies the linker that the symbols exist
    
    let out_dir = env::var("OUT_DIR").unwrap();
    let lib_output = Path::new(&out_dir).join("SDL2_orig.lib");
    
    // Check if SDL2.dll exists in the project root
    if Path::new("SDL2.dll").exists() {
        // Use lib.exe to generate an import library from the DLL
        // lib.exe should be in the same directory as link.exe (MSVC toolchain)
        let lib_exe = if env::var("GITHUB_ACTIONS").is_ok() {
            "lib.exe"
        } else {
            r"C:\Program Files\Microsoft Visual Studio\18\Community\VC\Tools\MSVC\14.50.35717\bin\HostX64\x86\lib.exe"
        };
        
        let status = Command::new(lib_exe)
            .args(&[
                &format!("/DEF:exports.def"),
                &format!("/OUT:{}", lib_output.display()),
                "/MACHINE:X86",
            ])
            .status()
            .expect("Failed to run lib.exe");
        
        if !status.success() {
            panic!("lib.exe failed to create import library");
        }
        
        // Tell cargo to link against this library
        println!("cargo:rustc-link-search=native={}", out_dir);
        println!("cargo:rustc-link-lib=static=SDL2_orig");
    }
    
    // Read the exports.def file to set up export forwarding
    let exports_content = fs::read_to_string("exports.def")
        .expect("Could not read exports.def");
    
    // Parse each line and add /EXPORT forwarders  
    for line in exports_content.lines() {
        let line = line.trim();
        if line.starts_with("LIBRARY") || line.starts_with("EXPORTS") || line.is_empty() {
            continue;
        }
        
        if let Some(func_name) = line.split('=').next() {
            let func_name = func_name.trim();
            // Forward the export to SDL2_orig.dll
            println!("cargo:rustc-link-arg=/EXPORT:{}=SDL2_orig.{}", func_name, func_name);
        }
    }
    
    println!("cargo:rerun-if-changed=SDL2.dll");
    println!("cargo:rerun-if-changed=exports.def");
}
