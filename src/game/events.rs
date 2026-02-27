use minhook::MinHook;
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;

use crate::{neuro_sdk::{DIALOGUE_TX, DialogLine}};

const ACTOR_TALK_OFFSET: usize = 0xC0C70;

type ActorTalkFn = unsafe extern "thiscall" fn(usize, *const u8);

static mut ORIGINAL_ACTOR_TALK: Option<ActorTalkFn> = None;

unsafe extern "thiscall" fn on_actor_talk(this_ptr: usize, msg: *const u8) {
    if !msg.is_null() {
        let raw_text = unsafe { std::ffi::CStr::from_ptr(msg as *const i8).to_string_lossy() };
        
        // "\x7FT[offset],[length]\x7F[Dialogue]"
        if raw_text.starts_with("\x7FT") {
            let parts: Vec<&str> = raw_text.split("\x7F").collect();
            if parts.len() >= 3 {
                let numbers: Vec<&str> = parts[1][1..].split(',').collect();
                
                if numbers.len() == 2 {
                    if let (Ok(offset), Ok(length)) = (numbers[0].parse::<u32>(), numbers[1].parse::<u32>()) {
                        let clean_text = parts[2..].join(""); // Rejoin in case text has \x7F (rare)
                        
                        let line = DialogLine {
                            offset_id: offset,
                            length,
                            text: clean_text,
                        };

                        println!("Intercepted Line: {:?}", line);
                        
                        if let Some(tx) = DIALOGUE_TX.get() {
                            let _ = tx.send(line);
                        }
                    }
                }
            }
        }
    }

    unsafe {
        if let Some(original) = ORIGINAL_ACTOR_TALK {
            original(this_ptr, msg);
        }
    }
}

pub unsafe fn init_hooks() {
    unsafe {
        let module_base = GetModuleHandleW(std::ptr::null()) as usize;
        let func_addr = module_base + ACTOR_TALK_OFFSET;

        let trampoline_ptr = MinHook::create_hook(func_addr as _, on_actor_talk as _)
            .expect("Failed to create hook");

        ORIGINAL_ACTOR_TALK = Some(std::mem::transmute(trampoline_ptr));

        MinHook::enable_all_hooks().expect("Failed to enable hooks");

        println!("Successfully hooked actorTalk at {:#X}", func_addr);
    }
}
