
#![crate_name = "voxlap"]
#![crate_type = "lib"]

#![desc = ""]
#![license = "MIT"]

extern crate libc;

use std::c_str::CString;
use std::c_vec::CVec;

pub mod ll {
    use libc::{c_long, c_int, c_char};
    #[link(name="voxlap")]
    extern "C" {
        pub fn initvoxlap() -> c_int;
        pub fn uninitvoxlap();

        pub fn voxsetframebuffer(ptr_to_dst_buffer: c_long, pitch: c_long, buffer_width: c_int, buffer_height: c_int);
        pub fn print6x8(x: c_long, y: c_long, fg_color: c_long, bg_color: c_long, fmt: *const c_char, ...);
    }
}

pub enum Color {
    RGB(u8, u8, u8),
}

impl Color {
    pub fn to_u32(&self) -> i32 {
        match self {
            &RGB(r, g, b) => {
                (r as i32 << 16) | (g as i32 << 8) | (b as i32)
            }
        }
    }

    pub fn from_u32(pixel: i32) -> Color {
        let r: u8 = 0;
        let g: u8 = 0;
        let b: u8 = 0;

        unsafe {
            RGB( ((pixel >> 16) & 0xFF) as u8, ((pixel >> 8) & 0xFF) as u8, ((pixel) & 0xFF) as u8)
        }
    }
}

pub fn init() -> Result<(), int> {
    unsafe {
        let result = ll::initvoxlap();

        if result == 0 {
            Ok(())
        } else {
            Err(result as int)
        }
    }
}

pub fn uninit() {
    unsafe {
        ll::uninitvoxlap();
    }
}

pub fn print6x8(x: i32, y: i32, fg_color: Color, bg_color: Color, text: &str) {
    let c_str = text.to_c_str();
    let ptr = c_str.as_ptr();
    unsafe {
        println!("fg: 0x{:X}", fg_color.to_u32());
        ll::print6x8(x, y, fg_color.to_u32(), bg_color.to_u32(), ptr);
    }   
}

pub fn set_frame_buffer(dst_c_vec: CVec<u8>, pitch: i32, buffer_width: i32, buffer_height: i32) {
    unsafe {
        let ptr = dst_c_vec.unwrap() as i32;
        println!("ptr: 0x{:X}", ptr);
        ll::voxsetframebuffer(ptr, pitch, buffer_width, buffer_height);
    }
}