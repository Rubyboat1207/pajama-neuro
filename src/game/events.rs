use minhook::MinHook;
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;

use crate::{game::engine::ScummEngine, neuro_sdk::{DIALOGUE_TX, DialogLine}};

const ACTOR_TALK_OFFSET: usize = 0xC0C70;
const CURSOR_COMMAND_OFFSET: usize = 0x004882d0 - 0x400000;
const START_SCENE_OFFSET: usize = 0x0052d040 - 0x400000;

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

type CursorCommandFn = unsafe extern "thiscall" fn(usize);

static mut ORIGINAL_CURSOR_COMMAND: Option<CursorCommandFn> = None;

unsafe extern "thiscall" fn on_cursor_command(this_ptr: usize) {
    println!("Cursor command intercepted!");

    let subOp = unsafe { ScummEngine::get().unwrap().get_script_byte() };

    match subOp {
        0x86 => println!("Cursor command: Cursor On"),
        0x87 => println!("Cursor command: Cursor Off"),
        0x8B => println!("UserPut on"),
        0x8C => println!("UserPut off"),
        _ => (),
    };

    unsafe {
        if let Some(original) = ORIGINAL_CURSOR_COMMAND {
            original(this_ptr);
        }
    }
}

// void ScummEngine::startScene(int room, Actor *a, int objectNr)
type StartSceneFn = unsafe extern "thiscall" fn(usize, isize, usize, isize);

static mut ORIGINAL_START_SCENE: Option<StartSceneFn> = None;
unsafe extern "thiscall" fn on_start_scene(this_ptr: usize, room: isize, a: usize, objectNr: isize) {
    println!("StartScene called with room {}, actor {}, object {}", room, a, objectNr);

    unsafe {
        if let Some(original) = ORIGINAL_START_SCENE {
            original(this_ptr, room, a, objectNr);
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

        let cursor_func_addr = module_base + CURSOR_COMMAND_OFFSET;
        let cursor_trampoline_ptr = MinHook::create_hook(cursor_func_addr as _, on_cursor_command as _)
            .expect("Failed to create hook");

        ORIGINAL_CURSOR_COMMAND = Some(std::mem::transmute(cursor_trampoline_ptr));

        let start_scene_addr = module_base + START_SCENE_OFFSET;
        let start_scene_trampoline_ptr = MinHook::create_hook(start_scene_addr as _, on_start_scene as _)
            .expect("Failed to create hook");
        ORIGINAL_START_SCENE = Some(std::mem::transmute(start_scene_trampoline_ptr));

        MinHook::enable_all_hooks().expect("Failed to enable hooks");

        println!("Successfully hooked actorTalk at {:#X}", func_addr);
        println!("Successfully hooked cursorCommand at {:#X}", cursor_func_addr);
        println!("Successfully hooked startScene at {:#X}", module_base + START_SCENE_OFFSET);
    }
}
