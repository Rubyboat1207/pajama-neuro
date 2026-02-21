#![allow(non_snake_case)]
pub(crate) mod game;

use std::ptr::null_mut;
use std::time::Duration;
use windows_sys::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use windows_sys::Win32::System::Threading::{CreateThread};
use windows_sys::Win32::System::Console::AllocConsole;

use crate::game::init_game;

#[unsafe(no_mangle)]
pub extern "system" fn DllMain(_: *const (), reason: u32, _: *const ()) -> i32 {
    if reason == DLL_PROCESS_ATTACH {
        unsafe {
            // CreateThread is now visible
            CreateThread(
                null_mut(),
                0,
                Some(main_thread),
                null_mut(),
                0,
                null_mut(),
            );
        }
    }
    1
}

unsafe extern "system" fn main_thread(_: *mut std::ffi::c_void) -> u32 {
    
    unsafe {
        AllocConsole();
    }

    std::fs::OpenOptions::new().write(true).open("CONOUT$").expect("Could not set stdout.");
    
    println!("Pajama Sam from rust!");
    // init_game();

    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}