extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn buffree(buf: *mut caryll_Buffer);
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
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type va_list = __builtin_va_list;
pub type size_t = usize;
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
pub struct caryll_Buffer {
    pub cursor: size_t,
    pub size: size_t,
    pub free: size_t,
    pub data: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_bkblock {
    pub _visitstate: bk_cell_visit_state,
    pub _index: uint32_t,
    pub _height: uint32_t,
    pub _depth: uint32_t,
    pub length: uint32_t,
    pub free: uint32_t,
    pub cells: *mut bk_Cell,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bk_Cell {
    pub t: bk_CellType,
    pub c2rust_unnamed: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub z: uint32_t,
    pub p: *mut __caryll_bkblock,
}
pub type bk_CellType = ::core::ffi::c_uint;
pub const bkembed: bk_CellType = 255;
pub const bkcopy: bk_CellType = 254;
pub const sp32: bk_CellType = 129;
pub const sp16: bk_CellType = 128;
pub const p32: bk_CellType = 17;
pub const p16: bk_CellType = 16;
pub const b32: bk_CellType = 3;
pub const b16: bk_CellType = 2;
pub const b8: bk_CellType = 1;
pub const bkover: bk_CellType = 0;
pub type bk_cell_visit_state = ::core::ffi::c_uint;
pub const VISIT_BLACK: bk_cell_visit_state = 2;
pub const VISIT_GRAY: bk_cell_visit_state = 1;
pub const VISIT_WHITE: bk_cell_visit_state = 0;
pub type bk_Block = __caryll_bkblock;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
unsafe extern "C" fn bkblock_acells(mut b: *mut bk_Block, mut len: uint32_t) {
    if len <= (*b).length.wrapping_add((*b).free) {
        (*b).free = (*b).free.wrapping_sub(len.wrapping_sub((*b).length));
        (*b).length = len;
    } else {
        (*b).length = len;
        (*b).free = len >> 1 as ::core::ffi::c_int & 0xffffff as uint32_t;
        (*b).cells = __caryll_reallocate(
            (*b).cells as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<bk_Cell>() as size_t)
                .wrapping_mul((*b).length.wrapping_add((*b).free) as size_t),
            12 as ::core::ffi::c_ulong,
        ) as *mut bk_Cell;
    };
}
#[no_mangle]
pub unsafe extern "C" fn bk_cellIsPointer(mut cell: *mut bk_Cell) -> bool {
    return (*cell).t as ::core::ffi::c_uint >= p16 as ::core::ffi::c_int as ::core::ffi::c_uint;
}
unsafe extern "C" fn bkblock_grow(mut b: *mut bk_Block, mut len: uint32_t) -> *mut bk_Cell {
    let mut olen: uint32_t = (*b).length;
    bkblock_acells(b, olen.wrapping_add(len));
    return (*b).cells.offset(olen as isize) as *mut bk_Cell;
}
#[no_mangle]
pub unsafe extern "C" fn _bkblock_init() -> *mut bk_Block {
    let mut b: *mut bk_Block = ::core::ptr::null_mut::<bk_Block>();
    b = __caryll_allocate_clean(
        ::core::mem::size_of::<bk_Block>() as size_t,
        27 as ::core::ffi::c_ulong,
    ) as *mut bk_Block;
    bkblock_acells(b, 0 as uint32_t);
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn bkblock_pushint(
    mut b: *mut bk_Block,
    mut type_0: bk_CellType,
    mut x: uint32_t,
) {
    let mut cell: *mut bk_Cell = bkblock_grow(b, 1 as uint32_t);
    (*cell).t = type_0;
    (*cell).c2rust_unnamed.z = x;
}
#[no_mangle]
pub unsafe extern "C" fn bkblock_pushptr(
    mut b: *mut bk_Block,
    mut type_0: bk_CellType,
    mut p: *mut bk_Block,
) {
    let mut cell: *mut bk_Cell = bkblock_grow(b, 1 as uint32_t);
    (*cell).t = type_0;
    (*cell).c2rust_unnamed.p = p as *mut __caryll_bkblock;
}
unsafe extern "C" fn vbkpushitems(
    mut b: *mut bk_Block,
    mut type0: bk_CellType,
    mut ap: ::core::ffi::VaList,
) {
    let mut curtype: bk_CellType = type0;
    while curtype as u64 != 0 {
        if curtype as ::core::ffi::c_uint == bkcopy as ::core::ffi::c_int as ::core::ffi::c_uint
            || curtype as ::core::ffi::c_uint
                == bkembed as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut par: *mut bk_Block = ap.arg::<*mut bk_Block>();
            if !par.is_null() && !(*par).cells.is_null() {
                let mut j: uint32_t = 0 as uint32_t;
                while j < (*par).length {
                    if bk_cellIsPointer((*par).cells.offset(j as isize)) {
                        bkblock_pushptr(
                            b,
                            (*(*par).cells.offset(j as isize)).t,
                            (*(*par).cells.offset(j as isize)).c2rust_unnamed.p as *mut bk_Block,
                        );
                    } else {
                        bkblock_pushint(
                            b,
                            (*(*par).cells.offset(j as isize)).t,
                            (*(*par).cells.offset(j as isize)).c2rust_unnamed.z,
                        );
                    }
                    j = j.wrapping_add(1);
                }
            }
            if curtype as ::core::ffi::c_uint
                == bkembed as ::core::ffi::c_int as ::core::ffi::c_uint
                && !par.is_null()
            {
                free((*par).cells as *mut ::core::ffi::c_void);
                (*par).cells = ::core::ptr::null_mut::<bk_Cell>();
                free(par as *mut ::core::ffi::c_void);
                par = ::core::ptr::null_mut::<bk_Block>();
            }
        } else if (curtype as ::core::ffi::c_uint)
            < p16 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut par_0: uint32_t = ap.arg::<::core::ffi::c_int>() as uint32_t;
            bkblock_pushint(b, curtype, par_0);
        } else {
            let mut par_1: *mut bk_Block = ap.arg::<*mut bk_Block>();
            bkblock_pushptr(b, curtype, par_1);
        }
        curtype = ap.arg::<::core::ffi::c_int>() as bk_CellType;
    }
}
#[no_mangle]
pub unsafe extern "C" fn bk_new_Block(
    mut type0: ::core::ffi::c_int,
    mut args: ...
) -> *mut bk_Block {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    let mut b: *mut bk_Block = _bkblock_init();
    vbkpushitems(b, type0 as bk_CellType, ap.as_va_list());
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn bk_push(
    mut b: *mut bk_Block,
    mut type0: ::core::ffi::c_int,
    mut args: ...
) -> *mut bk_Block {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    vbkpushitems(b, type0 as bk_CellType, ap.as_va_list());
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn bk_newBlockFromStringLen(
    mut len: size_t,
    mut str: *const ::core::ffi::c_char,
) -> *mut bk_Block {
    if str.is_null() {
        return ::core::ptr::null_mut::<bk_Block>();
    }
    let mut b: *mut bk_Block = bk_new_Block(bkover as ::core::ffi::c_int);
    let mut j: size_t = 0 as size_t;
    while j < len {
        bkblock_pushint(b, b8, *str.offset(j as isize) as uint32_t);
        j = j.wrapping_add(1);
    }
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn bk_newBlockFromBuffer(mut buf: *mut caryll_Buffer) -> *mut bk_Block {
    if buf.is_null() {
        return ::core::ptr::null_mut::<bk_Block>();
    }
    let mut b: *mut bk_Block = bk_new_Block(bkover as ::core::ffi::c_int);
    let mut j: size_t = 0 as size_t;
    while j < (*buf).size {
        bkblock_pushint(b, b8, *(*buf).data.offset(j as isize) as uint32_t);
        j = j.wrapping_add(1);
    }
    buffree(buf);
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn bk_newBlockFromBufferCopy(mut buf: *const caryll_Buffer) -> *mut bk_Block {
    if buf.is_null() {
        return ::core::ptr::null_mut::<bk_Block>();
    }
    let mut b: *mut bk_Block = bk_new_Block(bkover as ::core::ffi::c_int);
    let mut j: size_t = 0 as size_t;
    while j < (*buf).size {
        bkblock_pushint(b, b8, *(*buf).data.offset(j as isize) as uint32_t);
        j = j.wrapping_add(1);
    }
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn bk_printBlock(mut b: *mut bk_Block) {
    fprintf(
        stderr,
        b"Block size %08x\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*b).length,
    );
    fprintf(
        stderr,
        b"------------------\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut j: uint32_t = 0 as uint32_t;
    while j < (*b).length {
        if bk_cellIsPointer((*b).cells.offset(j as isize)) {
            if !(*(*b).cells.offset(j as isize)).c2rust_unnamed.p.is_null() {
                fprintf(
                    stderr,
                    b"  %3d %p[%d]\n\0" as *const u8 as *const ::core::ffi::c_char,
                    (*(*b).cells.offset(j as isize)).t as ::core::ffi::c_uint,
                    (*(*b).cells.offset(j as isize)).c2rust_unnamed.p,
                    (*(*(*b).cells.offset(j as isize)).c2rust_unnamed.p)._index,
                );
            } else {
                fprintf(
                    stderr,
                    b"  %3d [NULL]\n\0" as *const u8 as *const ::core::ffi::c_char,
                    (*(*b).cells.offset(j as isize)).t as ::core::ffi::c_uint,
                );
            }
        } else {
            fprintf(
                stderr,
                b"  %3d %d\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*(*b).cells.offset(j as isize)).t as ::core::ffi::c_uint,
                (*(*b).cells.offset(j as isize)).c2rust_unnamed.z,
            );
        }
        j = j.wrapping_add(1);
    }
    fprintf(
        stderr,
        b"------------------\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
