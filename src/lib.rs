
#![crate_name = "voxlap"]
#![crate_type = "lib"]

#![desc = ""]
#![license = "MIT"]

extern crate libc;

use std::c_str::CString;
use std::c_vec::CVec;

#[deriving(PartialEq, Clone, Show)]
#[repr(C)]
pub struct dpoint3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl dpoint3d {
    pub fn new(x: f64, y: f64, z: f64) -> dpoint3d {
        dpoint3d {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn to_point3d(&self) -> point3d {
        point3d::new(self.x as f32, self.y as f32, self.z as f32)
    }

    
    pub fn to_lpoint3d(&self) -> lpoint3d {
        lpoint3d::new(self.x as i32, self.y as i32, self.z as i32)
    }
}

#[deriving(PartialEq, Clone, Show)]
#[repr(C)]
pub struct point3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl point3d {
    pub fn new(x: f32, y: f32, z: f32) -> point3d {
        point3d {
            x: x,
            y: y,
            z: z,
        }
    }
}

#[deriving(PartialEq, Clone, Show)]
#[repr(C)]
pub struct lpoint3d {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl lpoint3d {
    pub fn new(x: i32, y: i32, z: i32) -> lpoint3d {
        lpoint3d {
            x: x,
            y: y,
            z: z,
        }
    }
}

#[deriving(PartialEq, Clone, Show)]
pub struct Orientation {
    pub pos: dpoint3d,
    pub right_vec: dpoint3d,
    pub down_vec: dpoint3d,
    pub forward_vec: dpoint3d
}

impl Orientation {

}

pub mod ll {
    use libc::{c_long, c_int, c_char, c_float, c_double};
    #[link(name="voxlap")]
    extern "C" {

        pub static mut maxscandist: c_long;

        pub fn initvoxlap() -> c_int;
        pub fn uninitvoxlap();

        pub fn voxsetframebuffer(ptr_to_dst_buffer: c_long, pitch: c_long, buffer_width: c_int, buffer_height: c_int);
        pub fn print6x8(x: c_long, y: c_long, fg_color: c_long, bg_color: c_long, fmt: *const c_char, ...);

        pub fn loadnul(ipo: *mut ::dpoint3d, ist: *mut ::dpoint3d, ihe: *mut ::dpoint3d, ifo: *mut ::dpoint3d);
        pub fn setcamera(ipo: *const ::dpoint3d, ist: *const ::dpoint3d, ihe: *const ::dpoint3d, ifo: *const ::dpoint3d, dahx: c_float, dahy: c_float, dahz: c_float);

        pub fn updatevxl();
        pub fn opticast();

        pub fn clipmove(p: *const ::dpoint3d, v: *const ::dpoint3d, acr: c_double);

        pub fn axisrotate(p: *mut ::point3d, axis: *const ::point3d, w: c_float);

        pub fn setcube(px: c_float, px: c_float, px: c_float, col: c_long);
        pub fn setsphere(center: &::lpoint3d, hitrad: c_long, dacol: c_long);
        pub fn setnormflash(px: c_float, px: c_float, px: c_float, flash_radius: c_long, intens: c_long);


        // custom
        pub fn set_max_scan_dist_to_max(dist: c_long);
        pub fn setLightingMode(mode: c_long);
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
            ll::setLightingMode(1);
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
        ll::print6x8(x, y, fg_color.to_u32(), bg_color.to_u32(), ptr);
    }   
}

pub fn set_frame_buffer(dst_c_vec: CVec<u8>, pitch: i32, buffer_width: i32, buffer_height: i32) {
    unsafe {
        let ptr = dst_c_vec.unwrap() as i32;
        ll::voxsetframebuffer(ptr, pitch, buffer_width, buffer_height);
    }
}

pub fn load_default_map() -> Orientation {
    let mut ipo: dpoint3d = dpoint3d::new(0.0, 0.0, 0.0);
    let mut ist: dpoint3d = dpoint3d::new(0.0, 0.0, 0.0);
    let mut ihe: dpoint3d = dpoint3d::new(0.0, 0.0, 0.0);
    let mut ifo: dpoint3d = dpoint3d::new(0.0, 0.0, 0.0);
    unsafe {
        ll::loadnul(&mut ipo, &mut ist, &mut ihe, &mut ifo);        
    }
    Orientation {
        pos: ipo,
        right_vec: ist,
        down_vec: ihe,
        forward_vec: ifo
    }
}

pub fn update_vxl() {
    unsafe {
        ll::updatevxl();
    }
}

pub fn set_camera(ori: &Orientation, center_x: f32, center_y: f32, focal_length: f32) {
    unsafe {
        ll::setcamera(&ori.pos, &ori.right_vec, &ori.down_vec, &ori.forward_vec, center_x, center_y, focal_length);
    }
}

pub fn opticast() {
    unsafe {
        ll::opticast();
    }
}

pub fn clip_move(pos: &dpoint3d, move_vec: &dpoint3d, acr: f64) {
    unsafe {
        ll::clipmove(pos, move_vec, acr);
    }   
}

pub fn axis_rotate(pos: &mut point3d, axis: &point3d, w: f32) {
    unsafe {
        ll::axisrotate(pos, axis, w);
    }
}

pub fn axis_rotate_d(dpos: &mut dpoint3d, axis: &dpoint3d, w: f32) {
    let mut fpos = point3d::new(dpos.x as f32, dpos.y as f32, dpos.z as f32);
    let mut faxis = point3d::new(axis.x as f32, axis.y as f32, axis.z as f32);
    unsafe {
        ll::axisrotate(&mut fpos, &faxis, w);
    }
    dpos.x = fpos.x as f64;
    dpos.y = fpos.y as f64;
    dpos.z = fpos.z as f64;
}

pub fn z_rotate(pos: &mut point3d, w: f32) {
    let axis = point3d::new(0.0, 0.0, 1.0);
    unsafe {
        ll::axisrotate(pos, &axis, w);
    }
}

pub fn z_rotate_d(dpos: &mut dpoint3d, w: f32) {
    let axis = point3d::new(0.0, 0.0, 1.0);
    let mut fpos = point3d::new(dpos.x as f32, dpos.y as f32, dpos.z as f32);
    unsafe {
        ll::axisrotate(&mut fpos, &axis, w);
    }
    dpos.x = fpos.x as f64;
    dpos.y = fpos.y as f64;
    dpos.z = fpos.z as f64;
}

pub fn set_max_scan_dist_to_max() {
    unsafe {
        let maxscandist = (2048f64 * 1.41421356237f64) as i32;
        ll::set_max_scan_dist_to_max(maxscandist);
    }
}

pub fn set_norm_flash(pos: &point3d, flash_radius: i32, intens: i32) {
    unsafe {
        ll::setnormflash(pos.x, pos.y, pos.z, flash_radius, intens);
    }
}

pub fn set_cube(pos: &point3d, col: Color) {
    unsafe {
        ll::setcube(pos.x, pos.y, pos.z, 0x80FFFFFF);//col.to_u32()
    }
}

pub fn set_sphere(pos: &lpoint3d, radius: i32, dacol: i32) {
    unsafe {
        ll::setsphere(pos, radius, dacol);
    }
}
