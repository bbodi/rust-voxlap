
#![crate_name = "voxlap"]
#![crate_type = "lib"]

#![desc = ""]
#![license = "MIT"]

extern crate libc;

pub use c_api::vspans;


use std::rand::{Rng, Rand};
use std::collections::HashMap;
use std::io::File;
use std::vec::Vec;

use std::mem;
use std::c_str::CString;
use std::c_vec::CVec;
use libc::{free, c_long, c_int, c_char, c_float, c_double, c_void, c_short, c_ushort};
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

#[deriving(PartialEq, Clone, Show)]
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

    pub fn identity() -> vec3 {
        vec3 {
            x: 1f32,
            y: 1f32,
            z: 1f32,
        }
    }

    pub fn null() -> vec3 {
        vec3 {
            x: 0f32,
            y: 0f32,
            z: 0f32,
        }
    }

    fn from_point3d(pos: c_api::point3d) -> vec3 {
        let mut vec = vec3::null();
        vec.x = pos.x as f32;
        vec.y = pos.y as f32;
        vec.z = pos.z as f32;
        return vec;
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

impl Rand for vec3 {
    fn rand<R:Rng>(rng: &mut R) -> vec3 {
        let mut vec = vec3::null();
        vec.z = (rng.gen::<i32>() & 32767) as f32 / 16383.5f32 - 1.0f32;
        let mut f = (((rng.gen::<i32>() & 32767)) as f32 / 16383.5f32 - 1.0f32) * std::num::Float::pi();
        vec.x = f.cos();
        vec.y = f.sin();
        f = (1.0 - vec.z * vec.z).sqrt();
        vec.x *= f;
        vec.y *= f;
        return vec;
    }
}

impl Add<vec3, vec3> for vec3 {
    fn add(&self, rhs: &vec3) -> vec3 {
        vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<vec3, vec3> for vec3 {
    fn sub(&self, rhs: &vec3) -> vec3 {
        vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f32, vec3> for vec3 {
    fn mul(&self, f: &f32) -> vec3 {
        vec3 {
            x: self.x * *f,
            y: self.y * *f,
            z: self.z * *f
        }
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

    pub fn to_vec3(&self) -> vec3 {
        vec3 {
            x: (self.x as f32) + 0.5f32,
            y: (self.y as f32) + 0.5f32,
            z: (self.z as f32) + 0.5f32,
        }
    }
}

impl Add<ivec3, ivec3> for ivec3 {
    fn add(&self, rhs: &ivec3) -> ivec3 {
        ivec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<ivec3, ivec3> for ivec3 {
    fn sub(&self, rhs: &ivec3) -> ivec3 {
        ivec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<i32, ivec3> for ivec3 {
    fn mul(&self, f: &i32) -> ivec3 {
        ivec3 {
            x: self.x * *f,
            y: self.y * *f,
            z: self.z * *f
        }
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

    pub fn get_pos(&self) -> vec3 {
        unsafe {
            vec3::from_point3d(self.ptr.pos)
        }
    }

    pub fn add_pos(&mut self, dir: &vec3) {
        unsafe {
            self.ptr.pos = *(vec3::from_point3d(self.ptr.pos) + *dir).as_point3d();
        }
    }

    pub fn rotate(&mut self, around_angle: &vec3, w: f32) {
        c_axis_rotate(&mut self.ptr.s, around_angle, w);
        c_axis_rotate(&mut self.ptr.h, around_angle, w);
        c_axis_rotate(&mut self.ptr.f, around_angle, w);
    }

    pub fn scale(&mut self, scale: &vec3) {
        self.ptr.s.x *= scale.x;
        self.ptr.h.y *= scale.y;
        self.ptr.f.z *= scale.z;
    }

    pub fn set_scale(&mut self, scale: &vec3) {
        self.ptr.s.x = scale.x;
        self.ptr.h.y = scale.y;
        self.ptr.f.z = scale.z;
    }

    pub fn animate(&mut self, time_add: u32) {
        unsafe {
            c_api::animsprite(&self.ptr, time_add);
        }
    }
}

impl Drop for VxSprite {
    fn drop(&mut self) {
        if !self.managed_by_voxlap && self.ptr.voxnum != ptr::null_mut() {
            unsafe {
                c_api::freekv6(&*self.ptr.voxnum);
            }
        }
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


#[repr(C)]
#[deriving(PartialEq, Clone, Show)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: 255,
        }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }

    pub fn to_i32(&self) -> i32 {
        (0x80 << 24) | (self.r as i32 << 16) | (self.g as i32 << 8) | (self.b as i32)
    }


    pub fn from_i32(pixel: i32) -> Color {
        Color::rgb(((pixel >> 16) & 0xFF) as u8, ((pixel >> 8) & 0xFF) as u8, ((pixel) & 0xFF) as u8)
    }
}

impl Rand for Color {
    fn rand<R:Rng>(rng: &mut R) -> Color {
        Color::rgba(rng.gen_range(0, 255), rng.gen_range(0, 255), rng.gen_range(0, 255), 0x80)
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

pub enum RenderDestinationBuffer {
    Foreign(CVec<Color>),
    Own(Vec<Color>),
}

pub struct RenderDestination {
    buffer: RenderDestinationBuffer,
    width: u32,
    height: u32,
    bytes_per_line: u32,
}

pub struct RenderContext<'a> {
    render_dst: &'a mut RenderDestination
}

impl RenderDestination {

    fn as_ptr(&self) -> *mut u8 {
        unsafe {
            match self.buffer {
                Foreign(ref buffer) => {
                    std::mem::transmute(buffer.get(0).unwrap())
                },
                Own(ref buffer) => {
                    buffer.as_ptr() as *mut u8
                }
            }
        }
    }

    fn in_screen_x(&self, num: u32) -> bool {
        num >= 0 && num < self.width && num < self.height
    }

    fn in_screen_y(&self, num: u32) -> bool {
        num >= 0 && num < self.height
    }

    pub fn new(buffer_width: u32, buffer_height: u32) -> RenderDestination {
        let elem = buffer_width * buffer_height;
        let b = std::mem::size_of::<Color>();
        let mut buff = Vec::<Color>::with_capacity((buffer_width * buffer_height) as uint);
        for i in range(0, buffer_width * buffer_height) {
            buff.push(Color::rgb(0, 0, 0));
        }
        let dst = RenderDestination {
            buffer: Own(buff),
            width: buffer_width,
            height: buffer_height,
            bytes_per_line: buffer_width * 4,
        };

        return dst;
    }

    pub fn from_cvec<T>(buff: CVec<T>, buffer_width: u32, buffer_height: u32, bytes_per_line: u32) -> RenderDestination {
        unsafe {
            let ptr = buff.unwrap() as *mut u8;
            RenderDestination {
                buffer: Foreign(CVec::new(ptr as *mut Color, (buffer_width * buffer_height * 4) as uint)),
                width: buffer_width,
                height: buffer_height,
                bytes_per_line: bytes_per_line,
            }
        }
    }

    pub fn width(&self) -> u32 {self.width}
    pub fn height(&self) -> u32 {self.height}

    pub fn get(&self, x: u32, y: u32) -> Color {
        let index = (y * self.width + x) as uint;
        match self.buffer {
            Foreign(ref buffer) => {
                *buffer.get(index).unwrap()
            },
            Own(ref buffer) => {
                *buffer.get(index)
            }
        }
    }
}

impl<'a> RenderContext<'a> {
    pub fn print6x8(&self, x: u32, y: u32, fg_color: Color, bg_color: Color, text: &str) {
        assert!(self.render_dst.in_screen_y(y+7), "y = {}", y);
        let c_str = text.to_c_str();
        let ptr = c_str.as_ptr();
        unsafe {
            c_api::print6x8(x, y, fg_color.to_i32(), bg_color.to_i32(), ptr);
        }
    }

    pub fn draw_line_2d(&self, x1: u32, y1: u32, x2: u32, y2: u32, col: Color) {
        assert!(self.render_dst.in_screen_x(x1), "x1 = {}", x1);
        assert!(self.render_dst.in_screen_x(x2), "x2 = {}", x2);
        assert!(self.render_dst.in_screen_y(y1), "y1 = {}", y1);
        assert!(self.render_dst.in_screen_y(y2), "y2 = {}", y2);
        unsafe {
            c_api::drawline2d(x1 as f32, y1 as f32, x2 as f32, y2 as f32, col.to_i32());
        }
    }

    pub fn draw_point_3d(&self, pos: &vec3, col: Color) {
        unsafe {
            c_api::drawpoint3d(pos.x, pos.y, pos.z, col.to_i32());
        }
    }

    pub fn set_camera(&self, ori: &Orientation, focal_length: f32) {
        let ref dst = self.render_dst;
        unsafe {
            c_api::setcamera(&ori.pos.to_dpoint3d(),
                &ori.right_vec.to_dpoint3d(),
                &ori.down_vec.to_dpoint3d(),
                &ori.forward_vec.to_dpoint3d(),
                dst.width as f32* 0.5f32, dst.height as f32*0.5f32, dst.width as f32 * 0.5f32 * focal_length);
        }
    }

    pub fn opticast(&self) {
        unsafe {
            c_api::opticast();
        }
    }

    pub fn draw_image(&self, img: &Image, x: u32, y: u32, w: u32, h: u32) {
        let ref dst = self.render_dst;
        unsafe {
            c_api::drawpicinquad(
                img.ptr, img.bytes_per_line, img.width, img.height,
                dst.as_ptr(),  dst.bytes_per_line, dst.width, dst.height,
                x as f32, y as f32,
                (x + w) as f32, y as f32,
                (x + w) as f32, (y + h) as f32,
                x as f32, (y + h) as f32);
        }
    }


    pub fn draw_line_3d (&self, from: &vec3, to: &vec3, col: Color) {
        unsafe {
            c_api::drawline3d(from.x, from.y, from.z, to.x, to.y, to.z, col.to_i32());
        }
    }

    pub fn draw_sprite(&self, spr: &VxSprite) {
        unsafe {
            c_api::drawsprite(&spr.ptr);
        }
    }

    pub fn draw_sphere_with_z_buffer(&self, pos: &vec3, radius: f32, col: Color) {
        unsafe {
            c_api::drawspherefill(pos.x, pos.y, pos.z, -radius, col.to_i32());
        }
    }

    pub fn draw_sphere_without_z_buffer(&self, pos: &vec3, radius: f32, col: Color) {
        unsafe {
            c_api::drawspherefill(pos.x, pos.y, pos.z, radius, col.to_i32());
        }
    }
}

pub fn set_frame_buffer<'a>(render_dst: &'a mut RenderDestination) -> RenderContext<'a> {
    unsafe {
        c_api::voxsetframebuffer(render_dst.as_ptr(), render_dst.bytes_per_line, render_dst.width, render_dst.height);
        RenderContext {
            render_dst: render_dst,
        }
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

pub fn load_bsp(filename: &str) -> Result<Orientation, i32> {
    let mut ipo = c_api::dpoint3d { x: 0.0, y: 0.0, z: 0.0};
    let mut ist = c_api::dpoint3d { x: 0.0, y: 0.0, z: 0.0};
    let mut ihe = c_api::dpoint3d { x: 0.0, y: 0.0, z: 0.0};
    let mut ifo = c_api::dpoint3d { x: 0.0, y: 0.0, z: 0.0};
    let c_str = filename.to_c_str();
    let filename_ptr = c_str.as_ptr();
    match unsafe {
        c_api::loadbsp(filename_ptr, &mut ipo, &mut ist, &mut ihe, &mut ifo)
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

pub fn c_axis_rotate(pos: &mut c_api::point3d, axis: &vec3, w: f32) {
    unsafe {
        c_api::axisrotate(pos, axis.as_point3d(), w);
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
        c_api::setMaxScanDistToMax();
    }
}

pub fn set_max_scan_dist(dist: i32) {
    unsafe {
        c_api::setMaxScanDist(dist);
    }
}

pub fn set_norm_flash(pos: &vec3, flash_radius: i32, intens: i32) {
    unsafe {
        c_api::setnormflash(pos.x, pos.y, pos.z, flash_radius, intens);
    }
}


pub fn set_sphere(pos: &ivec3, radius: u32, operation_type: CsgOperationType) {
    unsafe {
        c_api::setsphere(pos.as_lpoint3d(), radius, operation_type.as_int());
    }
}

pub fn update_lighting(x0: i32, y0: i32, z0: i32, x1: i32, y1: i32, z1: i32) {
    unsafe {
        c_api::updatelighting(x0, y0, z0, x1, y1, z1);
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
        let col = col.map_or(-1, |c| c.to_i32());
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
        c_api::set_fogcol(param.to_i32());
    }
}

pub fn set_kv6col(param: Color) {
    unsafe {
        c_api::set_kv6col(param.to_i32());
    }
}

pub fn set_curcol(param: Color) {
    unsafe {
        c_api::set_curcol(param.to_i32());
    }
}

pub fn set_curpow(param: c_float) {
    unsafe {
        c_api::set_curpow(param);
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

pub fn melt_sphere(center: &ivec3, radius: u32) -> (VxSprite, u32) {
    let mut spr = c_api::vx5sprite::new();
    let melted_voxel_count = unsafe {
        c_api::meltsphere(&mut spr, center.as_lpoint3d(), radius)
    };
    (VxSprite {
        ptr: spr,
        managed_by_voxlap: false,
    }, melted_voxel_count)
}


pub struct Image {
    pub width: u32,
    pub height: u32,
    pub bytes_per_line: u32,
    ptr: *mut u8,
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            c_api::vox_free(self.ptr as *const c_void);
        }
    }
}

impl Image {
    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        let elem_count = (self.width * self.height) as uint;
        unsafe {
            let slice: &[i32] = mem::transmute( std::raw::Slice { data: self.ptr as *const u8, len: elem_count } );
            Color::from_i32(slice[(y * self.width + x) as uint])
        }
    }

    pub fn pixels(&self) -> &[i32] {
        let elem_count = (self.width * self.height) as uint;
        unsafe {
            let slice: &[i32] = mem::transmute( std::raw::Slice { data: self.ptr as *const u8, len: elem_count } );
            return slice;
        }
    }
}

pub fn load_image(filename: &str) -> Image {
    let c_str = filename.to_c_str();
    let filename_ptr = c_str.as_ptr();
    let mut ptr: i32 = 0;
    let mut bpl: u32 = 0;
    let mut xsiz: u32 = 0;
    let mut ysiz: u32 = 0;

    unsafe {
        c_api::kpzload(filename_ptr, &mut ptr, &mut bpl, &mut xsiz, &mut ysiz);
    }
    Image {
        width: xsiz,
        height: ysiz,
        bytes_per_line: bpl,
        ptr: ptr as *mut u8,
    }
}

pub fn draw_image(img: &Image, pos0: &vec3, pos1: &vec3, pos2: &vec3, pos3: &vec3) {
    unsafe {
        c_api::drawpolyquad(img.ptr as i32, img.bytes_per_line, img.width, img.height,
            pos0.x, pos0.y, pos0.z, 0f32, 0f32,
            pos1.x, pos1.y, pos1.z, 0f32, img.height as f32,
            pos2.x, pos2.y, pos2.z, img.width as f32, img.height as f32,
            pos3.x, pos3.y, pos3.z);
    }
}

pub fn is_voxel_solid(pos: &ivec3) -> bool {
    unsafe {
        c_api::isvoxelsolid(pos.x, pos.y, pos.z) == 1
    }
}

pub fn all_voxel_empty(start_pos: &ivec3, end_pos: &ivec3) -> bool {
    let x_step = if start_pos.x < end_pos.x {1} else {-1};
    let y_step = if start_pos.y < end_pos.y {1} else {-1};
    let z_step = if start_pos.z < end_pos.z {1} else {-1};
    for x in std::iter::range_step_inclusive(start_pos.x, end_pos.x, x_step) {
        for y in std::iter::range_step_inclusive(start_pos.y, end_pos.y, y_step) {
            for z in std::iter::range_step_inclusive(start_pos.z, end_pos.z, z_step) {
                if is_voxel_solid(&ivec3::new(x, y, z)) {
                    return false;
                }
            }
        }
    }
    return true;
}

/*#[repr(C)]
pub struct vspans {
    z1: c_char,
    z0: c_char,
    x: c_char,
    y: c_char,
}*/
pub fn meltspans(vspans: &[vspans], offs: &ivec3) -> (VxSprite, i32) {
    let mut spr = c_api::vx5sprite::new();
    let melted_voxel_count = unsafe {
        c_api::meltspans(&mut spr, vspans.as_ptr(), vspans.len() as i32, offs.as_lpoint3d())
    };
    (VxSprite {
        ptr: spr,
        managed_by_voxlap: false,
    }, melted_voxel_count)
}

pub fn get_cube(x: i32, y: i32, z: i32, ) -> Option<Color> {
    unsafe {
        let ptr_to_color = c_api::getcube(x, y, z) as *const i32;
        if ptr_to_color == ptr::null() || (ptr_to_color as i32) == 1 {
            return None;
        }
        return Some(Color::from_i32(*ptr_to_color));
    }
}

pub fn melt_rect2(pos: &ivec3, size: &ivec3) {
    let mut palette = vec![];
    let mut book_reviews: HashMap<i32, i32> = HashMap::new();

    let path = Path::new("lorem_ipsum.vox");
    let display = path.display();

    // Open a file in write-only mode, returns `IoResult<File>`
    let mut file = match File::create(&path) {
        Err(why) => fail!("couldn't create {}: {}", display, why.desc),
        Ok(file) => file,
    };
    file.write_le_i32(size.x);
    file.write_le_i32(size.y);
    file.write_le_i32(size.z);
    for x in range(pos.x, pos.x + size.x) {
        for y in range(pos.y, pos.y + size.y) {
            for z in range(pos.z, pos.z + size.z) {
                let color = get_cube(x, y, z);
                let index = match color {
                    None => 255,
                    Some(c) => {
                        let c_key = c.to_i32();
                        if book_reviews.contains_key(&c_key) {
                            *book_reviews.get(&c_key)
                        } else {
                            let index = palette.len() as i32;
                            if index == 255 {
                                fail!();
                            }
                            book_reviews.insert(c_key, index);
                            palette.push(c_key);
                            index
                        }
                    }
                };
                file.write_u8(index as u8);
            }
        }
    }
    for i in range(0, 255) {
        let color = if i < palette.len() {
            *palette.get(i)
        } else {
            0
        };
        let r = ((color>>18) & 0xFF) as u8;
        let g = ((color>>10) & 0xFF) as u8;
        let b = ((color>>2) & 0xFF) as u8;
        file.write_u8(r);
        file.write_u8(g);
        file.write_u8(b);
    }
}

pub fn melt_rect3(pos: &ivec3, size: &ivec3) {
    //let mut palette = vec![];
    //let mut book_reviews: HashMap<i32, i32> = HashMap::new();

    let path = Path::new("lorem_ipsum.rawvox");
    let display = path.display();

    // Open a file in write-only mode, returns `IoResult<File>`
    let mut file = match File::create(&path) {
        Err(why) => fail!("couldn't create {}: {}", display, why.desc),
        Ok(file) => file,
    };
    file.write_u8('X' as u8);
    file.write_u8('R' as u8);
    file.write_u8('A' as u8);
    file.write_u8('W' as u8);
    file.write_u8(0);
    file.write_u8(4);
    file.write_u8(8);
    file.write_u8(8);
    file.write_le_i32(size.x);
    file.write_le_i32(size.y);
    file.write_le_i32(size.z);
    file.write_le_i32(256);

    //palette.push(0);
    //book_reviews.insert(0, 0);
    for x in range(pos.x, pos.x + size.x) {
        for y in range(pos.y, pos.y + size.y) {
            for z in range(pos.z, pos.z + size.z) {
                let color = get_cube(x, y, z);
                let c = match color {
                    None => 0,
                    Some(c) => {
                        c.to_i32()
                    }
                };
                file.write_le_i32(c);

            }
        }
    }
    /*for i in range(0, 255) {
        let color = if i < palette.len() {
            *palette.get(i)
        } else {
            0
        };
        let r = ((color>>16) & 0xFF) as u8;
        let g = ((color>>8) & 0xFF) as u8;
        let b = ((color) & 0xFF) as u8;
        //file.write_u8(r);
        //file.write_u8(g);
        //file.write_u8(b);
    }*/
}

pub fn melt_rect(pos: &ivec3, size: &ivec3) -> (VxSprite, i32) {
    let mut spans = vec![];
    for y in range(pos.y, pos.y + size.y) {
        for x in range(pos.x, pos.x + size.x) {
            //for y in range(pos.y, pos.y + size.y) {
                spans.push(c_api::vspans {
                    z0: pos.z as u8,
                    z1: (pos.z + size.z) as u8,
                    x: x as u8,
                    y: y as u8
                });
            //}
        }
    }
    /*0x5 11 11 11,
0x5 12 11 11,
0x5 13 11 11,
0x6 11 11 11,
0x6 12 11 11,
0x6 13 11 11,*/
    /*spans.push(c_api::vspans {
                    z1: 0x5,
                    z0: 0x11,
                    x: 0x11,
                    y: 0x11
                });*/
return meltspans(spans.as_slice(), pos);
}

pub fn savekv6 (filename: &str, spr: &VxSprite) {
    let c_str = filename.to_c_str();
    let filename_ptr = c_str.as_ptr();
    unsafe {
        c_api::savekv6(filename_ptr, &*spr.ptr.voxnum);
    }
}

pub fn setkvx (filename: &str, pos: &ivec3, rot: i32) {
    let c_str = filename.to_c_str();
    let filename_ptr = c_str.as_ptr();
    unsafe {
        c_api::setkvx(filename_ptr, pos.x, pos.y, pos.z, rot, 0);
    }
}

pub struct DrawTileBuilder {
    tile_width: u32,
    tile_height: u32,
    screen_x: u32,
    screen_y: u32,
    zoom_x: u32,
    zoom_y: u32,
    row: u32,
    column: u32,
    tile_per_row: u32,
}

impl DrawTileBuilder {
    pub fn new() -> DrawTileBuilder {
        DrawTileBuilder {
            tile_width: 0,
            tile_height: 0,
            screen_x: 0,
            screen_y: 0,
            zoom_x: 1,
            zoom_y: 1,
            row: 0,
            column: 0,
            tile_per_row: 0,
        }
    }

    pub fn tile_width(mut self, param: u32) -> DrawTileBuilder {self.tile_width = param; self }
    pub fn tile_per_row(mut self, param: u32) -> DrawTileBuilder {self.tile_per_row = param; self }
    pub fn tile_height(mut self, param: u32) -> DrawTileBuilder {self.tile_height = param; self }
    pub fn screen_x(mut self, param: u32) -> DrawTileBuilder {self.screen_x = param; self }
    pub fn screen_y(mut self, param: u32) -> DrawTileBuilder {self.screen_y = param; self }
    pub fn row(mut self, param: u32) -> DrawTileBuilder {self.row = param; self }
    pub fn column(mut self, param: u32) -> DrawTileBuilder {self.column = param; self }
    pub fn draw(&self, img: &Image) {
        assert!(self.tile_per_row > 0, "tile_per_row must be > 0");
        do_draw_tile(img, self.tile_width, self.tile_height,
            self.screen_x, self.screen_y,
            self.zoom_x, self.zoom_y,
            self.row, self.column, self.tile_per_row);
    }
}

pub fn draw_tile() -> DrawTileBuilder {
    DrawTileBuilder::new()
}

fn do_draw_tile(img: &Image, tile_width: u32, tile_height: u32,
    screen_x: u32, screen_y: u32, zoom_x: u32, zoom_y: u32,
    row: u32, column: u32, tile_per_row: u32) {
    unsafe {
        let offset_per_tile = tile_width * tile_height * 4;
        let offset = (row*(tile_per_row*offset_per_tile) + (column*offset_per_tile)) as int;
        c_api::drawtile(img.ptr.offset(offset) as *const u8, img.bytes_per_line, tile_width, tile_height,
            -screen_x<<16, -screen_y<<16,
            0, 0,
            zoom_x<<16, zoom_y<<16,
            0, -1);
    }
}
