
#![crate_name = "voxlap"]
#![crate_type = "lib"]

#![desc = ""]
#![license = "MIT"]

extern crate libc;

use std::mem;
use std::c_str::CString;
use std::c_vec::CVec;
use libc::{c_long, c_int, c_char, c_float, c_double, c_void, c_short, c_ushort};
use std::ptr;

mod c_api;


pub enum CsgOperationType {
    Insert,
    Remove
}

impl CsgOperationType {
    fn as_int(self) -> i32 {
        match self {
            Insert => 0,
            Remove => -1,
        }
    }
}

pub enum LightingMode {
    NoSpecialLighting,
    SimpleEstimatedNormalLighting,
    MultiplePointSourceLighting
}

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

    pub fn newi(x: i32, y: i32, z: i32) -> vec3 {
        vec3 {
            x: x as f32,
            y: y as f32,
            z: z as f32,
        }
    }

    fn as_point3d(&self) -> &c_api::point3d {
        unsafe {mem::transmute(self)}
    }

    fn as_mut_point3d(&mut self) -> &mut c_api::point3d {
        unsafe {mem::transmute(self)}
    }

    fn to_dpoint3d(&self) -> c_api::dpoint3d {
        c_api::dpoint3d {
            x: self.x as f64, 
            y: self.y as f64, 
            z: self.z as f64, 
        }
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

#[deriving(PartialEq, Clone, Show)]
pub struct ivec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl ivec3 {
    pub fn new(x: i32, y: i32, z: i32) -> ivec3 {
        ivec3 {
            x: x,
            y: y,
            z: z,
        }
    }
    
    fn as_lpoint3d(&self) -> &c_api::lpoint3d {
        unsafe {mem::transmute(self)}
    }

    fn as_mut_lpoint3d(&mut self) -> &mut c_api::lpoint3d {
        unsafe {mem::transmute(self)}
    }
}

pub struct VxSprite {
    ptr: c_api::vx5sprite,
    managed_by_voxlap: bool
}

impl VxSprite {
    pub fn new(filename: &str) -> VxSprite {
        let mut spr = c_api::vx5sprite::new();
        let c_str = filename.to_c_str();
        let filename_ptr = c_str.as_ptr();
        unsafe {
            c_api::getspr(&mut spr, filename_ptr);
        }

        VxSprite {
            ptr: spr,
            managed_by_voxlap: true,
        }
    }

    pub fn set_pos(&mut self, pos: &vec3) {
        unsafe {
            self.ptr.pos = *pos.as_point3d();
        }
    }
}

        /// This converts a spherical cut-out of the VXL map into a .KV6 sprite in
        ///   memory. This function can be used to make walls fall over (with full
        ///   rotation). It allocates a new vx5sprite sprite structure and you are
        ///   responsible for freeing the memory using "free" in your own code.
        ///   spr: new vx5sprite structure. Position & orientation are initialized
        ///           so when you call drawsprite, it exactly matches the VXL map.
        ///   hit: center of sphere
        /// hitrad: radius of sphere
        /// returns: 0:bad, >0:mass of captured object (# of voxels)
//pub fn meltsphere (spr: &vx5sprite, hit: &lpoint3d, hitrad: c_long) -> c_long {
pub fn melt_sphere(center: &ivec3, radius: i32) -> Result<(VxSprite, i32), ()> {
    let mut spr = c_api::vx5sprite::new();
    match unsafe {
        c_api::meltsphere(&mut spr, center.as_lpoint3d(), radius)
    } {
        0 => Err(()),
        x => Ok(
            (VxSprite {
                ptr: spr,
                managed_by_voxlap: false,
            }, x)
        )
    }
}

impl Drop for VxSprite {
    fn drop(&mut self) {
        if !self.managed_by_voxlap {
            println!("FREE");
            unsafe {
                c_api::freekv6(&*self.ptr.voxnum);
            }
        }
    }
}

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
        if (y >= 600 - 7) {
            fail!("print6x8: y pos: {}", y);
        }
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
    let mut ipo = c_api::dpoint3d { x: 0.0, y: 0.0, z: 0.0};
    let mut ist = c_api::dpoint3d { x: 0.0, y: 0.0, z: 0.0};
    let mut ihe = c_api::dpoint3d { x: 0.0, y: 0.0, z: 0.0};
    let mut ifo = c_api::dpoint3d { x: 0.0, y: 0.0, z: 0.0};
    let c_str = filename.to_c_str();
    let filename_ptr = c_str.as_ptr();
    match unsafe {
        c_api::loadvxl(filename_ptr, &mut ipo, &mut ist, &mut ihe, &mut ifo)
    } {
        1 => Ok(Orientation {
            pos: vec3::from_dpoint3d(ipo),
            right_vec: vec3::from_dpoint3d(ist),
            down_vec: vec3::from_dpoint3d(ihe),
            forward_vec: vec3::from_dpoint3d(ifo)
        }),
        _ => Err(0),
    }
}

pub fn update_vxl() {
    unsafe {
        c_api::updatevxl();
    }
}

pub fn set_camera(ori: &Orientation, center_x: f32, center_y: f32, focal_length: f32) {
    unsafe {
        c_api::setcamera(&ori.pos.to_dpoint3d(), 
            &ori.right_vec.to_dpoint3d(), 
            &ori.down_vec.to_dpoint3d(), 
            &ori.forward_vec.to_dpoint3d(), 
            center_x, center_y, focal_length);
    }
}

pub fn opticast() {
    unsafe {
        c_api::opticast();
    }
}

pub fn clip_move(pos: &mut vec3, move_vec: &vec3, acr: f64) {
    let mut dpos = pos.to_dpoint3d();
    unsafe {
        c_api::clipmove(&mut dpos, &move_vec.to_dpoint3d(), acr);
    } 
    pos.fill_from_dpoint3d(dpos);
}

pub fn axis_rotate(pos: &mut vec3, axis: &vec3, w: f32) {
    unsafe {
        c_api::axisrotate(pos.as_mut_point3d(), axis.as_point3d(), w);
    }
}

pub fn z_rotate(pos: &mut vec3, w: f32) {
    unsafe {
        let axis = c_api::point3d{ x: 0.0, y: 0.0, z: 1.0 };
        c_api::axisrotate(pos.as_mut_point3d(), &axis, w);
    }
}


pub fn set_max_scan_dist_to_max() {
    unsafe {
        //let maxscandist = (2048f64 * 1.41421356237f64) as i32;
        c_api::setMaxScanDistToMax();
    }
}

pub fn set_norm_flash(pos: &vec3, flash_radius: i32, intens: i32) {
    unsafe {
        c_api::setnormflash(pos.x, pos.y, pos.z, flash_radius, intens);
    }
}


pub fn set_sphere(pos: &ivec3, radius: i32, operation_type: CsgOperationType) {
    unsafe {
        c_api::setsphere(pos.as_lpoint3d(), radius, operation_type.as_int());
    }
}

pub fn update_lighting(x0: i32, y0: i32, z0: i32, x1: i32, y1: i32, z1: i32) {
    unsafe {
        c_api::updatelighting(x0, y0, z0, x1, y1, z1);
    }
}

fn in_screen(num: i32) -> bool {
    num >= 0 && num < 600
}

pub fn draw_line_2d(x1: i32, y1: i32, x2: i32, y2: i32, col: Color) {
    assert!(in_screen(x1), "x1 = {}", x1);
    assert!(in_screen(x2), "x2 = {}", x2);
    assert!(in_screen(y1), "y1 = {}", y1);
    assert!(in_screen(y2), "y2 = {}", y2);
    unsafe {
        c_api::drawline2d(x1 as f32, y1 as f32, x2 as f32, y2 as f32, col.to_u32());
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

pub fn set_kv6_into_vxl_memory(spr: &VxSprite, operation_type: CsgOperationType) {
    unsafe {
        c_api::setkv6(&spr.ptr, operation_type.as_int());
    }
}

pub fn set_lighting_mode(mode: LightingMode) {
    let m = match mode {
        NoSpecialLighting => 0,
        SimpleEstimatedNormalLighting => 1,
        MultiplePointSourceLighting => 2,
    };
    unsafe {
        c_api::setLightingMode(m);
    }
    
}

pub fn set_rect(p1: &ivec3, p2: &ivec3, mode: CsgOperationType) {
    unsafe {
        c_api::setrect(p1.as_lpoint3d(), p2.as_lpoint3d(), mode.as_int());
    }
}

pub fn set_cube(pos: &ivec3, col: Option<Color>) {
    unsafe {
        let col = col.map_or(-1, |c| c.to_u32());
        c_api::setcube(pos.x, pos.y, pos.z, col);
    }
}

pub fn load_sky(filename: &str) -> Result<(), ()> {
    match unsafe {
        let c_str = filename.to_c_str();
        let filename_ptr = c_str.as_ptr();
        c_api::loadsky(filename_ptr)
    } {
        0 => Ok(()),
        _ => Err(()),
    }
}

pub fn set_raycast_density(param: i32) {
    assert!(param >= 1, "Param cannot be < 0!");
    unsafe {
        c_api::set_anginc(param);
    }
}

pub fn get_raycast_density() -> i32 {
    unsafe {
        c_api::get_anginc()
    }
}

pub fn set_fog_color(param: Color) {
    unsafe {
        c_api::set_fogcol(param.to_u32());
    }
}

pub fn set_kv6col(param: Color) {
    unsafe {
        c_api::set_kv6col(param.to_u32());
    }
}

pub fn set_curcol(param: Color) {
    unsafe {
        c_api::set_curcol(param.to_u32());
    }
}

pub fn set_curpow(param: c_float) {
    unsafe {
        c_api::set_curpow(param);
    }
}

pub fn set_fallcheck(param: bool) {
    unsafe {
        c_api::set_fallcheck(param as i32);
    }
}

pub fn generate_vxl_mipmapping(x0: i32, y0: i32, x1: i32, y1: i32) {
    unsafe {
        c_api::genmipvxl(x0, y0, x1, y1);
    }
}

pub fn get_max_xy_dimension() -> i32 {
    unsafe { c_api::getVSID() }
}

pub fn draw_sphere_fill(pos: &vec3, radius: f32, col: Color) {
    unsafe {
        c_api::drawspherefill(pos.x, pos.y, pos.z, radius, col.to_u32());
    }
}

pub enum VisibilityResult {
    CanSee,
    CannotSee(ivec3),
}

pub fn can_see (starting_point: &vec3, ending_point: &vec3) -> VisibilityResult {
    let mut hit_pos = ivec3::new(0, 0, 0);
    match unsafe {
        c_api::cansee(starting_point.as_point3d(), ending_point.as_point3d(), hit_pos.as_mut_lpoint3d())
    } {
        1 => CanSee,
        _ => CannotSee(hit_pos)
    }
}