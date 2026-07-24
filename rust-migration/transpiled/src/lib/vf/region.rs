extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __uint16_t = u16;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type uint16_t = __uint16_t;
pub type shapeid_t = uint16_t;
pub type pos_t = ::core::ffi::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VV {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut pos_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vq_AxisSpan {
    pub start: pos_t,
    pub peak: pos_t,
    pub end: pos_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vq_Region {
    pub dimensions: shapeid_t,
    pub spans: [vq_AxisSpan; 0],
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn __caryll_allocate_clean(
    mut n: size_t,
    mut line: ::core::ffi::c_ulong,
) -> *mut ::core::ffi::c_void {
    if n == 0 {
        return NULL;
    }
    let mut p: *mut ::core::ffi::c_void = calloc(n, 1 as size_t);
    if p.is_null() {
        fprintf(
            stderr,
            b"[%ld]Out of memory(%ld bytes)\n\0" as *const u8 as *const ::core::ffi::c_char,
            line,
            n as ::core::ffi::c_ulong,
        );
        exit(EXIT_FAILURE);
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn vq_createRegion(mut dimensions: shapeid_t) -> *mut vq_Region {
    let mut r: *mut vq_Region = ::core::ptr::null_mut::<vq_Region>();
    r = __caryll_allocate_clean(
        (::core::mem::size_of::<vq_Region>() as size_t).wrapping_add(
            (::core::mem::size_of::<vq_AxisSpan>() as size_t).wrapping_mul(dimensions as size_t),
        ),
        6 as ::core::ffi::c_ulong,
    ) as *mut vq_Region;
    (*r).dimensions = dimensions;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn vq_deleteRegion(mut region: *mut vq_Region) {
    free(region as *mut ::core::ffi::c_void);
    region = ::core::ptr::null_mut::<vq_Region>();
}
#[no_mangle]
pub unsafe extern "C" fn vq_copyRegion(mut region: *const vq_Region) -> *mut vq_Region {
    let mut dst: *mut vq_Region = vq_createRegion((*region).dimensions);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        region as *const ::core::ffi::c_void,
        (::core::mem::size_of::<vq_Region>() as size_t).wrapping_add(
            (::core::mem::size_of::<vq_AxisSpan>() as size_t)
                .wrapping_mul((*region).dimensions as size_t),
        ),
    );
    return dst;
}
#[no_mangle]
pub unsafe extern "C" fn vq_compareRegion(
    mut a: *const vq_Region,
    mut b: *const vq_Region,
) -> ::core::ffi::c_int {
    if ((*a).dimensions as ::core::ffi::c_int) < (*b).dimensions as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if (*a).dimensions as ::core::ffi::c_int > (*b).dimensions as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    return strncmp(
        a as *const ::core::ffi::c_char,
        b as *const ::core::ffi::c_char,
        (::core::mem::size_of::<vq_Region>() as size_t).wrapping_add(
            (::core::mem::size_of::<vq_AxisSpan>() as size_t)
                .wrapping_mul((*a).dimensions as size_t),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn vq_AxisSpanIsOne(mut s: *const vq_AxisSpan) -> bool {
    let a: pos_t = (*s).start;
    let p: pos_t = (*s).peak;
    let z: pos_t = (*s).end;
    return a > p
        || p > z
        || a < 0 as ::core::ffi::c_int as pos_t
            && z > 0 as ::core::ffi::c_int as pos_t
            && p != 0 as ::core::ffi::c_int as pos_t
        || p == 0 as ::core::ffi::c_int as pos_t;
}
#[inline]
unsafe extern "C" fn weightAxisRegion(mut as_0: *const vq_AxisSpan, x: pos_t) -> pos_t {
    let a: pos_t = (*as_0).start;
    let p: pos_t = (*as_0).peak;
    let z: pos_t = (*as_0).end;
    if a > p || p > z {
        return 1 as ::core::ffi::c_int as pos_t;
    } else if a < 0 as ::core::ffi::c_int as pos_t
        && z > 0 as ::core::ffi::c_int as pos_t
        && p != 0 as ::core::ffi::c_int as pos_t
    {
        return 1 as ::core::ffi::c_int as pos_t;
    } else if p == 0 as ::core::ffi::c_int as pos_t {
        return 1 as ::core::ffi::c_int as pos_t;
    } else if x < a || x > z {
        return 0 as ::core::ffi::c_int as pos_t;
    } else if x == p {
        return 1 as ::core::ffi::c_int as pos_t;
    } else if x < p {
        return (x - a) / (p - a);
    } else {
        return (z - x) / (z - p);
    };
}
#[no_mangle]
pub unsafe extern "C" fn vqRegionGetWeight(mut r: *const vq_Region, mut v: *const VV) -> pos_t {
    let mut w: pos_t = 1 as ::core::ffi::c_int as pos_t;
    let mut j: size_t = 0 as size_t;
    while j < (*r).dimensions as size_t && (*v).length != 0 {
        w *= weightAxisRegion(
            (&raw const (*r).spans as *const vq_AxisSpan).offset(j as isize) as *const vq_AxisSpan,
            *(*v).items.offset(j as isize),
        );
        j = j.wrapping_add(1);
    }
    return w;
}
#[no_mangle]
pub unsafe extern "C" fn vq_showRegion(mut _r: *const vq_Region) {}
