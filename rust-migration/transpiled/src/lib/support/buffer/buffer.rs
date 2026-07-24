extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
pub type __builtin_va_list = __va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list {
    pub __stack: *mut ::core::ffi::c_void,
    pub __gr_top: *mut ::core::ffi::c_void,
    pub __vr_top: *mut ::core::ffi::c_void,
    pub __gr_offs: ::core::ffi::c_int,
    pub __vr_offs: ::core::ffi::c_int,
}
pub type size_t = usize;
pub type __gnuc_va_list = __builtin_va_list;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
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
pub type va_list = __gnuc_va_list;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type sds = *mut ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr8 {
    pub len: uint8_t,
    pub alloc: uint8_t,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr16 {
    pub len: uint16_t,
    pub alloc: uint16_t,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr32 {
    pub len: uint32_t,
    pub alloc: uint32_t,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr64 {
    pub len: uint64_t,
    pub alloc: uint64_t,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct caryll_Buffer {
    pub cursor: size_t,
    pub size: size_t,
    pub free: size_t,
    pub data: *mut uint8_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SDS_TYPE_5: ::core::ffi::c_int = 0;
pub const SDS_TYPE_8: ::core::ffi::c_int = 1;
pub const SDS_TYPE_16: ::core::ffi::c_int = 2;
pub const SDS_TYPE_32: ::core::ffi::c_int = 3;
pub const SDS_TYPE_64: ::core::ffi::c_int = 4;
pub const SDS_TYPE_MASK: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SDS_TYPE_BITS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn sdslen(s: sds) -> size_t {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as size_t,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr8>() as usize as isize))
                as *mut sdshdr8))
                .len as size_t;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr16>() as usize as isize))
                as *mut sdshdr16))
                .len as size_t;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr32>() as usize as isize))
                as *mut sdshdr32))
                .len as size_t;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr64>() as usize as isize))
                as *mut sdshdr64))
                .len as size_t;
        }
        _ => {}
    }
    return 0 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn bufnew() -> *mut caryll_Buffer {
    let mut buf: *mut caryll_Buffer = ::core::ptr::null_mut::<caryll_Buffer>();
    buf = __caryll_allocate_clean(
        ::core::mem::size_of::<caryll_Buffer>() as size_t,
        6 as ::core::ffi::c_ulong,
    ) as *mut caryll_Buffer;
    (*buf).free = 0 as size_t;
    (*buf).size = (*buf).free;
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn buffree(mut buf: *mut caryll_Buffer) {
    if buf.is_null() {
        return;
    }
    if !(*buf).data.is_null() {
        free((*buf).data as *mut ::core::ffi::c_void);
        (*buf).data = ::core::ptr::null_mut::<uint8_t>();
    }
    free(buf as *mut ::core::ffi::c_void);
    buf = ::core::ptr::null_mut::<caryll_Buffer>();
}
#[no_mangle]
pub unsafe extern "C" fn buflen(mut buf: *mut caryll_Buffer) -> size_t {
    return (*buf).size;
}
#[no_mangle]
pub unsafe extern "C" fn bufpos(mut buf: *mut caryll_Buffer) -> size_t {
    return (*buf).cursor;
}
#[no_mangle]
pub unsafe extern "C" fn bufseek(mut buf: *mut caryll_Buffer, mut pos: size_t) {
    (*buf).cursor = pos;
}
#[no_mangle]
pub unsafe extern "C" fn bufclear(mut buf: *mut caryll_Buffer) {
    (*buf).cursor = 0 as size_t;
    (*buf).free = (*buf).size.wrapping_add((*buf).free);
    (*buf).size = 0 as size_t;
}
unsafe extern "C" fn bufbeforewrite(mut buf: *mut caryll_Buffer, mut towrite: size_t) {
    let mut currentSize: size_t = (*buf).size;
    let mut allocated: size_t = (*buf).size.wrapping_add((*buf).free);
    let mut required: size_t = (*buf).cursor.wrapping_add(towrite);
    if required < currentSize {
        return;
    } else if required <= allocated {
        (*buf).size = required;
        (*buf).free = allocated.wrapping_sub((*buf).size);
    } else {
        (*buf).size = required;
        (*buf).free = required;
        if (*buf).free > 0x1000000 as ::core::ffi::c_int as size_t {
            (*buf).free = 0x1000000 as ::core::ffi::c_int as size_t;
        }
        (*buf).data = __caryll_reallocate(
            (*buf).data as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<uint8_t>() as size_t)
                .wrapping_mul((*buf).size.wrapping_add((*buf).free)),
            46 as ::core::ffi::c_ulong,
        ) as *mut uint8_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn bufwrite8(mut buf: *mut caryll_Buffer, mut byte: uint8_t) {
    bufbeforewrite(buf, 1 as size_t);
    let fresh0 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh0 as isize) = byte;
}
#[no_mangle]
pub unsafe extern "C" fn bufwrite16l(mut buf: *mut caryll_Buffer, mut x: uint16_t) {
    bufbeforewrite(buf, 2 as size_t);
    let fresh1 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh1 as isize) =
        (x as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
    let fresh2 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh2 as isize) = (x as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
        & 0xff as ::core::ffi::c_int) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn bufwrite16b(mut buf: *mut caryll_Buffer, mut x: uint16_t) {
    bufbeforewrite(buf, 2 as size_t);
    let fresh3 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh3 as isize) = (x as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
        & 0xff as ::core::ffi::c_int) as uint8_t;
    let fresh4 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh4 as isize) =
        (x as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn bufwrite24l(mut buf: *mut caryll_Buffer, mut x: uint32_t) {
    bufbeforewrite(buf, 3 as size_t);
    let fresh5 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh5 as isize) = (x & 0xff as uint32_t) as uint8_t;
    let fresh6 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh6 as isize) =
        (x >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
    let fresh7 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh7 as isize) =
        (x >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn bufwrite24b(mut buf: *mut caryll_Buffer, mut x: uint32_t) {
    bufbeforewrite(buf, 3 as size_t);
    let fresh8 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh8 as isize) =
        (x >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
    let fresh9 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh9 as isize) =
        (x >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
    let fresh10 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh10 as isize) = (x & 0xff as uint32_t) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn bufwrite32l(mut buf: *mut caryll_Buffer, mut x: uint32_t) {
    bufbeforewrite(buf, 4 as size_t);
    let fresh11 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh11 as isize) = (x & 0xff as uint32_t) as uint8_t;
    let fresh12 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh12 as isize) =
        (x >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
    let fresh13 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh13 as isize) =
        (x >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
    let fresh14 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh14 as isize) =
        (x >> 24 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn bufwrite32b(mut buf: *mut caryll_Buffer, mut x: uint32_t) {
    bufbeforewrite(buf, 4 as size_t);
    let fresh15 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh15 as isize) =
        (x >> 24 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
    let fresh16 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh16 as isize) =
        (x >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
    let fresh17 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh17 as isize) =
        (x >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
    let fresh18 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh18 as isize) = (x & 0xff as uint32_t) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn bufwrite64l(mut buf: *mut caryll_Buffer, mut x: uint64_t) {
    bufbeforewrite(buf, 8 as size_t);
    let fresh19 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh19 as isize) = (x & 0xff as uint64_t) as uint8_t;
    let fresh20 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh20 as isize) =
        (x >> 8 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    let fresh21 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh21 as isize) =
        (x >> 16 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    let fresh22 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh22 as isize) =
        (x >> 24 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    let fresh23 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh23 as isize) =
        (x >> 32 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    let fresh24 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh24 as isize) =
        (x >> 40 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    let fresh25 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh25 as isize) =
        (x >> 48 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    let fresh26 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh26 as isize) =
        (x >> 56 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn bufwrite64b(mut buf: *mut caryll_Buffer, mut x: uint64_t) {
    bufbeforewrite(buf, 8 as size_t);
    let fresh27 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh27 as isize) =
        (x >> 56 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    let fresh28 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh28 as isize) =
        (x >> 48 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    let fresh29 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh29 as isize) =
        (x >> 40 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    let fresh30 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh30 as isize) =
        (x >> 32 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    let fresh31 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh31 as isize) =
        (x >> 24 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    let fresh32 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh32 as isize) =
        (x >> 16 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    let fresh33 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh33 as isize) =
        (x >> 8 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    let fresh34 = (*buf).cursor;
    (*buf).cursor = (*buf).cursor.wrapping_add(1);
    *(*buf).data.offset(fresh34 as isize) = (x & 0xff as uint64_t) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn bufninit(mut n: uint32_t, mut args: ...) -> *mut caryll_Buffer {
    let mut buf: *mut caryll_Buffer = bufnew();
    bufbeforewrite(buf, n as size_t);
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    let mut j: uint16_t = 0 as uint16_t;
    while (j as uint32_t) < n {
        bufwrite8(buf, ap.arg::<::core::ffi::c_int>() as uint8_t);
        j = j.wrapping_add(1);
    }
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn bufnwrite8(mut buf: *mut caryll_Buffer, mut n: uint32_t, mut args: ...) {
    bufbeforewrite(buf, n as size_t);
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    let mut j: uint16_t = 0 as uint16_t;
    while (j as uint32_t) < n {
        bufwrite8(buf, ap.arg::<::core::ffi::c_int>() as uint8_t);
        j = j.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn bufwrite_sds(mut buf: *mut caryll_Buffer, mut str: sds) {
    if str.is_null() {
        return;
    }
    let mut len: size_t = sdslen(str);
    if len == 0 {
        return;
    }
    bufbeforewrite(buf, len);
    memcpy(
        (*buf).data.offset((*buf).cursor as isize) as *mut ::core::ffi::c_void,
        str as *const ::core::ffi::c_void,
        len,
    );
    (*buf).cursor = (*buf).cursor.wrapping_add(len);
}
#[no_mangle]
pub unsafe extern "C" fn bufwrite_str(
    mut buf: *mut caryll_Buffer,
    mut str: *const ::core::ffi::c_char,
) {
    if str.is_null() {
        return;
    }
    let mut len: size_t = strlen(str);
    if len == 0 {
        return;
    }
    bufbeforewrite(buf, len);
    memcpy(
        (*buf).data.offset((*buf).cursor as isize) as *mut ::core::ffi::c_void,
        str as *const ::core::ffi::c_void,
        len,
    );
    (*buf).cursor = (*buf).cursor.wrapping_add(len);
}
#[no_mangle]
pub unsafe extern "C" fn bufwrite_bytes(
    mut buf: *mut caryll_Buffer,
    mut len: size_t,
    mut str: *const uint8_t,
) {
    if str.is_null() {
        return;
    }
    if len == 0 {
        return;
    }
    bufbeforewrite(buf, len);
    memcpy(
        (*buf).data.offset((*buf).cursor as isize) as *mut ::core::ffi::c_void,
        str as *const ::core::ffi::c_void,
        len,
    );
    (*buf).cursor = (*buf).cursor.wrapping_add(len);
}
#[no_mangle]
pub unsafe extern "C" fn bufwrite_buf(mut buf: *mut caryll_Buffer, mut that: *mut caryll_Buffer) {
    if that.is_null() || (*that).data.is_null() {
        return;
    }
    let mut len: size_t = buflen(that);
    bufbeforewrite(buf, len);
    memcpy(
        (*buf).data.offset((*buf).cursor as isize) as *mut ::core::ffi::c_void,
        (*that).data as *const ::core::ffi::c_void,
        len,
    );
    (*buf).cursor = (*buf).cursor.wrapping_add(len);
}
#[no_mangle]
pub unsafe extern "C" fn bufwrite_bufdel(
    mut buf: *mut caryll_Buffer,
    mut that: *mut caryll_Buffer,
) {
    if that.is_null() {
        return;
    }
    if (*that).data.is_null() {
        buffree(that);
        return;
    }
    let mut len: size_t = buflen(that);
    bufbeforewrite(buf, len);
    memcpy(
        (*buf).data.offset((*buf).cursor as isize) as *mut ::core::ffi::c_void,
        (*that).data as *const ::core::ffi::c_void,
        len,
    );
    buffree(that);
    (*buf).cursor = (*buf).cursor.wrapping_add(len);
}
#[no_mangle]
pub unsafe extern "C" fn buflongalign(mut buf: *mut caryll_Buffer) {
    let mut cp: size_t = (*buf).cursor;
    bufseek(buf, buflen(buf));
    if buflen(buf).wrapping_rem(4 as size_t) == 1 as size_t {
        bufwrite8(buf, 0 as uint8_t);
        bufwrite8(buf, 0 as uint8_t);
        bufwrite8(buf, 0 as uint8_t);
    } else if buflen(buf).wrapping_rem(4 as size_t) == 2 as size_t {
        bufwrite8(buf, 0 as uint8_t);
        bufwrite8(buf, 0 as uint8_t);
    } else if buflen(buf).wrapping_rem(4 as size_t) == 3 as size_t {
        bufwrite8(buf, 0 as uint8_t);
    }
    bufseek(buf, cp);
}
#[no_mangle]
pub unsafe extern "C" fn bufping16b(
    mut buf: *mut caryll_Buffer,
    mut offset: *mut size_t,
    mut cp: *mut size_t,
) {
    bufwrite16b(buf, *offset as uint16_t);
    *cp = (*buf).cursor;
    bufseek(buf, *offset);
}
#[no_mangle]
pub unsafe extern "C" fn bufping16bd(
    mut buf: *mut caryll_Buffer,
    mut offset: *mut size_t,
    mut shift: *mut size_t,
    mut cp: *mut size_t,
) {
    bufwrite16b(buf, (*offset).wrapping_sub(*shift) as uint16_t);
    *cp = (*buf).cursor;
    bufseek(buf, *offset);
}
#[no_mangle]
pub unsafe extern "C" fn bufpong(
    mut buf: *mut caryll_Buffer,
    mut offset: *mut size_t,
    mut cp: *mut size_t,
) {
    *offset = (*buf).cursor;
    bufseek(buf, *cp);
}
#[no_mangle]
pub unsafe extern "C" fn bufpingpong16b(
    mut buf: *mut caryll_Buffer,
    mut that: *mut caryll_Buffer,
    mut offset: *mut size_t,
    mut cp: *mut size_t,
) {
    bufwrite16b(buf, *offset as uint16_t);
    *cp = (*buf).cursor;
    bufseek(buf, *offset);
    bufwrite_bufdel(buf, that);
    *offset = (*buf).cursor;
    bufseek(buf, *cp);
}
#[no_mangle]
pub unsafe extern "C" fn bufprint(mut buf: *mut caryll_Buffer) {
    let mut j: size_t = 0 as size_t;
    while j < (*buf).size {
        if j.wrapping_rem(16 as size_t) != 0 {
            fprintf(stderr, b" \0" as *const u8 as *const ::core::ffi::c_char);
        }
        fprintf(
            stderr,
            b"%02X\0" as *const u8 as *const ::core::ffi::c_char,
            *(*buf).data.offset(j as isize) as ::core::ffi::c_int,
        );
        if j.wrapping_rem(16 as size_t) == 15 as size_t {
            fprintf(stderr, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
        }
        j = j.wrapping_add(1);
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
}
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
#[inline]
unsafe extern "C" fn __caryll_reallocate(
    mut ptr: *mut ::core::ffi::c_void,
    mut n: size_t,
    mut line: ::core::ffi::c_ulong,
) -> *mut ::core::ffi::c_void {
    if n == 0 {
        free(ptr);
        return NULL;
    }
    if ptr.is_null() {
        return __caryll_allocate_clean(n, line);
    } else {
        let mut p: *mut ::core::ffi::c_void = realloc(ptr, n);
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
    };
}
