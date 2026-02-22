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

    pub unsafe fn get_room_object(&self, index: usize) -> Option<ObjectData> {
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

    pub unsafe fn get_current_room_id(&self) -> i32 {
        let room_id_ptr = (self.base_address + ROOM_INDEX_OFFSET) as *const i32;
        unsafe { ptr::read_unaligned(room_id_ptr) }
    }

    pub unsafe fn get_num_actors(&self) -> u8 {
        let num_actors_ptr = (self.base_address + NUM_ACTORS_OFFSET) as *const u8;
        unsafe { ptr::read_unaligned(num_actors_ptr) }
    }

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

use std::ffi::CStr;

type GetActorNameFn = unsafe extern "thiscall" fn(this: usize) -> *const i8;

pub struct Actor {
    address: usize,
}

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
