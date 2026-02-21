use std::ptr;

use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;

pub unsafe fn get_current_room_id() -> Option<i32> {
    // 1. Get the base address of the main executable (scummvm.exe)
    let module_handle = unsafe { GetModuleHandleW(ptr::null()) };
    if module_handle.is_null() {
        return None;
    }
    let base_address = module_handle as usize;

    // 2. Navigate to the static engine pointer
    let static_offset = 0x2756030;
    let engine_ptr_loc = (base_address + static_offset) as *const usize;
    
    // 3. Dereference it to get the dynamic engine base (EBP from your debugger)
    // We use read_volatile so the compiler doesn't optimize this read away
    let engine_base = unsafe { ptr::read_volatile(engine_ptr_loc) };

    // 4. If the pointer is 0, the game hasn't allocated the engine yet (e.g., main menu)
    if engine_base == 0 {
        return None;
    }

    // 5. Add the struct offset and read the actual room ID
    let room_id_offset = 0x48B4;
    let room_id_ptr = (engine_base + room_id_offset) as *const i32;
    
    Some(unsafe { ptr::read_volatile(room_id_ptr) })
}