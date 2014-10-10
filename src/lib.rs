
#![crate_name = "voxlap"]
#![crate_type = "lib"]

#![desc = ""]
#![license = "MIT"]

extern crate libc;

use std::c_str::CString;
use std::c_vec::CVec;
use libc::{c_long, c_int, c_char, c_float, c_double, c_void, c_short, c_ushort};
use std::ptr;

mod c_api;


#[deriving(PartialEq, Clone, Show)]
pub struct vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> vec3 {
        vec3 {
            x: x,
            y: y,
            z: z,
        }
    }

    fn as_point3d(&self) -> c_api::point3d {
        c_api::point3d {x: self.x as f32, y: self.y as f32, z: self.z as f32}
    }

    
    fn as_lpoint3d(&self) -> c_api::lpoint3d {
        c_api::lpoint3d {x: self.x as i32, y: self.y as i32, z: self.z as i32}
    }

    fn as_dpoint3d(&self) -> c_api::dpoint3d {
        c_api::dpoint3d {x: self.x as f64, y: self.y as f64, z: self.z as f64}
    }

    fn from_dpoint3d(pos: c_api::dpoint3d) -> vec3 {
        vec3::new(pos.x as f32, pos.y as f32, pos.z as f32)
    }

    fn fill_from_point3d(&mut self, pos: c_api::point3d)  {
        self.x = pos.x as f32;
        self.y = pos.y as f32;
        self.z = pos.z as f32;
    }

    fn fill_from_dpoint3d(&mut self, pos: c_api::dpoint3d)  {
        self.x = pos.x as f32;
        self.y = pos.y as f32;
        self.z = pos.z as f32;
    }
}

pub struct VxSprite {
    ptr: c_api::vx5sprite,
}

impl VxSprite {

    pub fn new(filename: &str) -> VxSprite {
        let mut spr = c_api::vx5sprite::new();
        let c_str = filename.to_c_str();
        let filename_ptr = c_str.as_ptr();
        println!("flags before: {}", spr.flags);
        unsafe {
            c_api::getspr(&mut spr, filename_ptr);
        }
        println!("flags after: {}", spr.flags);
        VxSprite{ptr: spr}
    }

    pub fn set_pos(&mut self, pos: &vec3) {
        unsafe {
            self.ptr.pos = pos.as_point3d();
        }
    }

}

// The `Add<T, U>` trait needs two generic parameters:
// * T is the type of the RHS summand, and
// * U is the type of the sum
// This block implements the operation: Foo + Bar = FooBar
impl Add<vec3, vec3> for vec3 {
    fn add(&self, rhs: &vec3) -> vec3 {
        vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

#[deriving(PartialEq, Clone, Show)]
pub struct Orientation {
    pub pos: vec3,
    pub right_vec: vec3,
    pub down_vec: vec3,
    pub forward_vec: vec3
}

impl Orientation {

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
        let result = c_api::initvoxlap();

        if result == 0 {
            c_api::setLightingMode(1);
            Ok(())
        } else {
            Err(result as int)
        }
    }
}

pub fn uninit() {
    unsafe {
        c_api::uninitvoxlap();
    }
}

pub fn print6x8(x: i32, y: i32, fg_color: Color, bg_color: Color, text: &str) {
    let c_str = text.to_c_str();
    let ptr = c_str.as_ptr();
    unsafe {
        c_api::print6x8(x, y, fg_color.to_u32(), bg_color.to_u32(), ptr);
    }   
}

pub fn set_frame_buffer(dst_c_vec: CVec<u8>, pitch: i32, buffer_width: i32, buffer_height: i32) {
    unsafe {
        let ptr = dst_c_vec.unwrap() as i32;
        c_api::voxsetframebuffer(ptr, pitch, buffer_width, buffer_height);
    }
}

pub fn load_default_map() -> Orientation {
    unsafe {
        let mut ipo = c_api::dpoint3d { x: 0.0, y: 0.0, z: 0.0};
        let mut ist = c_api::dpoint3d { x: 0.0, y: 0.0, z: 0.0};
        let mut ihe = c_api::dpoint3d { x: 0.0, y: 0.0, z: 0.0};
        let mut ifo = c_api::dpoint3d { x: 0.0, y: 0.0, z: 0.0};
        c_api::loadnul(&mut ipo, &mut ist, &mut ihe, &mut ifo);        
        Orientation {
            pos: vec3::from_dpoint3d(ipo),
            right_vec: vec3::from_dpoint3d(ist),
            down_vec: vec3::from_dpoint3d(ihe),
            forward_vec: vec3::from_dpoint3d(ifo)
        }
    }
}

/*pub fn load_kv6(filename: &str) -> Result<VxSprite, i32> {
    unsafe {
        let c_str = filename.to_c_str();
        let filename_ptr = c_str.as_ptr();
        let ptr = c_api::getkv6(filename_ptr);
        println!("ptr: {}", ptr);
        if ptr.is_null() {
            Err(0)
        } else {
            Ok(VxSprite{ptr: ptr})
        }
    }
}*/

pub fn load_vxl(filename: &str) -> Result<Orientation, i32> {
    unsafe {
        let mut ipo = c_api::dpoint3d { x: 0.0, y: 0.0, z: 0.0};
        let mut ist = c_api::dpoint3d { x: 0.0, y: 0.0, z: 0.0};
        let mut ihe = c_api::dpoint3d { x: 0.0, y: 0.0, z: 0.0};
        let mut ifo = c_api::dpoint3d { x: 0.0, y: 0.0, z: 0.0};
        let c_str = filename.to_c_str();
        let filename_ptr = c_str.as_ptr();
        let result = c_api::loadvxl(filename_ptr, &mut ipo, &mut ist, &mut ihe, &mut ifo);        
        match result {
            1 => Ok(Orientation {
                    pos: vec3::from_dpoint3d(ipo),
                    right_vec: vec3::from_dpoint3d(ist),
                    down_vec: vec3::from_dpoint3d(ihe),
                    forward_vec: vec3::from_dpoint3d(ifo)
                }),
            _ => Err(0),

        }
    }
}

pub fn update_vxl() {
    unsafe {
        c_api::updatevxl();
    }
}

pub fn set_camera(ori: &Orientation, center_x: f32, center_y: f32, focal_length: f32) {
    unsafe {
        c_api::setcamera(&ori.pos.as_dpoint3d(), 
            &ori.right_vec.as_dpoint3d(), 
            &ori.down_vec.as_dpoint3d(), 
            &ori.forward_vec.as_dpoint3d(), 
            center_x, center_y, focal_length);
    }
}

pub fn opticast() {
    unsafe {
        c_api::opticast();
    }
}

pub fn clip_move(pos: &mut vec3, move_vec: &vec3, acr: f64) {
    unsafe {
        let mut dpoint3d = pos.as_dpoint3d();
        c_api::clipmove(&mut dpoint3d, &move_vec.as_dpoint3d(), acr);
        pos.fill_from_dpoint3d(dpoint3d);
    }   
}

pub fn axis_rotate(pos: &mut vec3, axis: &vec3, w: f32) {
    unsafe {
        let mut point3d = pos.as_point3d();
        c_api::axisrotate(&mut point3d, &axis.as_point3d(), w);
        pos.fill_from_point3d(point3d);
    }
}

pub fn z_rotate(pos: &mut vec3, w: f32) {
    unsafe {
        let axis = c_api::point3d{ x: 0.0, y: 0.0, z: 1.0 };
        let mut point3d = pos.as_point3d();
        c_api::axisrotate(&mut point3d, &axis, w);
        pos.fill_from_point3d(point3d);
    }
}


pub fn set_max_scan_dist_to_max() {
    unsafe {
        let maxscandist = (2048f64 * 1.41421356237f64) as i32;
        c_api::set_max_scan_dist_to_max(maxscandist);
    }
}

pub fn set_norm_flash(pos: &vec3, flash_radius: i32, intens: i32) {
    unsafe {
        c_api::setnormflash(pos.x, pos.y, pos.z, flash_radius, intens);
    }
}

pub fn set_cube(pos: &vec3, col: Color) {
    unsafe {
        c_api::setcube(pos.x as i32, pos.y as i32, pos.z as i32, col.to_u32());
    }
}

pub fn set_sphere(pos: &vec3, radius: i32, dacol: i32) {
    unsafe {
        c_api::setsphere(&pos.as_lpoint3d(), radius, dacol);
    }
}

pub fn update_lighting(x0: i32, y0: i32, z0: i32, x1: i32, y1: i32, z1: i32) {
    unsafe {
        c_api::updatelighting(x0, y0, z0, x1, y1, z1);
    }
}

pub fn draw_point_3d(pos: &vec3, col: Color) {
    unsafe {
        c_api::drawpoint3d(pos.x, pos.y, pos.z, col.to_u32());
    }
}

pub fn draw_sprite(spr: &VxSprite) {
    unsafe {
        c_api::drawsprite(&spr.ptr);
    }
}