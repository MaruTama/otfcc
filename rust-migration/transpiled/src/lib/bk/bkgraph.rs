extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn bufnew() -> *mut caryll_Buffer;
    fn bufwrite8(buf: *mut caryll_Buffer, byte: uint8_t);
    fn bufwrite16b(buf: *mut caryll_Buffer, x: uint16_t);
    fn bufwrite32b(buf: *mut caryll_Buffer, x: uint32_t);
    fn bk_new_Block(type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_cellIsPointer(cell: *mut bk_Cell) -> bool;
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bk_GraphNode {
    pub alias: uint32_t,
    pub order: uint32_t,
    pub height: uint32_t,
    pub hash: uint32_t,
    pub block: *mut bk_Block,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bk_Graph {
    pub length: uint32_t,
    pub free: uint32_t,
    pub entries: *mut bk_GraphNode,
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
unsafe extern "C" fn _bkgraph_grow(mut f: *mut bk_Graph) -> *mut bk_GraphNode {
    if (*f).free != 0 {
        (*f).length = (*f).length.wrapping_add(1);
        (*f).free = (*f).free.wrapping_sub(1);
    } else {
        (*f).length = (*f).length.wrapping_add(1 as uint32_t);
        (*f).free = (*f).length >> 1 as ::core::ffi::c_int & 0xffffff as uint32_t;
        (*f).entries = __caryll_reallocate(
            (*f).entries as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<bk_GraphNode>() as size_t)
                .wrapping_mul((*f).length.wrapping_add((*f).free) as size_t),
            10 as ::core::ffi::c_ulong,
        ) as *mut bk_GraphNode;
    }
    return (*f)
        .entries
        .offset((*f).length.wrapping_sub(1 as uint32_t) as isize) as *mut bk_GraphNode;
}
unsafe extern "C" fn dfs_insert_cells(
    mut b: *mut bk_Block,
    mut f: *mut bk_Graph,
    mut order: *mut uint32_t,
) -> uint32_t {
    if b.is_null()
        || (*b)._visitstate as ::core::ffi::c_uint
            == VISIT_GRAY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as uint32_t;
    }
    if (*b)._visitstate as ::core::ffi::c_uint
        == VISIT_BLACK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (*b)._height;
    }
    (*b)._visitstate = VISIT_GRAY;
    let mut height: uint32_t = 0 as uint32_t;
    let mut j: uint32_t = 0 as uint32_t;
    while j < (*b).length {
        let mut cell: *mut bk_Cell = (*b).cells.offset(j as isize) as *mut bk_Cell;
        if bk_cellIsPointer(cell) as ::core::ffi::c_int != 0 && !(*cell).c2rust_unnamed.p.is_null()
        {
            let mut thatHeight: uint32_t =
                dfs_insert_cells((*cell).c2rust_unnamed.p as *mut bk_Block, f, order);
            if thatHeight.wrapping_add(1 as uint32_t) > height {
                height = thatHeight.wrapping_add(1 as uint32_t);
            }
        }
        j = j.wrapping_add(1);
    }
    let mut e: *mut bk_GraphNode = _bkgraph_grow(f);
    (*e).alias = 0 as uint32_t;
    (*e).block = b;
    *order = (*order).wrapping_add(1 as uint32_t);
    (*e).order = *order;
    (*b)._height = height;
    (*e).height = (*b)._height;
    (*b)._visitstate = VISIT_BLACK;
    return height;
}
unsafe extern "C" fn _by_height(
    mut _a: *const ::core::ffi::c_void,
    mut _b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut a: *const bk_GraphNode = _a as *const bk_GraphNode;
    let mut b: *const bk_GraphNode = _b as *const bk_GraphNode;
    return (if (*a).height == (*b).height {
        (*a).order.wrapping_sub((*b).order)
    } else {
        (*b).height.wrapping_sub((*a).height)
    }) as ::core::ffi::c_int;
}
unsafe extern "C" fn _by_order(
    mut _a: *const ::core::ffi::c_void,
    mut _b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut a: *const bk_GraphNode = _a as *const bk_GraphNode;
    let mut b: *const bk_GraphNode = _b as *const bk_GraphNode;
    return (if !(*a).block.is_null()
        && !(*b).block.is_null()
        && (*(*a).block)._visitstate as ::core::ffi::c_uint
            != (*(*b).block)._visitstate as ::core::ffi::c_uint
    {
        ((*(*b).block)._visitstate as uint32_t).wrapping_sub((*(*a).block)._visitstate as uint32_t)
    } else if !(*a).block.is_null()
        && !(*b).block.is_null()
        && (*(*a).block)._depth != (*(*b).block)._depth
    {
        (*(*a).block)._depth.wrapping_sub((*(*b).block)._depth)
    } else {
        (*b).order.wrapping_sub((*a).order)
    }) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bk_newGraphFromRootBlock(mut b: *mut bk_Block) -> *mut bk_Graph {
    let mut forest: *mut bk_Graph = ::core::ptr::null_mut::<bk_Graph>();
    forest = __caryll_allocate_clean(
        ::core::mem::size_of::<bk_Graph>() as size_t,
        55 as ::core::ffi::c_ulong,
    ) as *mut bk_Graph;
    let mut tsOrder: uint32_t = 0 as uint32_t;
    dfs_insert_cells(b, forest, &raw mut tsOrder);
    qsort(
        (*forest).entries as *mut ::core::ffi::c_void,
        (*forest).length as size_t,
        ::core::mem::size_of::<bk_GraphNode>() as size_t,
        Some(
            _by_height
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    let mut j: uint32_t = 0 as uint32_t;
    while j < (*forest).length {
        (*(*(*forest).entries.offset(j as isize)).block)._index = j;
        (*(*forest).entries.offset(j as isize)).alias = j;
        j = j.wrapping_add(1);
    }
    return forest;
}
#[no_mangle]
pub unsafe extern "C" fn bk_delete_Graph(mut f: *mut bk_Graph) {
    if f.is_null() || (*f).entries.is_null() {
        return;
    }
    let mut j: uint32_t = 0 as uint32_t;
    while j < (*f).length {
        let mut b: *mut bk_Block = (*(*f).entries.offset(j as isize)).block;
        if !b.is_null() && !(*b).cells.is_null() {
            free((*b).cells as *mut ::core::ffi::c_void);
            (*b).cells = ::core::ptr::null_mut::<bk_Cell>();
        }
        free(b as *mut ::core::ffi::c_void);
        b = ::core::ptr::null_mut::<bk_Block>();
        j = j.wrapping_add(1);
    }
    free((*f).entries as *mut ::core::ffi::c_void);
    (*f).entries = ::core::ptr::null_mut::<bk_GraphNode>();
    free(f as *mut ::core::ffi::c_void);
    f = ::core::ptr::null_mut::<bk_Graph>();
}
unsafe extern "C" fn gethash(mut b: *mut bk_Block) -> uint32_t {
    let mut h: uint32_t = 5381 as uint32_t;
    let mut j: uint32_t = 0 as uint32_t;
    while j < (*b).length {
        h = (h << 5 as ::core::ffi::c_int)
            .wrapping_add(h)
            .wrapping_add((*(*b).cells.offset(j as isize)).t as uint32_t);
        h = (h << 5 as ::core::ffi::c_int).wrapping_add(h);
        match (*(*b).cells.offset(j as isize)).t as ::core::ffi::c_uint {
            1 | 2 | 3 => {
                h = h.wrapping_add((*(*b).cells.offset(j as isize)).c2rust_unnamed.z);
            }
            16 | 17 | 128 | 129 => {
                if !(*(*b).cells.offset(j as isize)).c2rust_unnamed.p.is_null() {
                    h = h.wrapping_add((*(*(*b).cells.offset(j as isize)).c2rust_unnamed.p)._index);
                }
            }
            _ => {}
        }
        j = j.wrapping_add(1);
    }
    return h;
}
unsafe extern "C" fn compareblock(mut a: *mut bk_Block, mut b: *mut bk_Block) -> bool {
    if a.is_null() && b.is_null() {
        return true_0 != 0;
    }
    if a.is_null() || b.is_null() {
        return false_0 != 0;
    }
    if (*a).length != (*b).length {
        return false_0 != 0;
    }
    let mut j: uint32_t = 0 as uint32_t;
    while j < (*a).length {
        if (*(*a).cells.offset(j as isize)).t as ::core::ffi::c_uint
            != (*(*b).cells.offset(j as isize)).t as ::core::ffi::c_uint
        {
            return false_0 != 0;
        }
        match (*(*a).cells.offset(j as isize)).t as ::core::ffi::c_uint {
            1 | 2 | 3 => {
                if (*(*a).cells.offset(j as isize)).c2rust_unnamed.z
                    != (*(*b).cells.offset(j as isize)).c2rust_unnamed.z
                {
                    return false_0 != 0;
                }
            }
            16 | 17 | 128 | 129 => {
                if (*(*a).cells.offset(j as isize)).c2rust_unnamed.p
                    != (*(*b).cells.offset(j as isize)).c2rust_unnamed.p
                {
                    return false_0 != 0;
                }
            }
            _ => {}
        }
        j = j.wrapping_add(1);
    }
    return true_0 != 0;
}
unsafe extern "C" fn compareEntry(mut a: *mut bk_GraphNode, mut b: *mut bk_GraphNode) -> bool {
    if (*a).hash != (*b).hash {
        return false_0 != 0;
    }
    return compareblock((*a).block, (*b).block);
}
unsafe extern "C" fn replaceptr(mut f: *mut bk_Graph, mut b: *mut bk_Block) {
    let mut j: uint32_t = 0 as uint32_t;
    while j < (*b).length {
        match (*(*b).cells.offset(j as isize)).t as ::core::ffi::c_uint {
            16 | 17 | 128 | 129 => {
                if !(*(*b).cells.offset(j as isize)).c2rust_unnamed.p.is_null() {
                    let mut index: uint32_t =
                        (*(*(*b).cells.offset(j as isize)).c2rust_unnamed.p)._index;
                    while (*(*f).entries.offset(index as isize)).alias != index {
                        index = (*(*f).entries.offset(index as isize)).alias;
                    }
                    let ref mut fresh0 = (*(*b).cells.offset(j as isize)).c2rust_unnamed.p;
                    *fresh0 = (*(*f).entries.offset(index as isize)).block as *mut __caryll_bkblock;
                }
            }
            _ => {}
        }
        j = j.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn bk_minimizeGraph(mut f: *mut bk_Graph) {
    let mut rear: uint32_t = (*f).length.wrapping_sub(1 as uint32_t);
    while rear > 0 as uint32_t {
        let mut front: uint32_t = rear;
        while (*(*f).entries.offset(front as isize)).height
            == (*(*f).entries.offset(rear as isize)).height
            && front > 0 as uint32_t
        {
            front = front.wrapping_sub(1);
        }
        front = front.wrapping_add(1);
        let mut j: uint32_t = front;
        while j <= rear {
            (*(*f).entries.offset(j as isize)).hash =
                gethash((*(*f).entries.offset(j as isize)).block);
            j = j.wrapping_add(1);
        }
        let mut j_0: uint32_t = front;
        while j_0 <= rear {
            let mut a: *mut bk_GraphNode = (*f).entries.offset(j_0 as isize) as *mut bk_GraphNode;
            if (*a).alias == j_0 {
                let mut k: uint32_t = j_0.wrapping_add(1 as uint32_t);
                while k <= rear {
                    let mut b: *mut bk_GraphNode =
                        (*f).entries.offset(k as isize) as *mut bk_GraphNode;
                    if (*b).alias == k && compareEntry(a, b) as ::core::ffi::c_int != 0 {
                        (*b).alias = j_0;
                    }
                    k = k.wrapping_add(1);
                }
            }
            j_0 = j_0.wrapping_add(1);
        }
        let mut j_1: uint32_t = 0 as uint32_t;
        while j_1 < front {
            replaceptr(f, (*(*f).entries.offset(j_1 as isize)).block);
            j_1 = j_1.wrapping_add(1);
        }
        rear = front.wrapping_sub(1 as uint32_t);
    }
}
unsafe extern "C" fn otfcc_bkblock_size(mut b: *mut bk_Block) -> size_t {
    let mut size: size_t = 0 as size_t;
    let mut j: uint32_t = 0 as uint32_t;
    while j < (*b).length {
        match (*(*b).cells.offset(j as isize)).t as ::core::ffi::c_uint {
            1 => {
                size = size.wrapping_add(1 as size_t);
            }
            2 | 16 | 128 => {
                size = size.wrapping_add(2 as size_t);
            }
            3 | 17 | 129 => {
                size = size.wrapping_add(4 as size_t);
            }
            _ => {}
        }
        j = j.wrapping_add(1);
    }
    return size;
}
unsafe extern "C" fn getoffset(
    mut offsets: *mut size_t,
    mut ref_0: *mut bk_Block,
    mut target: *mut bk_Block,
    mut bits: uint8_t,
) -> uint32_t {
    let mut offref: size_t = *offsets.offset((*ref_0)._index as isize);
    let mut offtgt: size_t = *offsets.offset((*target)._index as isize);
    if (bits as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
        && (offtgt < offref || offtgt.wrapping_sub(offref) >> bits as ::core::ffi::c_int != 0)
    {
        fprintf(
            stderr,
            b"[otfcc-bk] Warning : Unable to fit offset %d into %d bits; output may be corrupted.\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            offtgt.wrapping_sub(offref) as int32_t,
            bits as ::core::ffi::c_int,
        );
    }
    return offtgt.wrapping_sub(offref) as uint32_t;
}
unsafe extern "C" fn getoffset_untangle(
    mut offsets: *mut size_t,
    mut ref_0: *mut bk_Block,
    mut target: *mut bk_Block,
) -> int64_t {
    let mut offref: size_t = *offsets.offset((*ref_0)._index as isize);
    let mut offtgt: size_t = *offsets.offset((*target)._index as isize);
    return offtgt.wrapping_sub(offref) as int64_t;
}
unsafe extern "C" fn escalate_sppointers(
    mut b: *mut bk_Block,
    mut f: *mut bk_Graph,
    mut order: *mut uint32_t,
    mut depth: uint32_t,
) {
    if b.is_null() {
        return;
    }
    let mut j: uint32_t = 0 as uint32_t;
    while j < (*b).length {
        let mut cell: *mut bk_Cell = (*b).cells.offset(j as isize) as *mut bk_Cell;
        if bk_cellIsPointer(cell) as ::core::ffi::c_int != 0
            && !(*cell).c2rust_unnamed.p.is_null()
            && (*cell).t as ::core::ffi::c_uint >= sp16 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            escalate_sppointers((*cell).c2rust_unnamed.p as *mut bk_Block, f, order, depth);
        }
        j = j.wrapping_add(1);
    }
    (*b)._depth = depth;
    *order = (*order).wrapping_add(1 as uint32_t);
    (*(*f).entries.offset((*b)._index as isize)).order = *order;
}
unsafe extern "C" fn dfs_attract_cells(
    mut b: *mut bk_Block,
    mut f: *mut bk_Graph,
    mut order: *mut uint32_t,
    mut depth: uint32_t,
) {
    if b.is_null() {
        return;
    }
    if (*b)._visitstate as ::core::ffi::c_uint
        != VISIT_WHITE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*b)._depth < depth {
            (*b)._depth = depth;
        }
        return;
    }
    (*b)._visitstate = VISIT_GRAY;
    let mut j: uint32_t = (*b).length;
    loop {
        let fresh1 = j;
        j = j.wrapping_sub(1);
        if !(fresh1 > 0 as uint32_t) {
            break;
        }
        let mut cell: *mut bk_Cell = (*b).cells.offset(j as isize) as *mut bk_Cell;
        if bk_cellIsPointer(cell) as ::core::ffi::c_int != 0 && !(*cell).c2rust_unnamed.p.is_null()
        {
            dfs_attract_cells(
                (*cell).c2rust_unnamed.p as *mut bk_Block,
                f,
                order,
                depth.wrapping_add(1 as uint32_t),
            );
        }
    }
    *order = (*order).wrapping_add(1 as uint32_t);
    (*(*f).entries.offset((*b)._index as isize)).order = *order;
    escalate_sppointers(b, f, order, depth);
    (*b)._visitstate = VISIT_BLACK;
}
unsafe extern "C" fn attract_bkgraph(mut f: *mut bk_Graph) {
    let mut j: uint32_t = 0 as uint32_t;
    while j < (*f).length {
        (*(*(*f).entries.offset(j as isize)).block)._visitstate = VISIT_WHITE;
        (*(*f).entries.offset(j as isize)).order = 0 as uint32_t;
        (*(*(*f).entries.offset(j as isize)).block)._index = j;
        (*(*(*f).entries.offset(j as isize)).block)._depth = 0 as uint32_t;
        j = j.wrapping_add(1);
    }
    let mut order: uint32_t = 0 as uint32_t;
    dfs_attract_cells(
        (*(*f).entries.offset(0 as ::core::ffi::c_int as isize)).block,
        f,
        &raw mut order,
        0 as uint32_t,
    );
    qsort(
        (*f).entries as *mut ::core::ffi::c_void,
        (*f).length as size_t,
        ::core::mem::size_of::<bk_GraphNode>() as size_t,
        Some(
            _by_order
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    let mut j_0: uint32_t = 0 as uint32_t;
    while j_0 < (*f).length {
        (*(*(*f).entries.offset(j_0 as isize)).block)._index = j_0;
        j_0 = j_0.wrapping_add(1);
    }
}
unsafe extern "C" fn try_untabgle_block(
    mut f: *mut bk_Graph,
    mut b: *mut bk_Block,
    mut offsets: *mut size_t,
    mut passes: uint16_t,
) -> bool {
    let mut didCopy: bool = false_0 != 0;
    let mut j: uint32_t = 0 as uint32_t;
    while j < (*b).length {
        match (*(*b).cells.offset(j as isize)).t as ::core::ffi::c_uint {
            16 | 128 => {
                if !(*(*b).cells.offset(j as isize)).c2rust_unnamed.p.is_null() {
                    let mut offset: int64_t = getoffset_untangle(
                        offsets,
                        b,
                        (*(*b).cells.offset(j as isize)).c2rust_unnamed.p as *mut bk_Block,
                    );
                    if offset < 0 as int64_t || offset > 0xffff as int64_t {
                        let mut e: *mut bk_GraphNode = _bkgraph_grow(f);
                        (*e).order = 0 as uint32_t;
                        (*e).alias = 0 as uint32_t;
                        (*e).block = bk_new_Block(
                            bkcopy as ::core::ffi::c_int,
                            (*(*b).cells.offset(j as isize)).c2rust_unnamed.p,
                            bkover as ::core::ffi::c_int,
                        );
                        (*(*b).cells.offset(j as isize)).t = sp16;
                        let ref mut fresh2 = (*(*b).cells.offset(j as isize)).c2rust_unnamed.p;
                        *fresh2 = (*e).block as *mut __caryll_bkblock;
                        didCopy = true_0 != 0;
                    }
                }
            }
            _ => {}
        }
        j = j.wrapping_add(1);
    }
    return didCopy;
}
unsafe extern "C" fn try_untangle(mut f: *mut bk_Graph, mut passes: uint16_t) -> bool {
    let mut offsets: *mut size_t = ::core::ptr::null_mut::<size_t>();
    offsets = __caryll_allocate_clean(
        (::core::mem::size_of::<size_t>() as size_t)
            .wrapping_mul((*f).length.wrapping_add(1 as uint32_t) as size_t),
        294 as ::core::ffi::c_ulong,
    ) as *mut size_t;
    *offsets.offset(0 as ::core::ffi::c_int as isize) = 0 as size_t;
    let mut j: uint32_t = 0 as uint32_t;
    while j < (*f).length {
        if (*(*(*f).entries.offset(j as isize)).block)._visitstate as ::core::ffi::c_uint
            == VISIT_BLACK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            *offsets.offset(j.wrapping_add(1 as uint32_t) as isize) = (*offsets.offset(j as isize))
                .wrapping_add(otfcc_bkblock_size((*(*f).entries.offset(j as isize)).block));
        } else {
            *offsets.offset(j.wrapping_add(1 as uint32_t) as isize) = *offsets.offset(j as isize);
        }
        j = j.wrapping_add(1);
    }
    let mut totalBlocks: uint32_t = (*f).length;
    let mut didUntangle: bool = false_0 != 0;
    let mut j_0: uint32_t = 0 as uint32_t;
    while j_0 < totalBlocks {
        if (*(*(*f).entries.offset(j_0 as isize)).block)._visitstate as ::core::ffi::c_uint
            == VISIT_BLACK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut didCopy: bool = try_untabgle_block(
                f,
                (*(*f).entries.offset(j_0 as isize)).block,
                offsets,
                passes,
            );
            didUntangle =
                didUntangle as ::core::ffi::c_int != 0 || didCopy as ::core::ffi::c_int != 0;
        }
        j_0 = j_0.wrapping_add(1);
    }
    free(offsets as *mut ::core::ffi::c_void);
    offsets = ::core::ptr::null_mut::<size_t>();
    return didUntangle;
}
unsafe extern "C" fn otfcc_build_bkblock(
    mut buf: *mut caryll_Buffer,
    mut b: *mut bk_Block,
    mut offsets: *mut size_t,
) {
    let mut j: uint32_t = 0 as uint32_t;
    while j < (*b).length {
        match (*(*b).cells.offset(j as isize)).t as ::core::ffi::c_uint {
            1 => {
                bufwrite8(
                    buf,
                    (*(*b).cells.offset(j as isize)).c2rust_unnamed.z as uint8_t,
                );
            }
            2 => {
                bufwrite16b(
                    buf,
                    (*(*b).cells.offset(j as isize)).c2rust_unnamed.z as uint16_t,
                );
            }
            3 => {
                bufwrite32b(buf, (*(*b).cells.offset(j as isize)).c2rust_unnamed.z);
            }
            16 | 128 => {
                if !(*(*b).cells.offset(j as isize)).c2rust_unnamed.p.is_null() {
                    bufwrite16b(
                        buf,
                        getoffset(
                            offsets,
                            b,
                            (*(*b).cells.offset(j as isize)).c2rust_unnamed.p as *mut bk_Block,
                            16 as uint8_t,
                        ) as uint16_t,
                    );
                } else {
                    bufwrite16b(buf, 0 as uint16_t);
                }
            }
            17 | 129 => {
                if !(*(*b).cells.offset(j as isize)).c2rust_unnamed.p.is_null() {
                    bufwrite32b(
                        buf,
                        getoffset(
                            offsets,
                            b,
                            (*(*b).cells.offset(j as isize)).c2rust_unnamed.p as *mut bk_Block,
                            32 as uint8_t,
                        ),
                    );
                } else {
                    bufwrite32b(buf, 0 as uint32_t);
                }
            }
            _ => {}
        }
        j = j.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn bk_build_Graph(mut f: *mut bk_Graph) -> *mut caryll_Buffer {
    let mut buf: *mut caryll_Buffer = bufnew();
    let mut offsets: *mut size_t = ::core::ptr::null_mut::<size_t>();
    offsets = __caryll_allocate_clean(
        (::core::mem::size_of::<size_t>() as size_t)
            .wrapping_mul((*f).length.wrapping_add(1 as uint32_t) as size_t),
        352 as ::core::ffi::c_ulong,
    ) as *mut size_t;
    *offsets.offset(0 as ::core::ffi::c_int as isize) = 0 as size_t;
    let mut j: uint32_t = 0 as uint32_t;
    while j < (*f).length {
        if (*(*(*f).entries.offset(j as isize)).block)._visitstate as ::core::ffi::c_uint
            == VISIT_BLACK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            *offsets.offset(j.wrapping_add(1 as uint32_t) as isize) = (*offsets.offset(j as isize))
                .wrapping_add(otfcc_bkblock_size((*(*f).entries.offset(j as isize)).block));
        } else {
            *offsets.offset(j.wrapping_add(1 as uint32_t) as isize) = *offsets.offset(j as isize);
        }
        j = j.wrapping_add(1);
    }
    let mut j_0: uint32_t = 0 as uint32_t;
    while j_0 < (*f).length {
        if (*(*(*f).entries.offset(j_0 as isize)).block)._visitstate as ::core::ffi::c_uint
            == VISIT_BLACK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            otfcc_build_bkblock(buf, (*(*f).entries.offset(j_0 as isize)).block, offsets);
        }
        j_0 = j_0.wrapping_add(1);
    }
    free(offsets as *mut ::core::ffi::c_void);
    offsets = ::core::ptr::null_mut::<size_t>();
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn bk_estimateSizeOfGraph(mut f: *mut bk_Graph) -> size_t {
    let mut offsets: *mut size_t = ::core::ptr::null_mut::<size_t>();
    offsets = __caryll_allocate_clean(
        (::core::mem::size_of::<size_t>() as size_t)
            .wrapping_mul((*f).length.wrapping_add(1 as uint32_t) as size_t),
        373 as ::core::ffi::c_ulong,
    ) as *mut size_t;
    *offsets.offset(0 as ::core::ffi::c_int as isize) = 0 as size_t;
    let mut j: uint32_t = 0 as uint32_t;
    while j < (*f).length {
        if (*(*(*f).entries.offset(j as isize)).block)._visitstate as ::core::ffi::c_uint
            == VISIT_BLACK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            *offsets.offset(j.wrapping_add(1 as uint32_t) as isize) = (*offsets.offset(j as isize))
                .wrapping_add(otfcc_bkblock_size((*(*f).entries.offset(j as isize)).block));
        } else {
            *offsets.offset(j.wrapping_add(1 as uint32_t) as isize) = *offsets.offset(j as isize);
        }
        j = j.wrapping_add(1);
    }
    let mut estimatedSize: size_t = *offsets.offset((*f).length as isize);
    free(offsets as *mut ::core::ffi::c_void);
    offsets = ::core::ptr::null_mut::<size_t>();
    return estimatedSize;
}
#[no_mangle]
pub unsafe extern "C" fn bk_untangleGraph(mut f: *mut bk_Graph) {
    let mut passes: uint16_t = 0 as uint16_t;
    let mut tangled: bool = false_0 != 0;
    attract_bkgraph(f);
    loop {
        tangled = try_untangle(f, passes);
        if tangled {
            attract_bkgraph(f);
        }
        passes = passes.wrapping_add(1);
        if !(tangled as ::core::ffi::c_int != 0
            && (passes as ::core::ffi::c_int) < 16 as ::core::ffi::c_int)
        {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn bk_build_Block(mut root: *mut bk_Block) -> *mut caryll_Buffer {
    let mut f: *mut bk_Graph = bk_newGraphFromRootBlock(root);
    bk_minimizeGraph(f);
    bk_untangleGraph(f);
    let mut buf: *mut caryll_Buffer = bk_build_Graph(f);
    bk_delete_Graph(f);
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn bk_build_Block_noMinimize(mut root: *mut bk_Block) -> *mut caryll_Buffer {
    let mut f: *mut bk_Graph = bk_newGraphFromRootBlock(root);
    bk_untangleGraph(f);
    let mut buf: *mut caryll_Buffer = bk_build_Graph(f);
    bk_delete_Graph(f);
    return buf;
}
