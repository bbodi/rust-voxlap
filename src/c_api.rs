use libc::{c_long, c_int, c_char, c_float, c_double, c_void, c_short, c_uchar, c_ushort, c_uint, c_ulong};
use std::ptr;

#[repr(C)]
pub struct lpoint3d {
    pub x: c_long,
    pub y: c_long,
    pub z: c_long,
}

#[repr(C)]
pub struct point3d {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
}

#[repr(C)]
pub struct point4d {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
    pub z2: c_float,
}

#[repr(C)]
pub struct dpoint3d {
    pub x: c_double,
    pub y: c_double,
    pub z: c_double,
}

#[repr(C)]
pub struct kv6data {
    pub leng: c_long,
    pub xsiz: c_long,
    pub ysiz: c_long,
    pub zsiz: c_long,
    pub xpiv: c_float,
    pub ypiv: c_float,
    pub zpiv: c_float,
    pub numvoxs: c_ulong,
    pub namoff: c_long,
    pub lowermip: *const kv6data,
    pub vox: *const kv6voxtype,      //numvoxs*sizeof(kv6voxtype)
    pub xlen: *const c_ulong,  //xsiz*sizeof(long)
    pub ylen: *const c_ushort, //xsiz*ysiz*sizeof(short)
}

#[repr(C)]
pub struct vx5sprite {
    pub pos: point3d, /// position in VXL coordinates
    pub flags: c_long, /// flags bit 0:0=use normal shading, 1=disable normal shading
                              /// flags bit 1:0=points to kv6data, 1=points to kfatype
                              /// flags bit 2:0=normal, 1=invisible sprite
    pub s: point3d, /// kv6data.xsiz direction in VXL coordinates

    pub voxnum: *mut kv6data, /// pointer to KV6 voxel data (bit 1 of flags = 0) or to KFA animation  (bit 1 of flags = 1)
    pub h: point3d,          /// kv6data.ysiz direction in VXL coordinates
    pub kfatim: c_long,        /// time (in milliseconds) of KFA animation
    pub f: point3d,          /// kv6data.zsiz direction in VXL coordinates
    /// make vx5sprite exactly 64 bytes :) ASSERT THAT IT IS 64 byte long!
    pub okfatim: c_long
}

impl vx5sprite {
    pub fn new() -> vx5sprite {
        vx5sprite {
            pos: point3d{x: 0f32, y: 0f32, z: 0f32},
            s: point3d{x: 1f32, y: 0f32, z: 0f32},
            h: point3d{x: 0f32, y: 1f32, z: 0f32},
            f: point3d{x: 0f32, y: 0f32, z: 1f32},
            okfatim: 0,
            kfatim: 0,
            voxnum: ptr::null_mut(),
            flags: 0,
        }
    }
}

#[repr(C)]
pub struct hingetype {
    parent: c_long,
    p: [point3d, ..2],
    v: [point3d, ..2],
    vmin: c_short,
    vmax: c_short,
    htype: c_char,
    filler: [c_char, ..7]
}

#[repr(C)]
pub struct kv6voxtype {
    col: c_long,
    z: c_ushort,
    vis: c_char,
    dir: c_char,
}

#[repr(C)]
pub struct seqtyp {
    tim: c_long,
    frm: c_long,
}

#[repr(C)]
pub struct vspans {
    pub z1: c_uchar,
    pub z0: c_uchar,
    pub x: c_uchar,
    pub y: c_uchar,
}

#[repr(C)]
pub struct kfatype {
    pub numspr: c_long,
    pub numhin: c_long,
    pub numfrm: c_long,
    pub seqnum: c_long,
    pub namoff: c_long,
        basekv6: *const kv6data,    // kv6data
        spr: *const vx5sprite,      //[numspr]
        hinge: *const hingetype,    //[numhin]
        hingesort: *const c_long,   //[numhin]
        frmval: *const c_short,     // [numfrm][numhin]
        seq: *const seqtyp,        //[seqnum]
    }

    #[link(name="voxlap")]
    extern "C" {

        /// -------------------------  Initialization functions: -------------------------
        pub fn initvoxlap() -> c_int;
        pub fn uninitvoxlap();

        /// --------------------------  File related functions: --------------------------
        pub fn loadnul(ipo: *mut dpoint3d, ist: *mut dpoint3d, ihe: *mut dpoint3d, ifo: *mut dpoint3d);
        pub fn loadsxl (filename: *const c_char, vxlnam: *mut*mut c_char,
          skynam: *mut*mut c_char, globst: *mut*mut c_char) -> c_long;

        pub fn parspr (spr: &mut vx5sprite, userst: *mut*mut c_char) -> *const c_char;

        pub fn loaddta (filename: *const c_char, pos: &mut dpoint3d, right_vec: &mut dpoint3d, down_vec: &mut dpoint3d, forward_vec: &mut dpoint3d) -> c_long;

        pub fn loadpng (filename: *const c_char, pos: &mut dpoint3d,
          right_vec: &mut dpoint3d, down_vec: &mut dpoint3d, forward_vec: &mut dpoint3d) -> c_long;

        pub fn loadbsp (filename: *const c_char, pos: &mut dpoint3d,
          right_vec: &mut dpoint3d, down_vec: &mut dpoint3d, forward_vec: &mut dpoint3d) -> c_long;

        pub fn loadvxl (filename: *const c_char, pos: &mut dpoint3d,
          right_vec: &mut dpoint3d, down_vec: &mut dpoint3d, forward_vec: &mut dpoint3d) -> c_long;

        pub fn savevxl (filename: *const c_char, pos: &mut dpoint3d,
         right_vec: &mut dpoint3d, down_vec: &mut dpoint3d, forward_vec: &mut dpoint3d) -> c_long;

        pub fn loadsky (filename: *const c_char) -> c_long;

        /// -------------------------  Screen related functions: -------------------------
        pub fn voxsetframebuffer(ptr_to_dst_buffer: *mut u8, pitch: c_ulong, buffer_width: c_uint, buffer_height: c_uint);

        pub fn setsideshades (sto: c_char, sbo: c_char, sle: c_char, sri: c_char, sup: c_char, sdo: c_char);

        pub fn setcamera(ipo: &dpoint3d, ist: &dpoint3d, ihe: &dpoint3d, ifo: &dpoint3d, dahx: c_float, dahy: c_float, dahz: c_float);
        pub fn opticast();

        pub fn drawpoint2d (sx: c_long, sy: c_long, col: c_long);

        pub fn drawpoint3d (x0: c_float, y0: c_float, z0: c_float, col: c_long);

        pub fn drawline2d (x1: c_float, y1: c_float, x2: c_float, y2: c_float, col: c_long);

        pub fn drawline3d (x0: c_float, y0: c_float, z0: c_float,
          x1: c_float, y1: c_float, z1: c_float, col: c_long);

        pub fn project2d (x: c_float, y: c_float, z: c_float, px: *mut c_float, py: *mut c_float, sx: *mut c_float) -> c_long;

        pub fn drawspherefill (ox: c_float, oy: c_float, oz: c_float, bakrad: c_float, col: c_long);

        pub fn drawpicinquad (rpic: *mut u8, rbpl: c_ulong, rxsiz: c_ulong, rysiz: c_ulong,
          wpic: *mut u8, wbpl: c_ulong, wxsiz: c_ulong, wysiz: c_ulong,
          x0: c_float, y0: c_float, x1: c_float, y1: c_float,
          x2: c_float, y2: c_float, x3: c_float, y3: c_float);

        pub fn drawpolyquad (rpic: c_long, rbpl: c_ulong, rxsiz: c_ulong, rysiz: c_ulong,
         x0: c_float, y0: c_float, z0: c_float, u0: c_float, v0: c_float,
         x1: c_float, y1: c_float, z1: c_float, u1: c_float, v1: c_float,
         x2: c_float, y2: c_float, z2: c_float, u2: c_float, v2: c_float,
         x3: c_float, y3: c_float, z3: c_float);

        pub fn print4x6(x: c_long, y: c_long, fg_color: c_long, bg_color: c_long, fmt: *const c_char, ...);

        pub fn print6x8(x: c_ulong, y: c_ulong, fg_color: c_long, bg_color: c_long, fmt: *const c_char, ...);

        /// Draws a 32-bit color texture from memory to the screen. This is the
        ///   low-level function used to draw text loaded from a PNG,JPG,TGA,GIF.
        ///     tf: pointer to top-left corner of SOURCE picture
        ///     tp: pitch (bytes per line) of the SOURCE picture
        ///  tx,ty: dimensions of the SOURCE picture
        /// tcx,tcy: texel (<<16) at (sx,sy). Set this to (0,0) if you want (sx,sy)
        ///            to be the top-left corner of the destination
        ///  sx,sy: screen coordinate (matches the texture at tcx,tcy)
        ///  xz,yz: x&y zoom, all (<<16). Use (65536,65536) for no zoom change
        /// black,white: shade scale (ARGB format). For no effects, use (0,-1)
        ///   NOTE: if alphas of black&white are same, then alpha channel ignored
        pub fn drawtile (tf: *const u8, tp: c_ulong, tx: c_ulong, ty: c_ulong, tcx: c_ulong, tcy: c_ulong,
            sx: c_ulong, sy: c_ulong, xz: c_ulong, yz: c_ulong, black: c_long, white: c_long);

        /// Captures a screenshot of the current frame to disk. The current frame
        ///   is defined by the last call to the voxsetframebuffer function. NOTE:
        ///   you MUST call this function while video memory is accessible. In
        ///   DirectX, that means it must be between a call to startdirectdraw and
        ///   stopdirectdraw.
        /// fname: filename to write to (writes uncompressed .PNG format)
        /// returns: 0:always
        pub fn screencapture32bit (fname: *const c_char) -> c_long;

        /// Generates a cubic panorama (skybox) from the given position
        ///   This is an old function that is very slow, but it is pretty cool
        ///   being able to view a full panorama screenshot. Unfortunately, it
        ///   doesn't draw sprites or the sky.
        ///   pos: VXL map position of camera
        /// fname: filename to write to (writes uncompressed .PNG format)
        /// boxsiz: length of side of square. I recommend using 256 or 512 for this.
        /// returns: 0:always
        pub fn surroundcapture32bit (pos: &dpoint3d, fname: *const c_char, boxsiz: c_long) -> c_long;
        /// -------------------------  Sprite related functions: -------------------------
        /// Loads a .KV6 voxel sprite into memory. It malloc's the array for you and
        ///   returns the pointer to the loaded vx5sprite. If the same filename was
        ///   passed before to this function, it will return the pointer to the
        ///   previous instance of the .KV6 buffer in memory (It will NOT load the
        ///   same file twice). Uninitvoxlap() de-allocates all .KV6 sprites for
        ///   you.
        /// Other advanced info: Uses a 256-entry hash table to compare filenames, so
        ///   it should be fast. If you want to modify a .KV6 without affecting all
        ///   instances, you must allocate&de-allocate your own kv6data structure,
        ///   and use memcpy. The buffer is kv6data.leng bytes (inclusive: c_long).
        /// kv6nam: .KV6 filename
        /// returns: pointer to malloc'ed kv6data structure. Do NOT free this buffer
        ///         yourself! Returns 0 if there's an error - such as bad filename.
        pub fn getkv6 (kv6nam: *const c_char) -> *mut vx5sprite;

        /// Loads a .KFA file and its associated .KV6 voxel sprite into memory. Works
        ///   just like getkv6() for for .KFA files.
        /// kfanam: .KFA filename
        /// returns: pointer to malloc'ed kfatype structure. Do NOT free this buffer
        ///         yourself! Returns 0 if there's an error - such as bad filename.
        pub fn getkfa (kfanam: *const c_char) -> *mut kfatype;

        /// If you generate any sprites using one of the melt* functions, and then
        ///   generate mip-maps for it, you can use this function to de-allocate
        ///   all mip-maps of the .KV6 safely. You don't need to use this for
        ///   kv6data objects that were loaded by getkv6,getkfa, or getspr since
        ///   these functions automatically de-allocate them using this function.
        pub fn freekv6 (kv6: &kv6data);

        /// This could be a handy function for debugging I suppose. Use it to save
        ///   .KV6 sprites to disk.
        /// filnam: filename of .KV6 to save to disk. It's your responsibility to
        ///         make sure it doesn't overwrite a file of the same name.
        ///     kv: pointer to .KV6 object to save to disk.
        pub fn savekv6 (filnam: *const c_char, kv: &kv6data);

        /// Cover-up function to handle both .KV6 and .KFA files. It looks at the
        ///   filename extension and uses the appropriate function (either getkv6
        ///   or getkfa) and sets the sprite flags depending on the type of file.
        ///   The file must have either .KV6 or .KFA as the filename extension. If
        ///   you want to use weird filenames, then use getkv6/getkfa instead.
        ///    spr: Pointer to sprite structure that you provide. getspr() writes:
        ///            only to the kv6data/voxtype, kfatim, and flags members.
        /// filnam: filename of either a .KV6 or .KFA file.
        pub fn getspr (spr: &mut vx5sprite, filnam: *const c_char);

        /// Generate 1 more mip-level for a .KV6 sprite. This function generates a
        ///   lower MIP level only if kv6->lowermip is NULL, and kv6->xsiz,
        ///   kv6->ysiz, and kv6->zsiz are all >= 3. When these conditions are
        ///   true, it will generate a new .KV6 sprite with half the resolution in
        ///   all 3 dimensions. It will set kv6->lowermip so it points to the newly
        ///   generated .KV6 object. You can use freekv6() to de-allocate all levels
        ///   of the .KV6 object. To generate all mip levels use this pseudo-code:
        ///      for(kv6data *tempkv6=mykv6;tempkv6=genmipkv6(tempkv6););
        /// kv6: pointer to current MIP-level
        /// returns: pointer to newly generated half-size MIP-level
        pub fn genmipkv6 (kv6: &kv6data) -> *const kv6data;

        /// Returns a pointer to the filename associated with the kv6data/kfatype
        ///   object. Notice that each structure has a "namoff" member. Since I
        ///   use remalloc(), I have to make these offsets, not pointers. Use this
        ///   function to convert the offsets into pointers.
        /// namoff: offset to the name
        pub fn getkfilname (namoff: c_long) -> *const c_char;

        /// You could animate .KFA sprites by simply modifying the .kfatim member of
        ///   vx5sprite structure. A better way is to use this function because it
        ///   will handle repeat/stop markers for you.
        ///    spr: .KFA sprite to animate
        /// timeadd: number of milliseconds to add to the current animation time
        pub fn animsprite (spr: &vx5sprite, timeadd: c_ulong);

        /// Draw a .KV6/.KFA voxel sprite to the screen. Position & orientation are
        ///  specified in the vx5sprite structure. See VOXLAP5.H for details on the
        ///  structure.
        pub fn drawsprite (spr: *const vx5sprite);

        /// This converts a spherical cut-out of the VXL map into a .KV6 sprite in
        ///   memory. This function can be used to make walls fall over (with full
        ///   rotation). It allocates a new vx5sprite sprite structure and you are
        ///   responsible for freeing the memory using "free" in your own code.
        ///   spr: new vx5sprite structure. Position & orientation are initialized
        ///           so when you call drawsprite, it exactly matches the VXL map.
        ///   hit: center of sphere
        /// hitrad: radius of sphere
        /// returns: 0:bad, >0:mass of captured object (# of voxels)
        pub fn meltsphere (spr: &mut vx5sprite, hit: &lpoint3d, hitrad: c_ulong) -> c_ulong;

        /// This function is similar to meltsphere, except you can use any user-
        ///   defined shape (with some size limits). The user-defined shape is
        ///   described by a list of vertical columns in the "vspans" format:
        ///      typedef struct { char z1, z0, x, y; } vspans;
        ///   The list MUST be ordered first in increasing Y, then in increasing X
        ///   or else the function will crash! Fortunately, the structure is
        ///   arranged in a way that the data can be sorted quite easily using a
        ///   simple trick: if you use a typecast from vspans to "unsigned long",
        ///   you can use a generic sort code on 32-bit integers to achieve a
        ///   correct sort. The vspans members are all treated as unsigned chars,
        ///   so it's usually a good idea to bias your columns by 128, and then
        ///   reverse-bias them in the "offs" offset.
        ///
        ///   spr: new vx5sprite structure. Position & orientation are initialized
        ///           so when you call drawsprite, it exactly matches the VXL map.
        ///   lst: list in "vspans" format
        /// lstnum: number of columns on list
        ///  offs: offset of top-left corner in VXL coordinates
        /// returns: mass (in voxel units), returns 0 if error (or no voxels)
        pub fn meltspans (spr: &vx5sprite, lst: *const vspans, lstnum: c_long, offs: &lpoint3d) -> c_long;
        /// -------------------------  Physics helper functions: -------------------------
        /// Math helper: The vectors are refreshed to be perpendicular to each other
        ///   and have unit length. v0 does not change orientation.
        pub fn orthonormalize (v0: &point3d, v1: &point3d, v2: &point3d);

        /// Math helper: same as orthonormalize but for doubles
        pub fn dorthonormalize (v0: &dpoint3d, v1: &dpoint3d, v2: &dpoint3d);

        /// Math helper: rotates 3 vectors using 3 Euclidian rotation
        /// ox: angle #1 (yaw)
        /// oy: angle #2 (up/down)
        /// oz: angle #3 (left/right)
        /// ist: input&output: vector #1 to rotate
        /// ihe: input&output: vector #2 to rotate
        /// ifo: input&output: vector #3 to rotate
        pub fn orthorotate (ox: c_float, oy: c_float, oz: c_float,
            ist: &point3d, ihe: &point3d, ifo: &point3d);

        /// Math helper: same as orthorotate but for doubles
        pub fn dorthorotate (ox: c_double, oy: c_double, oz: c_double,
         ist: &dpoint3d, ihe: &dpoint3d, ifo: &dpoint3d);

        pub fn axisrotate(p: &mut point3d, axis: &point3d, w: c_float);

        /// Math helper: Spherical Linear intERPolation. Quaternions not necessary :)
        ///   Given two 3*3 orthonormal orientation matrices, this finds a 3rd
        ///      matrix that is a smooth interpolation (shortest path) between them.
        ///   istr,ihei,ifor:  first 3*3 rotation matrix (right,down,forward vector)
        /// istr2,ihei2,ifor2: second 3*3 rotation matrix (right,down,forward vector)
        ///      ist,hei,ifo: output 3*3 rotation matrix (right,down,forward vector)
        ///              rat: ratio between first & second matrices to interpolate
        ///                   0 means ist=istr, etc..., 1 means ist=istr2, etc...
        pub fn slerp (istr: &point3d, ihei: &point3d, ifor: &point3d,
            istr2: &point3d, ihei2: &point3d, ifor2: &point3d,
            ist: &point3d, ihe: &point3d, ifo: &point3d, rat: &point3d);

        /// Detect if 2 points have a direct line-of-sight
        /// p0: starting point
        /// p1: ending point
        /// hit: integer VXL coordinate (closest to p0) that caused the collision
        /// returns: 1:didn't hit anything: c_float, 0:something is in the way
        pub fn cansee (p0: &point3d, p1: &point3d, hit: &mut lpoint3d) -> c_long;

        /// Shoot a vector until it hits something or escapes the board
        ///  p: start position
        ///  d: direction
        ///  h: coordinate of voxel hit (if any)
        /// ind: pointer to surface voxel's 32-bit color (0 if none hit)
        /// dir: 0-5: face of cube that was hit (-1 if inside solid)
        /// WARNING: 'h' and 'dir' are written only if a voxel is hit (remember it's
        ///   possible to shoot a ray into the sky!). To see if a voxel is hit, test
        ///   whether 'ind' is nonzero
        pub fn hitscan (p: &dpoint3d, d: &dpoint3d, h: &lpoint3d, ind: *mut*mut c_long, dir: *mut c_long);

        /// Similar to hitscan but for sprites. With this, you can determine exactly
        /// which voxel on a specified .KV6 sprite is hit. This is useful for .KV6
        /// selection in Voxed, or for weapons that require extreme accuracy.
        ///  p: start position
        ///  d: direction
        /// spr: pointer of sprite to test collision with
        ///  h: coordinate of voxel hit in sprite coordinates (if any)
        /// ind: pointer to voxel hit (kv6voxtype) (0 if none hit)
        /// vsc:  input: max multiple/fraction of v0's length to scan (1.0 for |v0|)
        ///     output: multiple/fraction of v0's length of hit point
        pub fn sprhitscan (p: &dpoint3d, d: &dpoint3d, spr: &vx5sprite, h: &lpoint3d,
          ind: &kv6voxtype, vsc: *mut c_float);

        /// Squish detection function: returns the radius of the biggest sphere that
        ///   can fit purely in air around the given point. Basically: c_float, it tells you
        ///   how big a "balloon" can get before it pops
        /// px,py,pz: VXL map coordinate to test
        /// cr: maximum radius to check
        pub fn findmaxcr (px: c_double, py: c_double, pz: c_double, cr: c_double) -> c_double;

        // Fancy collision detection function for spheres - does smooth sliding.
        // p: input/output: starting/ending position of object
        // v: vector to move sphere (specifies both direction&length)
        // acr: radius of sphere
        pub fn clipmove(p: &mut dpoint3d, v: &dpoint3d, acr: c_double);

        /// Special collision detection function (useful for rope) This function does
        ///   collision detection sort of like a windshield wiper, but aa
        ///   triangle instead of a circular arc. There is no thickness.
        ///    p0: joint/axis point
        ///    p1: vertex of start position
        ///    p2: vertex of goal position
        ///   hit: point along p1-p2 line where the "wiper" collided
        ///  lhit: VXL map location that caused the collision
        /// returns: 1:collision: c_long, 0:no collision
        pub fn triscan (p0: &point3d, p1: &point3d, p2: &point3d,
          hit: &point3d, lhit: &lpoint3d) -> c_long;

        /// Estimate normal vector direction. Useful for lighting / bouncing
        /// x,y,z: VXL map coordinate
        /// fp: estimated vector to be returned (magnitude always 1)
        pub fn estnorm (x: c_long, y: c_long, z: c_long, fp: &point3d);

        /// --------------------------- VXL reading functions: ---------------------------

        /// Returns 0 if voxel(x,y,z) is air, or 1 if it is solid
        pub fn isvoxelsolid (x: c_long, y: c_long, z: c_long) -> c_long;

        /// Returns 1 if any voxels in range (x,y,z0) to (x,y,z1-1) are solid, else 0
        pub fn anyvoxelsolid (x: c_long, y: c_long, z0: c_long, z1: c_long) -> c_long;

        /// Returns 1 if any voxels in range (x,y,z0) to (x,y,z1-1) are empty, else 0
        pub fn anyvoxelempty (x: c_long, y: c_long, z0: c_long, z1: c_long) -> c_long;

        /// Returns z of first solid voxel under (x,y,z). Returns z if in solid.
        pub fn getfloorz (x: c_long, y: c_long, z: c_long) -> c_long;

        /// Returns:
        ///   0: air
        ///   1: unexposed solid
        /// else: address to color in vbuf (this can never be 0 or 1)
        pub fn getcube (x: c_long, y: c_long, z: c_long) -> c_long;

        /// --------------------------- VXL writing functions: ---------------------------

        pub fn setcube(px: c_long, px: c_long, px: c_long, col: c_long);
        pub fn setsphere(center: &lpoint3d, hitrad: c_ulong, dacol: c_long);
        /// Render an ellipsoid to VXL memory (code is optimized!)
        ///   hit: focus #1
        ///  hit2: focus #2
        /// hitrad: radius of ellipsoid (length of minor axis/2)
        /// dacol:  0: insert (additive CSG)
        ///        -1: remove (subtractive CSG)
        /// bakit: 0:fast&permanent change, 1:backup (much slower: used in VOXED)
        pub fn setellipsoid (hit: &lpoint3d, hit2: &lpoint3d,
         hitrad: c_long, dacol: c_long, bakit: c_long);

        /// Render a cylinder to VXL memory (code is optimized!)
        ///    p0: endpoint #1
        ///    p1: endpoint #2
        ///    cr: radius of cylinder
        /// dacol:  0: insert (additive CSG)
        ///        -1: remove (subtractive CSG)
        /// bakit: 0:fast&permanent change, 1:backup (much slower: used in VOXED)
        pub fn setcylinder (p0: &lpoint3d, p1: &lpoint3d, cr: c_long,
            dacol: c_long, bakit: c_long);

        /// Render a box to VXL memory (code is optimized!)
        ///   hit: box corner #1
        ///  hit2: box corner #2
        /// dacol:  0: insert (additive CSG)
        ///        -1: remove (subtractive CSG)
        pub fn setrect (hit: &lpoint3d, hit2: &lpoint3d, dacol: c_long);

        /// Render a filled triangle to VXL memory (code is optimized!)
        ///    p0: triangle vertex #1
        ///    p1: triangle vertex #2
        ///    p2: triangle vertex #3
        /// bakit: 0:fast&permanent change, 1:backup (much slower: used in VOXED)
        pub fn settri (p0: &point3d, p1: &point3d, p2: &point3d, bakit: c_long);

        /// Render a complex polygon with TRANSLATIONAL SWEEP to VXL memory
        ///    (code is optimized!)
        ///     p: pointer to list of 3D coplanar vertices that make up polygon
        /// point2: pointer to list of indexes that describe the connectivity. Each
        ///           vertex "connects" to 1 vertex on its right. Holes supported.
        ///           For example: point2[4] = {1,2,3,0} might describe a square
        ///     n: numbers of vertices in polygon (limited by MAXCURS)
        /// thick: thickness of polygon (amount of translational sweep)
        /// dacol:  0: insert (additive CSG)
        ///        -1: remove (subtractive CSG)
        /// bakit: 0:fast&permanent change, 1:backup (much slower: used in VOXED)
        pub fn setsector (p: &point3d, point2: *const c_long, n: c_long,
         thick: c_float, dacol: c_long, bakit: c_long);

        /// Do CSG using pre-sorted spanlist.
        ///   lst: Spans (see meltspans() for structure description)
        /// lstnum: Number of entries in lst.
        ///  offs: offset in VXL map to apply CSG. This point is origin in vspans.
        /// dacol:  0: insert (additive CSG)
        ///        -1: remove (subtractive CSG)
        pub fn setspans (lst: &vspans, lstnum: c_long, offs: &lpoint3d, dacol: c_long);

        /// Apply additive CSG using a 2D heightmap (connected to floor) as source.
        /// where: hpic: pointer to top-left corner of heightmap
        ///       hbpl: pitch (bytes per line) of heightmap
        ///    hxs,hys: dimensions of heightmap
        /// x0,y0,x1,y1: 2D box in VXL coordinates to apply additive heightmap CSG.
        pub fn setheightmap (hptr: *const c_char, hbpl: c_long, hxs: c_long, hys: c_long,
         x0: c_long, y0: c_long, x1: c_long, y1: c_long);

        /// Render .KV6 voxel sprite to VXL memory. Instead of drawing the sprite
        ///   to the screen, this renders it permanently to VXL memory. This can
        ///   be used for many effects, such as piling up "infinite" dead bodies.
        ///   spr: sprite to "freeze" to the VXL map
        /// dacol:  0: insert (additive CSG)
        ///        -1: remove (subtractive CSG)
        pub fn setkv6 (spr: &vx5sprite, dacol: c_long);

        /// Render 3D convex hull to VXL memory (code is optimized!)
        ///    pt: pointer to list of points formatted as point3d
        ///  nump: number of points (note: limited by MAXPOINTS)
        /// dacol:  0: insert (additive CSG)
        ///        -1: remove (subtractive CSG)
        /// bakit: 0:fast&permanent change, 1:backup (much slower: used in VOXED)
        /// #WARNING
        ///  There is a lock-up bug if there are any planes are co-planar.
        ///  This can happen at the edges of the map even if you don't pass it
        ///  coplanar planes. You might want to avoid this function until I fix it!
        pub fn sethull3d (pt: &point3d, nump: c_long, dacol: c_long, bakit: c_long);

        /// Render a polygon with ROTATIONAL SWEEP to VXL memory. The first 2
        ///   vertices define the axis of rotation. (WARNING: code NOT optimized)
        ///     p: pointer to list of 3D coplanar vertices that make up polygon
        /// numcurs: number of vertices in polygon
        /// dacol:  0: insert (additive CSG)
        ///        -1: remove (subtractive CSG)
        /// bakit: 0:fast&permanent change, 1:backup (much slower: used in VOXED)
        pub fn setlathe (p: &point3d, numcurs: c_long, dacol: c_long, bakit: c_long);

        /// Render "metaballs" to VXL memory. (WARNING: code NOT optimized)
        ///     p: pointer to list of 3D points that make up the sources
        /// numcurs: number of sources
        /// dacol:  0: insert (additive CSG)
        ///        -1: remove (subtractive CSG)
        /// bakit: 0:fast&permanent change, 1:backup (much slower: used in VOXED)
        /// NOTE: uses vx5.currad as "threshold" for metaballs cutoff value
        pub fn setblobs (p: &point3d, numcurs: c_long, dacol: c_long, bakit: c_long);

        /// Conducts on air and writes solid.
        /// x,y,z: starting point
        /// minx,miny,minz: top/left/up corner of box used to restrict floodfill
        ///                (inclusive)
        /// maxx,maxy,maxz: bot/right/down corner of box used to restrict floodfill
        ///                (exclusive)
        pub fn setfloodfill3d (x: c_long, y: c_long, z: c_long, minx: c_long, miny: c_long, minz: c_long,
            maxx: c_long, maxy: c_long, maxz: c_long);

        /// Fill in all hollow areas of map - mainly used in editor. Very slow! This
        /// will destroy any hidden "bonus" areas in your map.
        pub fn sethollowfill ();

        /// Render VOX/KVX voxel sprite to VXL memory. (WARNING: code NOT optimized)
        /// filename: VOX/KVX file
        /// ox,oy,oz: VXL map location to render the object
        ///     rot: 0-47 possible rotation, all are axis-aligned
        /// bakit: 0:fast&permanent change, 1:backup (much slower: used in VOXED)
        pub fn setkvx (filename: *const c_char, ox: c_long, oy: c_long, oz: c_long,
         rot: c_long, bakit: c_long);

        /// Old lighting function (has aliasing artifacts)
        /// px,py,pz: origin of light source
        /// flashradius: maximum radius to scan out (recommended values: 128-253)
        /// numang: angle density (recommended values: 512,1024,2048)
        /// intens: intensity scale (recommended values: 1,2)
        pub fn setflash (px: c_float, py: c_float, pz: c_float,
            flashradius: c_long, numang: c_long, intens: c_long);

        pub fn setnormflash(px: c_float, px: c_float, px: c_float, flash_radius: c_long, intens: c_long);

        /// ---------------------------- VXL MISC functions:  ----------------------------

        pub fn updatebbox (x0: c_long, y0: c_long, z0: c_long, x1: c_long, y1: c_long, z1: c_long,
          csgdel: c_long);
        pub fn updatevxl();
        pub fn genmipvxl (x0: c_long, y0: c_long, x1: c_long, y1: c_long);
        pub fn updatelighting (x0: c_long, y0: c_long, z0: c_long, x1: c_long, y1: c_long, z1: c_long);

        /// custom
        pub fn getVSID() -> c_long;
        pub fn setMaxScanDistToMax();
        pub fn setMaxScanDist(dist: c_long);
        pub fn setLightingMode(mode: c_long);

        pub fn set_anginc(anginc: c_long);
        pub fn get_anginc() -> c_long;

        pub fn set_fogcol(fogcol: c_long);
        pub fn set_kv6col(kv6col: c_long);
        pub fn set_curcol(curcol: c_long);
        pub fn set_curpow(curpow: c_float);
        pub fn set_fallcheck(fallcheck: c_long);

        /// ------------------------- Falling voxels functions: --------------------------
        /// NOTE: THIS FUNCTION IS OBSOLETE!
        /// It has been replaced with updatevxl() (remember to set vx5.fallcheck=1;)
        /// Old documentation was:
        ///   (Call it after every set* call that removes voxels (subtractive CSG)
        ///   It remembers the location on an internal "check" list that will
        ///   be used in the following call to dofalls())
pub fn checkfloatinbox (x0: c_long, y0: c_long, z0: c_long, x1: c_long, y1: c_long, z1: c_long);

        /// Call this once per frame (or perhaps at a slower constant rate 20hz-40hz)
        pub fn startfalls ();

        /// NOTE: THIS FUNCTION IS OBSOLETE! It still works, but it is much better to
        ///   use the meltfall() function. With dofall(), pieces fall straight down
        ///   in the VXL map (without any kind of support for rotation).
        /// Old documentation was:
        ///   (Call this only between a call to startfalls() and
        ///   finishfalls(). You MUST call it either 0 or 1 times between each
        ///   startfalls and finishfalls. (See sample code in GAME.C))
pub fn dofall (i: c_long);

        /// Works sort of like meltsphere(), but works with floating sections of the
        ///   .VXL map instead of spheres. This function can be used to make
        ///   floating pieces fall over (with full rotation). It allocates a new
        ///   vx5sprite sprite structure and you are responsible for freeing the
        ///   memory using "free" in your own code.
        ///   NOTE: this MUST be called between startfalls() and finishfalls() and
        ///      you MUST NOT call dofall() if this function succeeds!
        ///   spr: new vx5sprite structure. Position & orientation are initialized
        ///        so when you call drawsprite, it exactly matches the VXL map.
        ///     i: index to falling object (same param passed to dofall())
        /// delvxl: 0:don't change .VXL map, 1:delete .VXL from map
        /// returns: 0:failed, >0:mass of captured object (# of voxels)
        pub fn meltfall (spr: &vx5sprite, i: c_long, delvxl: c_long) -> c_long;

        /// Call this once for each startfalls()
        pub fn finishfalls ();
        /// ----------------------- Procedural texture functions: ------------------------
        pub fn curcolfunc (p: &lpoint3d) -> c_long;

        /// returns color of nearest voxel below the specified point: (x,y,>=z)
        pub fn floorcolfunc (p: &lpoint3d) -> c_long;

        /// returns vx5.curcol with RGB randomly jittered; scaled by vx5.amount
        pub fn jitcolfunc (p: &lpoint3d) -> c_long;

        /// colorful sin waves: Red=x, Green=y, Blue=z
        pub fn manycolfunc (p: &lpoint3d) -> c_long;

        /// directional shading. uses vx5.cen as vector center and vx5.daf as scale
        pub fn sphcolfunc (p: &lpoint3d) -> c_long;

        /// wood, color can be selected with vx5.curcol
        pub fn woodcolfunc (p: &lpoint3d) -> c_long;

        /// use a 2D texture defined by frame: vx5.pic, vx5.bpl, vx5.xsiz, vx5.ysiz
        /// vx5.picmode =
        ///  0: aligned-axis mapping, uses vx5.pico, vx5.picu/v, vx5.xoru/v
        ///  1: cylindrical mapping, uses vx5.fpico, vx5.fpicu/v/w, vx5.xoru
        ///  2: spherical mapping, uses vx5.fpico, vx5.fpicu/v/w, vx5.xoru
        ///  3: any axis mapping, uses vx5.fpico, vx5.fpicu/v
        /// where:
        ///   vx5.(f)pico is the x,y,z location where: u=0 & v=0
        ///   vx5.(f)picu/v/w are directions vectors that specify how u & v map
        ///   vx5.xoru/v is used to mirror coordinates in picmode 0
        pub fn pngcolfunc (p: &lpoint3d) -> c_long;

        /// Used internally by setkv6(). Do not use for anything else!
        pub fn kv6colfunc (p: &lpoint3d) -> c_long;

        /// ---------------- Picture functions (PNG,JPG,TGA,GIF,PCX,BMP): ----------------

        /// Easy picture loading function. This does most of the background work for
        ///   you. It allocates the buffer for the uncompressed image for you, and
        ///   loads PNG,JPG,TGA,GIF,PCX,BMP files, even handling pictures inside ZIP
        ///   files. Kpzload() always writes 32-bit ARGB format (even if source is
        ///   less).
        ///   filnam: name of the graphic file (can be inside ZIP file).
        ///      pic: pointer to top-left corner of destination uncompressed image
        ///      bpl: pitch (bytes per line) of destination uncompressed image
        /// xsiz,ysiz: dimensions of destination image
        /// NOTE: You are responsible for calling free() on the returned pointer
        pub fn kpzload (filnam: *const c_char, pic: *mut c_long, bpl: *mut c_ulong,
          xsiz: *mut c_ulong, ysiz: *mut c_ulong);

        /// This retrieves the dimensions of a compressed graphic file image loaded
        ///   into memory. It supports the same file types as kpzload().
        ///     buf: pointer to file image in memory
        ///    leng: length of file (and file image)
        pub fn kpgetdim (buf: *const c_char, leng: c_long, xsiz: *const c_long, ysiz: *const c_long);

        /// This decompresses the compressed file image from memory to memory.
        ///   Kprender always writes 32-bit ARGB format (even if source is less).
        ///      buf: pointer to file image in memory
        ///     leng: length of file (and file image)
        /// frameptr: pointer to top-left corner of destination uncompressed image
        ///      bpl: pitch (bytes per line) of destination uncompressed image
        /// xdim,ydim: dimensions of destination image
        /// xoff,yoff: (x,y) offset into the destination image to store the tile.
        ///           Non-zero values are useful here for picture viewer programs.
        /// returns: -1:bad, 0:good
        pub fn kprender (buf: *const c_char, leng: c_long, frameptr: c_long, bpl: c_long,
            xdim: c_long, ydim: c_long, xoff: c_long, yoff: c_long) -> c_long;
            /// ------------------------------- ZIP functions: -------------------------------
            /// These functions are all optional. If you want to distribute the game
        ///   without cluttering up people's hard drives with tons of small files,
        ///   then you should really take a look at these functions.
        ///
        /// Except for kzaddstack(), these functions work very similar to the
        ///   low-level file functions from the standard C library. I did this so it
        ///   would be easy to convert standard file code to support my .ZIP
        ///   library. One thing you should note about my "kz" library is that it
        ///   doesn't support multiple file handles. Because of this, there is no
        ///   reason to maintain file handles - so I omitted that parameter from my
        ///   functions.

        /// This adds a .ZIP file to the internal list of .ZIP files to check. Every
        ///   time you open a file using kzopen(), it will check to see if the file
        ///   is located inside this ZIP file. Priority is given to the most recent
        ///   call to kzaddstack(), so you should always call kzaddstack() with your
        ///   big game data file first, and call it with any user patches later.
        pub fn kzaddstack (filnam: *const c_char);

        /// This clears all ZIP files from the kz stack. You would use this if for
        ///   some reason you need to re-load the user patches in the game
        pub fn kzuninit ();

        /// Similar to open/fopen. Kzopen file priority:
        ///   1. Search local dirs for stand-alone files
        ///   2. Search .ZIP filenames passed to kzaddstack (last one first)
        ///   3. return(0); (File not found)
        /// Always uses binary mode.
        /// returns: 0:bad/file not found, !=0:good (long)(FILE *fil)
        pub fn kzopen (filnam: *const c_char) -> c_long;

        /// Similar to read/fread: Returns number of bytes copied
        pub fn kzread (buffer: *mut c_void, leng: c_long) -> c_long;

        /// Similar to filelength: Returns file length
        pub fn kzfilelength () -> c_long;

        /// Similar to seek/fseek; whence can be: SEEK_SET, SEEK_CUR, or SEEK_END
        /// NOTE: try to avoid using kzseek(#,SEEK_CUR) where # is < -32768. For
        ///   compressed files, this is very slow, because KZLIB must decompress
        ///   the whole file up to that point, starting from the beginning.
        pub fn kzseek (offset: c_long, whence: c_long);

        /// Similar to tell/ftell: Returns file position (offset from beginning)
        pub fn kztell () -> c_long;

        /// Similar to fgetc: Reads 1 byte and returns the byte value
        ///  If file pointer is at the end of the file, it returns -1
        pub fn kzgetc () -> c_long;

        /// Similar to eof/feof: Returns 1 if at end of file, otherwise 0
        pub fn kzeof () -> c_long;

        /// Similar to close/fclose
        pub fn kzclose ();

        /// The following 2 functions are cover-up functions for FindFirstFile/
        ///   FindNextFile. In addition to finding stand-afiles: c_long, they also look
        ///   for files in any .ZIP files that have been specified by kzaddstack().
        ///   It supports full ? and * wildcards for both stand-alone files and
        ///   files inside .ZIP files. Unlike FindFirstFile, kzfindfilestart() does
        ///   not return a filename. This can make your loops simpler because you
        ///   only need to use a single function (kzfindfile) to retrieve all the
        ///   filenames. Here's a simple example showing how to use these functions:
        ///      char filnam[MAX_PATH];
        ///      kzfindfilestart("vxl/*.vxl");
        ///      while (kzfindfile(filnam)) puts(filnam);
        ///
        ///   NOTES:
        ///    * Directory names begin with '\'
        ///    * Files inside zip begin with '|'

        /// First, pass a file specification string (wildcards supported)
        pub fn kzfindfilestart (st: *const c_char);

        /// You must alloc buffer yourself (MAX_PATH characters)
        /// returns: 1 if file found, filnam written, continue processing
        ///         0 if no files left
        pub fn kzfindfile (filnam: *const c_char) -> c_long;
        pub fn vox_free(ptr: *const c_void);
    }
