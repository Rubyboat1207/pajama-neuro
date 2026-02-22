use std::ptr;
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;

// --- Engine Offsets ---
const VM_OFFSET: usize = 0x2756030;
const ROOM_INDEX_OFFSET: usize = 0x48B4;
const NUM_ACTORS_OFFSET: usize = 0x27E5;
const ACTORS_ARRAY_OFFSET: usize = 0x27E8;
const OBJS_OFFSET: usize = 0x68;

// --- Actor Offsets ---
const ACTOR_X_OFFSET: usize = 0x08;
const ACTOR_Y_OFFSET: usize = 0x0A;
// const GET_ACTOR_NAME_OFFSET: usize = 0xC11A0;

pub struct ScummEngine {
    base_address: usize,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ObjectData {
    pub obim_offset: u32,
    pub obcd_offset: u32,
    pub walk_x: i16,
    pub walk_y: i16,
    pub obj_nr: u16,
    pub x_pos: i16,
    pub y_pos: i16,
    pub width: u16,
    pub height: u16,
    pub actordir: u8,
    pub parent: u8,
    pub parentstate: u8,
    pub state: u8,
    pub fl_object_index: u8,
    pub flags: u8,
}

impl ScummEngine {
    pub unsafe fn get() -> Option<Self> {
        unsafe {
            let module_handle = GetModuleHandleW(ptr::null());
            if module_handle.is_null() {
                return None;
            }

            let module_base = module_handle as usize;
            let engine_ptr_loc = (module_base + VM_OFFSET) as *const usize;
            let base_address = ptr::read_volatile(engine_ptr_loc);

            if base_address == 0 {
                return None;
            }

            Some(Self { base_address })
        }
    }

    pub unsafe fn get_room_object_at(&self, index: usize) -> Option<ObjectData> {
        unsafe {
            let objs_array_ptr_loc = (self.base_address + OBJS_OFFSET) as *const usize;
            let objs_array_base = ptr::read_unaligned(objs_array_ptr_loc);

            if objs_array_base == 0 {
                return None;
            }

            let obj_ptr = (objs_array_base as *const ObjectData).add(index);
            let obj_data = ptr::read_unaligned(obj_ptr);

            if obj_data.obj_nr == 0 {
                return None;
            }

            Some(obj_data)
        }
    }

    // maybe this should be marked unsafe, but to me I don't think this would ever cause unsafe accesses under normal circumstances.
    // This list can be larger than 256, but pajama never uses that many anyways I think.
    pub fn get_room_object(&self, num: usize) -> Option<ObjectData> {
        for i in 1..256 {
            match unsafe { self.get_room_object_at(i) } {
                Some(obj) if obj.obj_nr as usize == num => return Some(obj),
                _ => continue,
            }
        }

        None
    }

    #[allow(unused)]
    pub unsafe fn print_all_objects(&self) {
        // Note: You can also map _numLocalObjects to stop the loop dynamically,
        // but checking until obj_nr == 0 or a hard cap of ~100 usually works.
        for i in 1..100 {
            if let Some(obj) = self.get_room_object(i) {
                let obj_nr = obj.obj_nr;
                let x_pos = obj.x_pos;
                let y_pos = obj.y_pos;
                let width = obj.width;
                let height = obj.height;

                println!(
                    "Found Object ID {} at X: {}, Y: {} ({}x{})",
                    obj_nr, x_pos, y_pos, width, height
                );
            }
        }
    }

    pub fn get_current_room_id(&self) -> i32 {
        let room_id_ptr = (self.base_address + ROOM_INDEX_OFFSET) as *const i32;
        unsafe { ptr::read_unaligned(room_id_ptr) }
    }

    #[allow(unused)]
    pub unsafe fn get_num_actors(&self) -> u8 {
        let num_actors_ptr = (self.base_address + NUM_ACTORS_OFFSET) as *const u8;
        unsafe { ptr::read_unaligned(num_actors_ptr) }
    }

    #[allow(unused)]
    pub unsafe fn get_actor(&self, index: usize) -> Option<Actor> {
        unsafe {
            let max_actors = self.get_num_actors() as usize;
            if index >= max_actors {
                return None;
            }

            let actors_array_ptr_loc = (self.base_address + ACTORS_ARRAY_OFFSET) as *const usize;
            let actors_array_base = ptr::read_unaligned(actors_array_ptr_loc);

            if actors_array_base == 0 {
                return None;
            }

            let actor_ptr_loc = (actors_array_base as *const usize).add(index);
            let actor_address = ptr::read_unaligned(actor_ptr_loc);

            if actor_address == 0 {
                return None;
            }

            Some(Actor {
                address: actor_address,
            })
        }
    }
}

use crate::sdl::{
    SDL_BUTTON_LEFT, SDL_Event, SDL_MOUSEBUTTONDOWN, SDL_MOUSEBUTTONUP, SDL_MouseButtonEvent,
    SDL_PRESSED, SDL_PushEvent, SDL_RELEASED,
};

// type GetActorNameFn = unsafe extern "thiscall" fn(this: usize) -> *const i8;

pub struct Actor {
    address: usize,
}

#[allow(unused)]
impl Actor {
    pub unsafe fn get_position(&self) -> (u16, u16) {
        let x_ptr = (self.address + ACTOR_X_OFFSET) as *const u16;
        let y_ptr = (self.address + ACTOR_Y_OFFSET) as *const u16;

        unsafe {
            let x = ptr::read_unaligned(x_ptr);
            let y = ptr::read_unaligned(y_ptr);
            (x, y)
        }
    }

    pub unsafe fn set_position(&self, x: u16, y: u16) {
        let x_ptr = (self.address + ACTOR_X_OFFSET) as *mut u16;
        let y_ptr = (self.address + ACTOR_Y_OFFSET) as *mut u16;

        unsafe {
            ptr::write_unaligned(x_ptr, x);
            ptr::write_unaligned(y_ptr, y);
        }
    }

    // commenting this out because it may be useful to someone at some point, but
    // pajama sam does not use names because it does not have hover text.
    // so the devs probably just never added them

    // pub unsafe fn get_name(&self) -> Option<String> {
    //     unsafe {
    //         let module_handle = GetModuleHandleW(std::ptr::null());
    //         if module_handle.is_null() {
    //             return None;
    //         }
    //         let module_base = module_handle as usize;

    //         let func_addr = module_base + GET_ACTOR_NAME_OFFSET;

    //         let get_name_native: GetActorNameFn = std::mem::transmute(func_addr);

    //         let name_ptr = get_name_native(self.address);

    //         // 5. Handle the result
    //         if name_ptr.is_null() {
    //             return None;
    //         }

    //         // 6. Convert the C string to a Rust String
    //         let c_str = CStr::from_ptr(name_ptr);
    //         Some(c_str.to_string_lossy().into_owned())
    //     }
    // }

    pub unsafe fn get_id(&self) -> u8 {
        let id_ptr = (self.address + 0x18) as *const u8;
        unsafe { ptr::read_unaligned(id_ptr) }
    }
}

impl ObjectData {
    /// Simulates an SDL mouse click at the exact center of this object's hitbox.
    pub fn click(&self, offset: Option<(i32, i32)>) {
        // 1. Calculate the center of the hitbox
        let center_x = (self.x_pos + (self.width as i16 / 2)) as i32;
        let center_y = (self.y_pos + (self.height as i16 / 2)) as i32;
        let obj_nr = self.obj_nr;

        let offset = offset.unwrap_or((0, 0));

        println!(
            "Simulating click on Object {} at X:{}, Y:{}",
            obj_nr, center_x + offset.0, center_y + offset.1
        );

        // 2. Teleport the mouse to the object (Mouse Motion)
        // (Optional, but helps update any hover states in the engine before the click)
        // You would construct an SDL_MouseMotionEvent here if needed.

        // 3. Construct the Mouse Down event
        let mut mouse_down = SDL_Event {
            button: SDL_MouseButtonEvent {
                type_: SDL_MOUSEBUTTONDOWN,
                timestamp: 0, // SDL usually handles 0 fine, or use SDL_GetTicks()
                windowID: 0,  // Target the main window
                which: 0,     // Touch device ID (0 is mouse)
                button: SDL_BUTTON_LEFT,
                state: SDL_PRESSED,
                clicks: 1,
                padding1: 0,
                x: center_x + offset.0,
                y: center_y + offset.1,
            },
        };

        // 4. Construct the Mouse Up event
        let mut mouse_up = SDL_Event {
            button: SDL_MouseButtonEvent {
                type_: SDL_MOUSEBUTTONUP,
                timestamp: 0,
                windowID: 0,
                which: 0,
                button: SDL_BUTTON_LEFT,
                state: SDL_RELEASED,
                clicks: 1,
                padding1: 0,
                x: center_x,
                y: center_y,
            },
        };

        // 5. Inject the events into the queue
        unsafe {
            SDL_PushEvent(&mut mouse_down);
            SDL_PushEvent(&mut mouse_up);
        }
    }
}
