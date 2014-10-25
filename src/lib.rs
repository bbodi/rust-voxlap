
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

    pub fn len(&self) -> f32 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
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

    pub fn to_point3d(&self) -> c_api::point3d {
        c_api::point3d {
            x: self.x as f32,
            y: self.y as f32,
            z: self.z as f32,
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

pub struct Sprite {
    ptr: c_api::vx5sprite,
    managed_by_voxlap: bool
}

impl Sprite {
    pub fn new(filename: &str) -> Sprite {
        let mut spr = c_api::vx5sprite::new();
        let c_str = filename.to_c_str();
        let filename_ptr = c_str.as_ptr();
        unsafe {
            c_api::getspr(&mut spr, filename_ptr);
        }

        Sprite {
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

    pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
        self.ptr.s.x = x;
        self.ptr.h.y = y;
        self.ptr.f.z = z;
    }

    pub fn animate(&mut self, time_add: u32) {
        unsafe {
            c_api::animsprite(&self.ptr, time_add);
        }
    }

    pub fn save(&self, filename: &str) {
        unsafe {
            let c_str = filename.to_c_str();
            let filename_ptr = c_str.as_ptr();
            c_api::savekv6(filename_ptr, &*self.ptr.voxnum);
        }
    }
}

impl Drop for Sprite {
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

    pub fn black() -> Color {Color::rgb(0, 0, 0)}
    pub fn white() -> Color {Color::rgb(255, 255, 255)}
    pub fn red() -> Color {Color::rgb(255, 0, 0)}
    pub fn green() -> Color {Color::rgb(0, 255, 0)}
    pub fn blue() -> Color {Color::rgb(0, 0, 255)}

    pub fn dark_red() -> Color {Color::rgb(150, 0, 0)}
    pub fn dark_green() -> Color {Color::rgb(0, 150, 0)}
    pub fn dark_blue() -> Color {Color::rgb(0, 0, 150)}

    pub fn to_i32(&self) -> i32 {
        (self.a as i32 << 24) | (self.r as i32 << 16) | (self.g as i32 << 8) | (self.b as i32)
    }


    pub fn from_i32(pixel: i32) -> Color {
        Color::rgba(((pixel >> 16) & 0xFF) as u8, ((pixel >> 8) & 0xFF) as u8, ((pixel) & 0xFF) as u8, ((pixel>>24) & 0xFF) as u8)
    }
}

impl Rand for Color {
    fn rand<R:Rng>(rng: &mut R) -> Color {
        Color::rgba(rng.gen_range(0, 255), rng.gen_range(0, 255), rng.gen_range(0, 255), 0x80)
    }
}

// -------------------------  Initialization functions: -------------------------

pub struct Voxlap;

impl Drop for Voxlap {
    fn drop(&mut self) {
        unsafe {
        c_api::uninitvoxlap();
    }
    }
}

impl Voxlap {
    pub fn new() -> Result<Voxlap, ()> {
        unsafe {
            match c_api::initvoxlap() {
                0 => Ok(Voxlap),
                _ => Err(())
            }
        }
    }

    // --------------------------  File related functions: --------------------------

    pub fn load_default_map(&mut self, ) -> Orientation {
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

    pub fn load_vxl(&mut self, filename: &str) -> Result<Orientation, i32> {
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

    pub fn load_bsp(&mut self, filename: &str) -> Result<Orientation, i32> {
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

    pub fn load_sky(&mut self, filename: &str) -> Result<(), ()> {
        match unsafe {
            let c_str = filename.to_c_str();
            let filename_ptr = c_str.as_ptr();
            c_api::loadsky(filename_ptr)
        } {
            0 => Ok(()),
            _ => Err(()),
        }
    }

    pub fn project_2d(&self, pos: &vec3) -> ProjecionResult {
        let mut screen_x = 0f32;
        let mut screen_y = 0f32;
        let mut depth = 0f32;
        let visible = unsafe {
            c_api::project2d(pos.x, pos.y, pos.z, &mut screen_x, &mut screen_y, &mut depth) == 1
        };
        ProjecionResult {
            screen_x: screen_x as u32,
            screen_y: screen_y as u32,
            depth: depth,
            visible: visible
        }
    }

    pub fn melt_sphere(&self, center: &ivec3, radius: u32) -> (Sprite, u32) {
        let mut spr = c_api::vx5sprite::new();
        let melted_voxel_count = unsafe {
            c_api::meltsphere(&mut spr, center.as_lpoint3d(), radius)
        };
        (Sprite {
            ptr: spr,
            managed_by_voxlap: false,
        }, melted_voxel_count)
    }

    pub fn meltspans(&self, vspans: &[vspans], offs: &ivec3) -> (Sprite, i32) {
        let mut spr = c_api::vx5sprite::new();
        let melted_voxel_count = unsafe {
            c_api::meltspans(&mut spr, vspans.as_ptr(), vspans.len() as i32, offs.as_lpoint3d())
        };
        (Sprite {
            ptr: spr,
            managed_by_voxlap: false,
        }, melted_voxel_count)
    }

    pub fn set_frame_buffer<'a>(&mut self, render_dst: &'a mut RenderDestination) -> RenderContext<'a> {
        unsafe {
            c_api::voxsetframebuffer(render_dst.as_mut_ptr(), render_dst.bytes_per_line, render_dst.width, render_dst.height);
            RenderContext {
                render_dst: render_dst,
            }
        }
    }


    pub fn hitscan(&mut self, pos: &vec3, dir: &vec3) -> Option<HitScanResult> {
        let mut voxel_pos = ivec3::new(0, 0, 0);
        let mut face: i32 = 0;
        unsafe {
            let mut color_ptr: *mut i32 = ptr::null_mut();
            c_api::hitscan(&pos.to_dpoint3d(), &dir.to_dpoint3d(), voxel_pos.as_mut_lpoint3d(), &mut color_ptr, &mut face);
            if color_ptr == ptr::null_mut() {
                None
            } else {
                Some(HitScanResult {
                    color_ptr: color_ptr,
                    hit_face: match face {
                        0 => Some(ZMin),
                        1 => Some(ZMax),
                        2 => Some(XMin),
                        3 => Some(XMax),
                        4 => Some(YMin),
                        5 => Some(YMax),
                        _ => None,  // -1 if inside solid
                    },
                    pos: voxel_pos,
                })
            }
        }
    }

    pub fn with_hitscan(&mut self, pos: &vec3, dir: &vec3, func: |&mut Voxlap, &mut HitScanResult|) -> bool  {
        let mut voxel_pos = ivec3::new(0, 0, 0);
        let mut face: i32 = 0;
        unsafe {
            let mut color_ptr: *mut i32 = ptr::null_mut();
            c_api::hitscan(&pos.to_dpoint3d(), &dir.to_dpoint3d(), voxel_pos.as_mut_lpoint3d(), &mut color_ptr, &mut face);
            if color_ptr == ptr::null_mut() {
                return false;
            } else {
                func(self, &mut HitScanResult {
                    color_ptr: color_ptr,
                    hit_face: match face {
                        0 => Some(ZMin),
                        1 => Some(ZMax),
                        2 => Some(XMin),
                        3 => Some(XMax),
                        4 => Some(YMin),
                        5 => Some(YMax),
                        _ => None,  // -1 if inside solid
                    },
                    pos: voxel_pos
                });
                return true;
            }
        }
    }

    pub fn sprhitscan(&self, pos: &vec3, dir: &vec3, spr: &Sprite, ) -> Option<SprHitScanResult> {
        let mut voxel_pos = ivec3::new(0, 0, 0);
        let mut unk = 100f32;
        unsafe {
            let mut kv6voxtype_ptr: *mut c_api::kv6voxtype = ptr::null_mut();
            c_api::sprhitscan(&pos.to_dpoint3d(), &dir.to_dpoint3d(), &spr.ptr, voxel_pos.as_mut_lpoint3d(), &mut kv6voxtype_ptr, &mut unk);
            if kv6voxtype_ptr == ptr::null_mut() {
                None
            } else {
                Some(SprHitScanResult {
                    //color: Color::from_i32(*color_ptr),
                    pos: voxel_pos
                })
            }
        }
    }

    pub fn calc_air_radius(&self, pos: &vec3, maxcr: f32) -> f32 {
        unsafe {
            c_api::findmaxcr(pos.x as f64, pos.y as f64, pos.z as f64, maxcr as f64) as f32
        }
    }

    pub fn clip_move(&self, pos: &mut vec3, move_vec: &vec3, acr: f64) {
        let mut dpos = pos.to_dpoint3d();
        unsafe {
            c_api::clipmove(&mut dpos, &move_vec.to_dpoint3d(), acr);
        }
        pos.fill_from_dpoint3d(dpos);
    }

    pub fn estimate_normal_vector(&self, pos: &ivec3) -> vec3 {
        let mut dir = vec3::new(0f32, 0f32, 0f32);
        unsafe {
            c_api::estnorm(pos.x, pos.y, pos.z, dir.as_mut_point3d());
        }
        return dir;
    }

    // --------------------------- VXL reading functions: ---------------------------

    pub fn is_voxel_solid(&self, pos: &ivec3) -> bool {
        unsafe {
            c_api::isvoxelsolid(pos.x, pos.y, pos.z) == 1
        }
    }

    pub fn all_voxel_empty(&self, start_pos: &ivec3, end_pos: &ivec3) -> bool {
        let x_step = if start_pos.x < end_pos.x {1} else {-1};
        let y_step = if start_pos.y < end_pos.y {1} else {-1};
        let z_step = if start_pos.z < end_pos.z {1} else {-1};
        for x in std::iter::range_step_inclusive(start_pos.x, end_pos.x, x_step) {
            for y in std::iter::range_step_inclusive(start_pos.y, end_pos.y, y_step) {
                for z in std::iter::range_step_inclusive(start_pos.z, end_pos.z, z_step) {
                    if self.is_voxel_solid(&ivec3::new(x, y, z)) {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    pub fn any_voxel_solid(&self, x: u32, y: u32, z0: i32, z1: i32) -> bool {
        unsafe {
            c_api::anyvoxelsolid(x, y, z0, z1) != 0
        }
    }

    pub fn any_voxel_empty(&self, x: u32, y: u32, z0: i32, z1: i32) -> bool {
        unsafe {
            c_api::anyvoxelempty(x, y, z0, z1) != 0
        }
    }

    pub fn get_floor_z(&self, pos: &ivec3) -> i32 {
        unsafe {
            c_api::getfloorz(pos.x, pos.y, pos.z)
        }
    }

    pub fn get_cube(&self, x: i32, y: i32, z: i32, ) -> Option<Color> {
        unsafe {
            let ptr_to_color = c_api::getcube(x, y, z) as *const i32;
            if ptr_to_color == ptr::null() || (ptr_to_color as i32) == 1 {
                return None;
            }
            return Some(Color::from_i32(*ptr_to_color));
        }
    }

    // --------------------------- VXL writing functions: ---------------------------

    pub fn set_cube(&mut self, pos: &ivec3, col: Option<Color>) {
        unsafe {
            let col = col.map_or(-1, |c| c.to_i32());
            c_api::setcube(pos.x, pos.y, pos.z, col);
        }
    }

    pub fn set_sphere(&mut self, pos: &ivec3, radius: u32, operation_type: CsgOperationType) {
        unsafe {
            c_api::setsphere(pos.as_lpoint3d(), radius, operation_type.as_int());
        }
    }

    pub fn set_elliposid(&mut self, focus_1: &ivec3, focus_2: &ivec3, radius: u32, operation_type: CsgOperationType) {
        unsafe {
            // 0: fast&permanent change, 1:backup (much slower: used in VOXED)
            c_api::setellipsoid(focus_1.as_lpoint3d(), focus_2.as_lpoint3d(), radius as i32, operation_type.as_int(), 0);
        }
    }

    pub fn set_cylinder(&mut self, end_point1: &ivec3, end_point2: &ivec3, radius: u32, operation_type: CsgOperationType) {
        unsafe {
            // 0: fast&permanent change, 1:backup (much slower: used in VOXED)
            c_api::setcylinder(end_point1.as_lpoint3d(), end_point2.as_lpoint3d(), radius as i32, operation_type.as_int(), 0);
        }
    }

    pub fn set_rect(&mut self, p1: &ivec3, p2: &ivec3, mode: CsgOperationType) {
        unsafe {
            c_api::setrect(p1.as_lpoint3d(), p2.as_lpoint3d(), mode.as_int());
        }
    }

    pub fn set_triangle(&mut self, p1: &ivec3, p2: &ivec3, p3: &ivec3) {
        unsafe {
            // 0: fast&permanent change, 1:backup (much slower: used in VOXED)
            c_api::settri(&p1.to_point3d(), &p2.to_point3d(), &p3.to_point3d(), 0);
        }
    }

    pub fn set_sector(&mut self, vertices: &[ivec3], edges: &[u32], thick: f32, mode: CsgOperationType) {
        let ivecs = vertices.iter().map(|&x| x.to_point3d()).collect::<Vec<c_api::point3d>>();
        unsafe {
            // 0: fast&permanent change, 1:backup (much slower: used in VOXED)
            c_api::setsector(ivecs.as_ptr(), edges.as_ptr(), vertices.len() as u32, thick, mode.as_int(), 0);
        }
    }

    pub fn set_spans(&self, vspans: &[vspans], offs: &ivec3, mode: CsgOperationType) {
        unsafe {
            c_api::setspans(vspans.as_ptr(), vspans.len() as u32, offs.as_lpoint3d(), mode.as_int());
        }
    }

    pub fn set_heightmap(&self, buff: &[u8], width: u32, height: u32, x0: u32, y0: u32) {
        unsafe {
            let bytes_per_line = width;
            c_api::setheightmap(buff.as_ptr(), bytes_per_line, width, height, x0, y0, x0+width, y0+height);
        }
    }

    pub fn set_kv6_into_vxl_memory(&mut self, spr: &Sprite, operation_type: CsgOperationType) {
        unsafe {
            c_api::setkv6(&spr.ptr, operation_type.as_int());
        }
    }

    pub fn set_kvx_into_vxl_memory (&mut self, filename: &str, pos: &ivec3, rot: i32) {
        let c_str = filename.to_c_str();
        let filename_ptr = c_str.as_ptr();
        unsafe {
            c_api::setkvx(filename_ptr, pos.x, pos.y, pos.z, rot, 0);
        }
    }

    // sethull3d
    // setlathe
    // setblobs
    // setfloodfill3d
    // sethollowfill
    pub fn set_norm_flash(&mut self, pos: &vec3, flash_radius: i32, intens: i32) {
        unsafe {
            c_api::setnormflash(pos.x, pos.y, pos.z, flash_radius, intens);
        }
    }


    // ---------------------------- VXL MISC functions:  ----------------------------
    // updatebbox

    pub fn update_vxl(&mut self) {
        unsafe {
            c_api::updatevxl();
        }
    }

    pub fn generate_vxl_mipmapping(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
        unsafe {
            c_api::genmipvxl(x0, y0, x1, y1);
        }
    }

    pub fn update_lighting(&mut self, x0: i32, y0: i32, z0: i32, x1: i32, y1: i32, z1: i32) {
        unsafe {
            c_api::updatelighting(x0, y0, z0, x1, y1, z1);
        }
    }

    // ------------------------- Falling voxels functions: --------------------------
    // TODO

    // ----------------------- Procedural texture functions: ------------------------
    // TODO

    // -------------------------- VX5 structure variables: --------------------------

    pub fn set_max_scan_dist_to_max(&mut self, ) {
        unsafe {
            c_api::setMaxScanDistToMax();
        }
    }

    pub fn set_max_scan_dist(&mut self, dist: i32) {
        unsafe {
            c_api::setMaxScanDist(dist);
        }
    }


    pub fn set_lighting_mode(&mut self, mode: LightingMode) {
        let m = match mode {
            NoSpecialLighting => 0,
            SimpleEstimatedNormalLighting => 1,
            MultiplePointSourceLighting => 2,
        };
        unsafe {
            c_api::setLightingMode(m);
        }

    }

    pub fn set_raycast_density(&mut self, param: i32) {
        assert!(param >= 1, "Param cannot be < 0!");
        unsafe {
            c_api::set_anginc(param);
        }
    }

    pub fn get_raycast_density(&self, ) -> i32 {
        unsafe {
            c_api::get_anginc()
        }
    }

    pub fn set_fog_color(&mut self, param: Color) {
        unsafe {
            c_api::set_fogcol(param.to_i32());
        }
    }

    pub fn set_kv6col(&mut self, param: Color) {
        unsafe {
            c_api::set_kv6col(param.to_i32());
        }
    }

    pub fn set_curcol(&mut self, param: Color) {
        unsafe {
            c_api::set_curcol(param.to_i32());
        }
    }

    pub fn set_curpow(&self, param: c_float) {
        unsafe {
            c_api::set_curpow(param);
        }
    }

    pub fn get_max_xy_dimension(&self, ) -> i32 {
        unsafe { c_api::getVSID() }
    }

    pub fn melt_rect(&self, pos: &ivec3, size: &ivec3) -> (Sprite, i32) {
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

        return self.meltspans(spans.as_slice(), pos);
    }

    pub fn can_see(&self, starting_point: &vec3, ending_point: &vec3) -> VisibilityResult {
        let mut hit_pos = ivec3::new(0, 0, 0);
        match unsafe {
            c_api::cansee(starting_point.as_point3d(), ending_point.as_point3d(), hit_pos.as_mut_lpoint3d())
        } {
            1 => CanSee,
            _ => CannotSee(hit_pos)
        }
    }
}

// ---------------- Picture functions (PNG,JPG,TGA,GIF,PCX,BMP): ----------------

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

// ------------------------------- ZIP functions: -------------------------------
// TODO
pub fn kz_addstack (filename: &str) {
    let c_str = filename.to_c_str();
    let filename_ptr = c_str.as_ptr();

    unsafe {
        c_api::kzaddstack(filename_ptr);
    }
}


// -------------------------  Screen related functions: -------------------------

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


impl RenderDestination {

    fn as_ptr(&self) -> *const u8 {
        unsafe {
            match self.buffer {
                Foreign(ref buffer) => {
                    std::mem::transmute(buffer.get(0).unwrap())
                },
                Own(ref buffer) => {
                    buffer.as_ptr() as *const u8
                }
            }
        }
    }

    fn as_mut_ptr(&mut self) -> *mut u8 {
        unsafe {
            match self.buffer {
                Foreign(ref buffer) => {
                    std::mem::transmute(buffer.get(0).unwrap())
                },
                Own(ref mut buffer) => {
                    buffer.as_mut_ptr() as *mut u8
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
                buffer[index]
            }
        }
    }

    pub fn set(&mut self, x: u32, y: u32, col: Color) {
        let index = (y * self.width + x) as uint;
        match self.buffer {
            Foreign(ref buffer) => {
                fail!("Cannot set color for a Foreign (allocated by Voxlap) buffer");
            },
            Own(ref mut buffer) => {
                (buffer.as_mut_slice())[index] = col;
                //println!("setting {},{} to {}", x, y, col);
            }
        }
    }
}

impl<'a> RenderContext<'a> {

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

    pub fn draw_point_2d(&self, x: u32, y: u32, col: Color) {
        assert!(self.render_dst.in_screen_x(x), "x = {}", x);
        assert!(self.render_dst.in_screen_y(y), "y = {}", y);
        unsafe {
            c_api::drawpoint2d(x, y, col.to_i32());
        }
    }

    pub fn draw_point_3d(&self, pos: &vec3, col: Color) {
        unsafe {
            c_api::drawpoint3d(pos.x, pos.y, pos.z, col.to_i32());
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

    pub fn draw_line_3d_with_z_buffer(&self, from: &vec3, to: &vec3, col: Color) {
        // Set alpha of col to zero to enable Z-buffering
        unsafe {
            c_api::drawline3d(from.x, from.y, from.z, to.x, to.y, to.z, col.to_i32() & 0x00FFFFFF);
        }
    }

    pub fn draw_line_3d_without_z_buffer(&self, from: &vec3, to: &vec3, col: Color) {
        // Set alpha of col to non-zero to disable Z-buffering
        unsafe {
            c_api::drawline3d(from.x, from.y, from.z, to.x, to.y, to.z, col.to_i32() | 0xFF000000);
        }
    }

    pub fn draw_sphere_with_z_buffer(&self, pos: &vec3, radius: f32, col: Color) {
        unsafe {
            c_api::drawspherefill(pos.x, pos.y, pos.z, -radius, col.to_i32());
        }
    }

    pub fn draw_sphere_without_z_buffer(&mut self, pos: &vec3, radius: f32, col: Color) {
        unsafe {
            c_api::drawspherefill(pos.x, pos.y, pos.z, radius, col.to_i32());
        }
    }

    pub fn draw_image_2d(&mut self, img: &Image, x: u32, y: u32, w: u32, h: u32) {
        let ref mut dst = self.render_dst;
        unsafe {
            c_api::drawpicinquad(
                img.ptr, img.bytes_per_line, img.width, img.height,
                dst.as_mut_ptr(),  dst.bytes_per_line, dst.width, dst.height,
                x as f32, y as f32,
                (x + w) as f32, y as f32,
                (x + w) as f32, (y + h) as f32,
                x as f32, (y + h) as f32);
        }
    }

    pub fn draw_image_3d(img: &Image, pos0: &vec3, pos1: &vec3, pos2: &vec3, pos3: &vec3) {
        unsafe {
            c_api::drawpolyquad(img.ptr as i32, img.bytes_per_line, img.width, img.height,
                pos0.x, pos0.y, pos0.z, 0f32, 0f32,
                pos1.x, pos1.y, pos1.z, 0f32, img.height as f32,
                pos2.x, pos2.y, pos2.z, img.width as f32, img.height as f32,
                pos3.x, pos3.y, pos3.z);
        }
    }

    pub fn print4x6(&self, x: u32, y: u32, fg_color: Color, bg_color: Color, text: &str) {
        assert!(self.render_dst.in_screen_y(y+5), "y = {}", y);
        let c_str = text.to_c_str();
        let ptr = c_str.as_ptr();
        unsafe {
            c_api::print4x6(x, y, fg_color.to_i32(), bg_color.to_i32(), ptr);
        }
    }

    pub fn print6x8(&self, x: u32, y: u32, fg_color: Color, bg_color: Option<Color>, text: &str) {
        assert!(self.render_dst.in_screen_y(y+7), "y = {}", y);
        let c_str = text.to_c_str();
        let ptr = c_str.as_ptr();
        let bg_color = match bg_color {
            None => -1,
            Some(c) => c.to_i32() & 0x00FFFFFF
        };
        unsafe {
            c_api::print6x8(x, y, fg_color.to_i32(), bg_color, ptr);
        }
    }

    pub fn draw_tile(&self, img: &Image, tile_width: u32, tile_height: u32,
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

    pub fn save_to_file(&self, filename: &str) {
        unsafe {
            let c_str = filename.to_c_str();
            let filename_ptr = c_str.as_ptr();
            c_api::screencapture32bit(filename_ptr);
        }
    }

    pub fn save_panorama_to_file(&self, pos: &vec3, filename: &str, box_size: u32) {
        unsafe {
            let c_str = filename.to_c_str();
            let filename_ptr = c_str.as_ptr();
            c_api::surroundcapture32bit(&pos.to_dpoint3d(), filename_ptr, box_size);
        }
    }


    pub fn draw_sprite(&self, spr: &Sprite) {
        unsafe {
            c_api::drawsprite(&spr.ptr);
        }
    }
}

pub struct ProjecionResult {
    pub screen_x: u32,
    pub screen_y: u32,
    pub depth: f32,
    pub visible: bool
}



// -------------------------  Physics helper functions: -------------------------


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

pub enum VisibilityResult {
    CanSee,
    CannotSee(ivec3),
}

pub enum CubeFace {
    ZMin,
    ZMax,
    XMin,
    XMax,
    YMin,
    YMax,
    InsideSolid
}

pub struct HitScanResult {
    pub hit_face: Option<CubeFace>,
    pub pos: ivec3,
    color_ptr: *mut i32,
}

impl HitScanResult {
    pub fn set_color(&mut self, color: Color) {
        unsafe {
            *self.color_ptr = color.to_i32();
        }
    }

    pub fn get_color(&self) -> Color {
        unsafe {
            Color::from_i32(*self.color_ptr)
        }
    }
}


pub struct SprHitScanResult {
    //pub color: Color,
    pub pos: ivec3,
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
    pub fn draw(&self, img: &Image, render_context: &RenderContext) {
        assert!(self.tile_per_row > 0, "tile_per_row must be > 0");
        render_context.draw_tile(img, self.tile_width, self.tile_height,
            self.screen_x, self.screen_y,
            self.zoom_x, self.zoom_y,
            self.row, self.column, self.tile_per_row);
    }
}

pub fn draw_tile() -> DrawTileBuilder {
    DrawTileBuilder::new()
}
