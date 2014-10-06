
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
    fn new(x: f64, y: f64, z: f64) -> dpoint3d {
        dpoint3d {
            x: x,
            y: y,
            z: z,
        }
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
    fn new(x: f32, y: f32, z: f32) -> point3d {
        point3d {
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
/*
#[repr(C)]
struct vx5 {
    //------------------------ DATA coming from VOXLAP5 ------------------------

        //Clipmove hit point info (use this after calling clipmove):
    clipmaxcr: c_double; //clipmove always calls findmaxcr even with no movement
    cliphit[3]: dpoint3d;
    cliphitnum: c_long;

        //Bounding box written by last set* VXL writing call
    minx: c_long;
    miny: c_long;
    minz: c_long;
    maxx: c_long;
    maxy: c_long;
    maxz: c_long;

        //Falling voxels shared data:
    flstnum: c_long;
    flstboxtype flstcnt[FLPIECES];

        //Total count of solid voxels in .VXL map (included unexposed voxels)
    globalmass: c_long;

        //Temp workspace for KFA animation (hinge angles)
        //Animsprite writes these values&you may modify them before drawsprite
    kfaval: c_short[MAXFRM];

    //------------------------ DATA provided to VOXLAP5 ------------------------

        //Opticast variables:
    long anginc, sideshademode, mipscandist, maxscandist, vxlmipuse, fogcol;

        //Drawsprite variables:
    long kv6mipfactor, kv6col;
        //Drawsprite x-plane clipping (reset to 0,(high int) after use!)
        //For example min=8,max=12 permits only planes 8,9,10,11 to draw
    long xplanemin, xplanemax;

        //Map modification function data:
    long curcol, currad, curhei;
    float curpow;

        //Procedural texture function data:
    long (*colfunc)(lpoint3d *);
    long cen, amount, *pic, bpl, xsiz, ysiz, xoru, xorv, picmode;
    point3d fpico, fpicu, fpicv, fpicw;
    lpoint3d pico, picu, picv;
    float daf;

        //Lighting variables: (used by updatelighting)
    long lightmode; //0 (default), 1:simple lighting, 2:lightsrc lighting
    lightsrctype lightsrc[MAXLIGHTS]; //(?,?,?),128*128,262144
    long numlights;

    long fallcheck;
} vx5;*/

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
        println!("fg: 0x{:X}", fg_color.to_u32());
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

pub fn z_rotate(pos: &mut point3d, w: f32) {
    let axis = point3d::new(0.0, 0.0, 1.0);
    unsafe {
        ll::axisrotate(pos, &axis, w);
    }
}

pub fn z_rotate_d(dpos: &mut dpoint3d, w: f32) {
    let axis = point3d::new(0.0, 0.0, 1.0);
    let mut fpos = point3d::new(dpos.x as f32, dpos.y as f32, dpos.z as f32, );
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