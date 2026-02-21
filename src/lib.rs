#![allow(non_snake_case)]
pub(crate) mod game;

use std::ptr::null_mut;
use std::time::Duration;
use windows_sys::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use windows_sys::Win32::System::Threading::{CreateThread};
use windows_sys::Win32::System::Console::AllocConsole;
use windows_sys::Win32::Foundation::{GENERIC_READ, GENERIC_WRITE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::Storage::FileSystem::{CreateFileA, OPEN_EXISTING, FILE_SHARE_READ, FILE_SHARE_WRITE};
use windows_sys::Win32::System::Console::SetStdHandle;
use windows_sys::Win32::System::Console::{STD_OUTPUT_HANDLE, STD_ERROR_HANDLE, STD_INPUT_HANDLE};

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
        // Allocate a console window
        AllocConsole();
        
        // Redirect stdout to the console
        let conout = b"CONOUT$\0".as_ptr();
        let stdout_handle = CreateFileA(
            conout,
            GENERIC_WRITE | GENERIC_READ,
            FILE_SHARE_WRITE | FILE_SHARE_READ,
            null_mut(),
            OPEN_EXISTING,
            0,
            null_mut(),
        );
        
        if stdout_handle != INVALID_HANDLE_VALUE && !stdout_handle.is_null() {
            SetStdHandle(STD_OUTPUT_HANDLE, stdout_handle);
            SetStdHandle(STD_ERROR_HANDLE, stdout_handle);
        }
        
        // Redirect stdin to the console
        let conin = b"CONIN$\0".as_ptr();
        let stdin_handle = CreateFileA(
            conin,
            GENERIC_WRITE | GENERIC_READ,
            FILE_SHARE_WRITE | FILE_SHARE_READ,
            null_mut(),
            OPEN_EXISTING,
            0,
            null_mut(),
        );
        
        if stdin_handle != INVALID_HANDLE_VALUE && !stdin_handle.is_null() {
            SetStdHandle(STD_INPUT_HANDLE, stdin_handle);
        }
    }
    
    println!("Pajama Sam from rust!");
    init_game();

    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}
