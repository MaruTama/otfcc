extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub type sds = *mut ::core::ffi::c_char;
pub type f16dot16 = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_ILoggerTarget {
    pub dispose: Option<unsafe extern "C" fn(*mut otfcc_ILoggerTarget) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otfcc_ILoggerTarget, sds) -> ()>,
}
pub type otfcc_LoggerType = ::core::ffi::c_uint;
pub const log_type_progress: otfcc_LoggerType = 3;
pub const log_type_info: otfcc_LoggerType = 2;
pub const log_type_warning: otfcc_LoggerType = 1;
pub const log_type_error: otfcc_LoggerType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_ILogger {
    pub dispose: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub indent: Option<unsafe extern "C" fn(*mut otfcc_ILogger, *const ::core::ffi::c_char) -> ()>,
    pub indentSDS: Option<unsafe extern "C" fn(*mut otfcc_ILogger, sds) -> ()>,
    pub start: Option<unsafe extern "C" fn(*mut otfcc_ILogger, *const ::core::ffi::c_char) -> ()>,
    pub startSDS: Option<unsafe extern "C" fn(*mut otfcc_ILogger, sds) -> ()>,
    pub log: Option<
        unsafe extern "C" fn(
            *mut otfcc_ILogger,
            uint8_t,
            otfcc_LoggerType,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    pub logSDS:
        Option<unsafe extern "C" fn(*mut otfcc_ILogger, uint8_t, otfcc_LoggerType, sds) -> ()>,
    pub dedent: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub finish: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub end: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub setVerbosity: Option<unsafe extern "C" fn(*mut otfcc_ILogger, uint8_t) -> ()>,
    pub getTarget: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> *mut otfcc_ILoggerTarget>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_Options {
    pub debug_wait_on_start: bool,
    pub ignore_glyph_order: bool,
    pub ignore_hints: bool,
    pub has_vertical_metrics: bool,
    pub export_fdselect: bool,
    pub keep_average_char_width: bool,
    pub keep_unicode_ranges: bool,
    pub short_post: bool,
    pub dummy_DSIG: bool,
    pub keep_modified_time: bool,
    pub instr_as_bytes: bool,
    pub verbose: bool,
    pub quiet: bool,
    pub cff_short_vmtx: bool,
    pub merge_lookups: bool,
    pub merge_features: bool,
    pub force_cid: bool,
    pub cff_rollCharString: bool,
    pub cff_doSubroutinize: bool,
    pub stub_cmap4: bool,
    pub decimal_cmap: bool,
    pub name_glyphs_by_hash: bool,
    pub name_glyphs_by_gid: bool,
    pub glyph_name_prefix: *mut ::core::ffi::c_char,
    pub logger: *mut otfcc_ILogger,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_PacketPiece {
    pub tag: uint32_t,
    pub checkSum: uint32_t,
    pub offset: uint32_t,
    pub length: uint32_t,
    pub data: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_Packet {
    pub sfnt_version: uint32_t,
    pub numTables: uint16_t,
    pub searchRange: uint16_t,
    pub entrySelector: uint16_t,
    pub rangeShift: uint16_t,
    pub pieces: *mut otfcc_PacketPiece,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_maxp {
    pub version: f16dot16,
    pub numGlyphs: uint16_t,
    pub maxPoints: uint16_t,
    pub maxContours: uint16_t,
    pub maxCompositePoints: uint16_t,
    pub maxCompositeContours: uint16_t,
    pub maxZones: uint16_t,
    pub maxTwilightPoints: uint16_t,
    pub maxStorage: uint16_t,
    pub maxFunctionDefs: uint16_t,
    pub maxInstructionDefs: uint16_t,
    pub maxStackElements: uint16_t,
    pub maxSizeOfInstructions: uint16_t,
    pub maxComponentElements: uint16_t,
    pub maxComponentDepth: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct device_record {
    pub pixelSize: uint8_t,
    pub maxWidth: uint8_t,
    pub widths: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_hdmx {
    pub version: uint16_t,
    pub numRecords: uint16_t,
    pub sizeDeviceRecord: uint32_t,
    pub records: *mut device_record,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_hdmx {
    pub init: Option<unsafe extern "C" fn(*mut table_hdmx) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_hdmx, *const table_hdmx) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_hdmx, *mut table_hdmx) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_hdmx) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_hdmx, table_hdmx) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_hdmx, table_hdmx) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_hdmx>,
    pub free: Option<unsafe extern "C" fn(*mut table_hdmx) -> ()>,
}
pub type font_file_pointer = *mut uint8_t;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn disposeHdmx(mut table: *mut table_hdmx) {
    if (*table).records.is_null() {
        return;
    }
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*table).numRecords as uint32_t {
        if !(*(*table).records.offset(i as isize)).widths.is_null() {
            free((*(*table).records.offset(i as isize)).widths as *mut ::core::ffi::c_void);
            let ref mut fresh0 = (*(*table).records.offset(i as isize)).widths;
            *fresh0 = ::core::ptr::null_mut::<uint8_t>();
        }
        i = i.wrapping_add(1);
    }
    free((*table).records as *mut ::core::ffi::c_void);
    (*table).records = ::core::ptr::null_mut::<device_record>();
}
#[no_mangle]
pub static mut table_iHdmx: __caryll_elementinterface_table_hdmx = unsafe {
    __caryll_elementinterface_table_hdmx {
        init: Some(table_hdmx_init as unsafe extern "C" fn(*mut table_hdmx) -> ()),
        copy: Some(
            table_hdmx_copy as unsafe extern "C" fn(*mut table_hdmx, *const table_hdmx) -> (),
        ),
        move_0: Some(
            table_hdmx_move as unsafe extern "C" fn(*mut table_hdmx, *mut table_hdmx) -> (),
        ),
        dispose: Some(table_hdmx_dispose as unsafe extern "C" fn(*mut table_hdmx) -> ()),
        replace: Some(
            table_hdmx_replace as unsafe extern "C" fn(*mut table_hdmx, table_hdmx) -> (),
        ),
        copyReplace: Some(
            table_hdmx_copyReplace as unsafe extern "C" fn(*mut table_hdmx, table_hdmx) -> (),
        ),
        create: Some(table_hdmx_create),
        free: Some(table_hdmx_free as unsafe extern "C" fn(*mut table_hdmx) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_hdmx_dispose(mut x: *mut table_hdmx) {
    disposeHdmx(x);
}
#[inline]
unsafe extern "C" fn table_hdmx_free(mut x: *mut table_hdmx) {
    if x.is_null() {
        return;
    }
    table_hdmx_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_hdmx_copyReplace(mut dst: *mut table_hdmx, src: table_hdmx) {
    table_hdmx_dispose(dst);
    table_hdmx_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_hdmx_copy(mut dst: *mut table_hdmx, mut src: *const table_hdmx) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_hdmx>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn table_hdmx_replace(mut dst: *mut table_hdmx, src: table_hdmx) {
    table_hdmx_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_hdmx>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn table_hdmx_create() -> *mut table_hdmx {
    let mut x: *mut table_hdmx =
        malloc(::core::mem::size_of::<table_hdmx>() as size_t) as *mut table_hdmx;
    table_hdmx_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_hdmx_move(mut dst: *mut table_hdmx, mut src: *mut table_hdmx) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_hdmx>() as size_t,
    );
    table_hdmx_init(src);
}
#[inline]
unsafe extern "C" fn table_hdmx_init(mut x: *mut table_hdmx) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<table_hdmx>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_readHdmx(
    mut packet: otfcc_Packet,
    mut options: *const otfcc_Options,
    mut maxp: *mut table_maxp,
) -> *mut table_hdmx {
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1751412088i32 as uint32_t {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                if __fortable_k2 != 0 {
                    let mut data: font_file_pointer = table.data as font_file_pointer;
                    let mut hdmx: *mut table_hdmx = ::core::ptr::null_mut::<table_hdmx>();
                    hdmx = __caryll_allocate_clean(
                        ::core::mem::size_of::<table_hdmx>() as size_t,
                        20 as ::core::ffi::c_ulong,
                    ) as *mut table_hdmx;
                    (*hdmx).version = read_16u(data as *const uint8_t);
                    (*hdmx).numRecords =
                        read_16u(data.offset(2 as ::core::ffi::c_int as isize) as *const uint8_t);
                    (*hdmx).sizeDeviceRecord =
                        read_32u(data.offset(4 as ::core::ffi::c_int as isize) as *const uint8_t);
                    (*hdmx).records = __caryll_allocate_clean(
                        (::core::mem::size_of::<device_record>() as size_t)
                            .wrapping_mul((*hdmx).numRecords as size_t),
                        24 as ::core::ffi::c_ulong,
                    ) as *mut device_record;
                    let mut i: uint32_t = 0 as uint32_t;
                    while i < (*hdmx).numRecords as uint32_t {
                        (*(*hdmx).records.offset(i as isize)).pixelSize = *data
                            .offset(8 as ::core::ffi::c_int as isize)
                            .offset(i.wrapping_mul(
                                (2 as ::core::ffi::c_int + (*maxp).numGlyphs as ::core::ffi::c_int)
                                    as uint32_t,
                            ) as isize);
                        (*(*hdmx).records.offset(i as isize)).maxWidth = *data
                            .offset(8 as ::core::ffi::c_int as isize)
                            .offset(i.wrapping_mul(
                                (2 as ::core::ffi::c_int + (*maxp).numGlyphs as ::core::ffi::c_int)
                                    as uint32_t,
                            ) as isize)
                            .offset(1 as ::core::ffi::c_int as isize);
                        let ref mut fresh1 = (*(*hdmx).records.offset(i as isize)).widths;
                        *fresh1 = __caryll_allocate_clean(
                            (::core::mem::size_of::<uint8_t>() as size_t)
                                .wrapping_mul((*maxp).numGlyphs as size_t),
                            29 as ::core::ffi::c_ulong,
                        ) as *mut uint8_t;
                        memcpy(
                            (*(*hdmx).records.offset(i as isize)).widths
                                as *mut ::core::ffi::c_void,
                            data.offset(8 as ::core::ffi::c_int as isize)
                                .offset(i.wrapping_mul(
                                    (2 as ::core::ffi::c_int
                                        + (*maxp).numGlyphs as ::core::ffi::c_int)
                                        as uint32_t,
                                ) as isize)
                                .offset(2 as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            (*maxp).numGlyphs as size_t,
                        );
                        i = i.wrapping_add(1);
                    }
                    return hdmx;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return ::core::ptr::null_mut::<table_hdmx>();
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
unsafe extern "C" fn read_16u(mut src: *const uint8_t) -> uint16_t {
    let mut b0: uint16_t = ((*src.offset(0 as ::core::ffi::c_int as isize) as uint16_t
        as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int) as uint16_t;
    let mut b1: uint16_t = *src.offset(1 as ::core::ffi::c_int as isize) as uint16_t;
    return (b0 as ::core::ffi::c_int | b1 as ::core::ffi::c_int) as uint16_t;
}
#[inline]
unsafe extern "C" fn read_32u(mut src: *const uint8_t) -> uint32_t {
    let mut b0: uint32_t =
        (*src.offset(0 as ::core::ffi::c_int as isize) as uint32_t) << 24 as ::core::ffi::c_int;
    let mut b1: uint32_t =
        (*src.offset(1 as ::core::ffi::c_int as isize) as uint32_t) << 16 as ::core::ffi::c_int;
    let mut b2: uint32_t =
        (*src.offset(2 as ::core::ffi::c_int as isize) as uint32_t) << 8 as ::core::ffi::c_int;
    let mut b3: uint32_t = *src.offset(3 as ::core::ffi::c_int as isize) as uint32_t;
    return b0 | b1 | b2 | b3;
}
