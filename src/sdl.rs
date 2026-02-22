use std::os::raw::c_int;

// --- SDL2 Constants ---
pub const SDL_MOUSEBUTTONDOWN: u32 = 0x401;
pub const SDL_MOUSEBUTTONUP: u32 = 0x402;
pub const SDL_BUTTON_LEFT: u8 = 1;
pub const SDL_PRESSED: u8 = 1;
pub const SDL_RELEASED: u8 = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SDL_MouseButtonEvent {
    pub type_: u32,
    pub timestamp: u32,
    pub windowID: u32,
    pub which: u32,
    pub button: u8,
    pub state: u8,
    pub clicks: u8,
    pub padding1: u8,
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
pub union SDL_Event {
    pub type_: u32,
    pub button: SDL_MouseButtonEvent,
    pub padding: [u8; 56],
}

unsafe extern "C" {
    pub fn SDL_PushEvent(event: *mut SDL_Event) -> c_int;
}