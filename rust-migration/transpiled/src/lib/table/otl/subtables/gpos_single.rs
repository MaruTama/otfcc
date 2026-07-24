extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
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
    fn json_object_new(length: size_t) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: size_t) -> sds;
    static otfcc_iHandle: otfcc_HandlePackage;
    static otl_iCoverage: __otfcc_ICoverage;
    fn bk_new_Block(type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_push(b: *mut bk_Block, type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_newBlockFromBuffer(buf: *mut caryll_Buffer) -> *mut bk_Block;
    fn bk_build_Block(root: *mut bk_Block) -> *mut caryll_Buffer;
    fn position_format_length(format: uint16_t) -> uint8_t;
    fn read_gpos_value(
        data: font_file_pointer,
        tableLength: uint32_t,
        offset: uint32_t,
        format: uint16_t,
    ) -> otl_PositionValue;
    fn required_position_format(v: otl_PositionValue) -> uint8_t;
    fn bk_gpos_value(v: otl_PositionValue, format: uint16_t) -> *mut bk_Block;
    fn gpos_dump_value(value: otl_PositionValue) -> *mut json_value;
    fn gpos_parse_value(pos: *mut json_value) -> otl_PositionValue;
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __int64_t = i64;
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
pub type json_type = ::core::ffi::c_uint;
pub const json_pre_serialized: json_type = 8;
pub const json_null: json_type = 7;
pub const json_boolean: json_type = 6;
pub const json_string: json_type = 5;
pub const json_double: json_type = 4;
pub const json_integer: json_type = 3;
pub const json_array: json_type = 2;
pub const json_object: json_type = 1;
pub const json_none: json_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _json_value {
    pub parent: *mut _json_value,
    pub type_0: json_type,
    pub u: C2RustUnnamed_0,
    pub _reserved: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub next_alloc: *mut _json_value,
    pub object_mem: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub boolean: ::core::ffi::c_int,
    pub integer: int64_t,
    pub dbl: ::core::ffi::c_double,
    pub string: C2RustUnnamed_3,
    pub object: C2RustUnnamed_2,
    pub array: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub length: ::core::ffi::c_uint,
    pub values: *mut *mut _json_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub length: ::core::ffi::c_uint,
    pub values: *mut json_object_entry,
}
pub type json_object_entry = _json_object_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _json_object_entry {
    pub name: *mut ::core::ffi::c_char,
    pub name_length: ::core::ffi::c_uint,
    pub value: *mut _json_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub length: ::core::ffi::c_uint,
    pub ptr: *mut ::core::ffi::c_char,
}
pub type json_value = _json_value;
pub type sds = *mut ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct caryll_Buffer {
    pub cursor: size_t,
    pub size: size_t,
    pub free: size_t,
    pub data: *mut uint8_t,
}
pub type glyphid_t = uint16_t;
pub type glyphclass_t = uint16_t;
pub type tableid_t = uint16_t;
pub type pos_t = ::core::ffi::c_double;
pub type handle_state = ::core::ffi::c_uint;
pub const HANDLE_STATE_CONSOLIDATED: handle_state = 3;
pub const HANDLE_STATE_NAME: handle_state = 2;
pub const HANDLE_STATE_INDEX: handle_state = 1;
pub const HANDLE_STATE_EMPTY: handle_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_Handle {
    pub state: handle_state,
    pub index: glyphid_t,
    pub name: sds,
}
pub type otfcc_GlyphHandle = otfcc_Handle;
pub type otfcc_LookupHandle = otfcc_Handle;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_HandlePackage {
    pub init: Option<unsafe extern "C" fn(*mut otfcc_Handle) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otfcc_Handle, *const otfcc_Handle) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otfcc_Handle, *mut otfcc_Handle) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otfcc_Handle) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otfcc_Handle, otfcc_Handle) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otfcc_Handle, otfcc_Handle) -> ()>,
    pub empty: Option<unsafe extern "C" fn() -> otfcc_Handle>,
    pub dup: Option<unsafe extern "C" fn(otfcc_Handle) -> otfcc_Handle>,
    pub fromIndex: Option<unsafe extern "C" fn(glyphid_t) -> otfcc_Handle>,
    pub fromName: Option<unsafe extern "C" fn(sds) -> otfcc_Handle>,
    pub fromConsolidated: Option<unsafe extern "C" fn(glyphid_t, sds) -> otfcc_Handle>,
    pub consolidateTo: Option<unsafe extern "C" fn(*mut otfcc_Handle, glyphid_t, sds) -> ()>,
}
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
pub type font_file_pointer = *mut uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_Coverage {
    pub numGlyphs: glyphid_t,
    pub capacity: uint32_t,
    pub glyphs: *mut otfcc_GlyphHandle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __otfcc_ICoverage {
    pub init: Option<unsafe extern "C" fn(*mut otl_Coverage) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_Coverage, *const otl_Coverage) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_Coverage, *mut otl_Coverage) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_Coverage) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_Coverage, otl_Coverage) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_Coverage, otl_Coverage) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_Coverage>,
    pub free: Option<unsafe extern "C" fn(*mut otl_Coverage) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_Coverage, uint32_t) -> ()>,
    pub read: Option<unsafe extern "C" fn(*const uint8_t, uint32_t, uint32_t) -> *mut otl_Coverage>,
    pub dump: Option<unsafe extern "C" fn(*const otl_Coverage) -> *mut json_value>,
    pub parse: Option<unsafe extern "C" fn(*const json_value) -> *mut otl_Coverage>,
    pub build: Option<unsafe extern "C" fn(*const otl_Coverage) -> *mut caryll_Buffer>,
    pub buildFormat:
        Option<unsafe extern "C" fn(*const otl_Coverage, uint16_t) -> *mut caryll_Buffer>,
    pub shrink: Option<unsafe extern "C" fn(*mut otl_Coverage, bool) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_Coverage, otfcc_GlyphHandle) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_ClassDef {
    pub numGlyphs: glyphid_t,
    pub capacity: uint32_t,
    pub maxclass: glyphclass_t,
    pub glyphs: *mut otfcc_GlyphHandle,
    pub classes: *mut glyphclass_t,
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
    pub c2rust_unnamed: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
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
pub type otl_LookupType = ::core::ffi::c_uint;
pub const otl_type_gpos_extend: otl_LookupType = 41;
pub const otl_type_gpos_chaining: otl_LookupType = 40;
pub const otl_type_gpos_context: otl_LookupType = 39;
pub const otl_type_gpos_markToMark: otl_LookupType = 38;
pub const otl_type_gpos_markToLigature: otl_LookupType = 37;
pub const otl_type_gpos_markToBase: otl_LookupType = 36;
pub const otl_type_gpos_cursive: otl_LookupType = 35;
pub const otl_type_gpos_pair: otl_LookupType = 34;
pub const otl_type_gpos_single: otl_LookupType = 33;
pub const otl_type_gpos_unknown: otl_LookupType = 32;
pub const otl_type_gsub_reverse: otl_LookupType = 24;
pub const otl_type_gsub_extend: otl_LookupType = 23;
pub const otl_type_gsub_chaining: otl_LookupType = 22;
pub const otl_type_gsub_context: otl_LookupType = 21;
pub const otl_type_gsub_ligature: otl_LookupType = 20;
pub const otl_type_gsub_alternate: otl_LookupType = 19;
pub const otl_type_gsub_multiple: otl_LookupType = 18;
pub const otl_type_gsub_single: otl_LookupType = 17;
pub const otl_type_gsub_unknown: otl_LookupType = 16;
pub const otl_type_unknown: otl_LookupType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _otl_subtable {
    pub gsub_single: subtable_gsub_single,
    pub gsub_multi: subtable_gsub_multi,
    pub gsub_ligature: subtable_gsub_ligature,
    pub chaining: subtable_chaining,
    pub gsub_reverse: subtable_gsub_reverse,
    pub gpos_single: subtable_gpos_single,
    pub gpos_pair: subtable_gpos_pair,
    pub gpos_cursive: subtable_gpos_cursive,
    pub gpos_markToSingle: subtable_gpos_markToSingle,
    pub gpos_markToLigature: subtable_gpos_markToLigature,
    pub extend: subtable_extend,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_extend {
    pub type_0: otl_LookupType,
    pub subtable: *mut otl_Subtable,
}
pub type otl_Subtable = _otl_subtable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_gpos_markToLigature {
    pub classCount: glyphclass_t,
    pub markArray: otl_MarkArray,
    pub ligArray: otl_LigatureArray,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_LigatureArray {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_LigatureBaseRecord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_LigatureBaseRecord {
    pub glyph: otfcc_GlyphHandle,
    pub componentCount: glyphid_t,
    pub anchors: *mut *mut otl_Anchor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_Anchor {
    pub present: bool,
    pub x: pos_t,
    pub y: pos_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_MarkArray {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_MarkRecord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_MarkRecord {
    pub glyph: otfcc_GlyphHandle,
    pub markClass: glyphclass_t,
    pub anchor: otl_Anchor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_gpos_markToSingle {
    pub classCount: glyphclass_t,
    pub markArray: otl_MarkArray,
    pub baseArray: otl_BaseArray,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_BaseArray {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_BaseRecord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_BaseRecord {
    pub glyph: otfcc_GlyphHandle,
    pub anchors: *mut otl_Anchor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_gpos_cursive {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_GposCursiveEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_GposCursiveEntry {
    pub target: otfcc_GlyphHandle,
    pub enter: otl_Anchor,
    pub exit: otl_Anchor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_gpos_pair {
    pub first: *mut otl_ClassDef,
    pub second: *mut otl_ClassDef,
    pub firstValues: *mut *mut otl_PositionValue,
    pub secondValues: *mut *mut otl_PositionValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_PositionValue {
    pub dx: pos_t,
    pub dy: pos_t,
    pub dWidth: pos_t,
    pub dHeight: pos_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_gpos_single {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_GposSingleEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_GposSingleEntry {
    pub target: otfcc_GlyphHandle,
    pub value: otl_PositionValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_gsub_reverse {
    pub matchCount: tableid_t,
    pub inputIndex: tableid_t,
    pub match_0: *mut *mut otl_Coverage,
    pub to: *mut otl_Coverage,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_chaining {
    pub type_0: otl_chaining_type,
    pub c2rust_unnamed: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub rule: otl_ChainingRule,
    pub c2rust_unnamed: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub rulesCount: tableid_t,
    pub rules: *mut *mut otl_ChainingRule,
    pub bc: *mut otl_ClassDef,
    pub ic: *mut otl_ClassDef,
    pub fc: *mut otl_ClassDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_ChainingRule {
    pub matchCount: tableid_t,
    pub inputBegins: tableid_t,
    pub inputEnds: tableid_t,
    pub match_0: *mut *mut otl_Coverage,
    pub applyCount: tableid_t,
    pub apply: *mut otl_ChainLookupApplication,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_ChainLookupApplication {
    pub index: tableid_t,
    pub lookup: otfcc_LookupHandle,
}
pub type otl_chaining_type = ::core::ffi::c_uint;
pub const otl_chaining_classified: otl_chaining_type = 2;
pub const otl_chaining_poly: otl_chaining_type = 1;
pub const otl_chaining_canonical: otl_chaining_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_gsub_ligature {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_GsubLigatureEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_GsubLigatureEntry {
    pub from: *mut otl_Coverage,
    pub to: otfcc_GlyphHandle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_gsub_multi {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_GsubMultiEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_GsubMultiEntry {
    pub from: otfcc_GlyphHandle,
    pub to: *mut otl_Coverage,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_gsub_single {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_GsubSingleEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_GsubSingleEntry {
    pub from: otfcc_GlyphHandle,
    pub to: otfcc_GlyphHandle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_subtable_gpos_single {
    pub init: Option<unsafe extern "C" fn(*mut subtable_gpos_single) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut subtable_gpos_single, *const subtable_gpos_single) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut subtable_gpos_single, *mut subtable_gpos_single) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut subtable_gpos_single) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut subtable_gpos_single, subtable_gpos_single) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut subtable_gpos_single, subtable_gpos_single) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut subtable_gpos_single>,
    pub free: Option<unsafe extern "C" fn(*mut subtable_gpos_single) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut subtable_gpos_single, size_t) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut subtable_gpos_single, size_t) -> ()>,
    pub createN: Option<unsafe extern "C" fn(size_t) -> *mut subtable_gpos_single>,
    pub fill: Option<unsafe extern "C" fn(*mut subtable_gpos_single, size_t) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut subtable_gpos_single) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut subtable_gpos_single, otl_GposSingleEntry) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut subtable_gpos_single) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut subtable_gpos_single) -> otl_GposSingleEntry>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut subtable_gpos_single, size_t) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut subtable_gpos_single,
            Option<
                unsafe extern "C" fn(*const otl_GposSingleEntry, *mut ::core::ffi::c_void) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut subtable_gpos_single,
            Option<
                unsafe extern "C" fn(
                    *const otl_GposSingleEntry,
                    *const otl_GposSingleEntry,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_otl_GposSingleEntry {
    pub init: Option<unsafe extern "C" fn(*mut otl_GposSingleEntry) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut otl_GposSingleEntry, *const otl_GposSingleEntry) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut otl_GposSingleEntry, *mut otl_GposSingleEntry) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_GposSingleEntry) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_GposSingleEntry, otl_GposSingleEntry) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut otl_GposSingleEntry, otl_GposSingleEntry) -> ()>,
}
pub type otl_BuildHeuristics = ::core::ffi::c_uint;
pub const OTL_BH_GSUB_VERT: otl_BuildHeuristics = 1;
pub const OTL_BH_NORMAL: otl_BuildHeuristics = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn read_16u(mut src: *const uint8_t) -> uint16_t {
    let mut b0: uint16_t = ((*src.offset(0 as ::core::ffi::c_int as isize) as uint16_t
        as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int) as uint16_t;
    let mut b1: uint16_t = *src.offset(1 as ::core::ffi::c_int as isize) as uint16_t;
    return (b0 as ::core::ffi::c_int | b1 as ::core::ffi::c_int) as uint16_t;
}
unsafe extern "C" fn deleteGposSingleEntry(mut entry: *mut otl_GposSingleEntry) {
    otfcc_iHandle.dispose.expect("non-null function pointer")(&raw mut (*entry).target);
}
static mut gss_typeinfo: __caryll_elementinterface_otl_GposSingleEntry = unsafe {
    __caryll_elementinterface_otl_GposSingleEntry {
        init: None,
        copy: None,
        move_0: None,
        dispose: Some(
            deleteGposSingleEntry as unsafe extern "C" fn(*mut otl_GposSingleEntry) -> (),
        ),
        replace: None,
        copyReplace: None,
    }
};
#[inline]
unsafe extern "C" fn subtable_gpos_single_move(
    mut dst: *mut subtable_gpos_single,
    mut src: *mut subtable_gpos_single,
) {
    *dst = *src;
    subtable_gpos_single_init(src);
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_resizeTo(
    mut arr: *mut subtable_gpos_single,
    mut target: size_t,
) {
    (*arr).capacity = target;
    if !(*arr).items.is_null() {
        (*arr).items = realloc(
            (*arr).items as *mut ::core::ffi::c_void,
            (*arr)
                .capacity
                .wrapping_mul(::core::mem::size_of::<otl_GposSingleEntry>() as size_t),
        ) as *mut otl_GposSingleEntry;
    } else {
        (*arr).items = calloc(
            (*arr).capacity,
            ::core::mem::size_of::<otl_GposSingleEntry>() as size_t,
        ) as *mut otl_GposSingleEntry;
    };
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_filterEnv(
    mut arr: *mut subtable_gpos_single,
    mut fn_0: Option<
        unsafe extern "C" fn(*const otl_GposSingleEntry, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: size_t = 0 as size_t;
    let mut k: size_t = 0 as size_t;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut otl_GposSingleEntry,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if gss_typeinfo.dispose.is_some() {
                gss_typeinfo.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut otl_GposSingleEntry,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_init(mut arr: *mut subtable_gpos_single) {
    (*arr).length = 0 as size_t;
    (*arr).capacity = 0 as size_t;
    (*arr).items = ::core::ptr::null_mut::<otl_GposSingleEntry>();
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_disposeItem(
    mut arr: *mut subtable_gpos_single,
    mut n: size_t,
) {
    if gss_typeinfo.dispose.is_some() {
        gss_typeinfo.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut otl_GposSingleEntry
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_sort(
    mut arr: *mut subtable_gpos_single,
    mut fn_0: Option<
        unsafe extern "C" fn(
            *const otl_GposSingleEntry,
            *const otl_GposSingleEntry,
        ) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<otl_GposSingleEntry>() as size_t,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const otl_GposSingleEntry,
                    *const otl_GposSingleEntry,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_fill(mut arr: *mut subtable_gpos_single, mut n: size_t) {
    while (*arr).length < n {
        let mut x: otl_GposSingleEntry = otl_GposSingleEntry {
            target: otfcc_Handle {
                state: HANDLE_STATE_EMPTY,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            value: otl_PositionValue {
                dx: 0.,
                dy: 0.,
                dWidth: 0.,
                dHeight: 0.,
            },
        };
        if gss_typeinfo.init.is_some() {
            gss_typeinfo.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<otl_GposSingleEntry>() as size_t,
            );
        }
        subtable_gpos_single_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_push(
    mut arr: *mut subtable_gpos_single,
    mut elem: otl_GposSingleEntry,
) {
    subtable_gpos_single_grow(arr);
    let fresh0 = (*arr).length;
    (*arr).length = (*arr).length.wrapping_add(1);
    *(*arr).items.offset(fresh0 as isize) = elem;
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_grow(mut arr: *mut subtable_gpos_single) {
    subtable_gpos_single_growTo(arr, (*arr).length.wrapping_add(1 as size_t));
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_growTo(
    mut arr: *mut subtable_gpos_single,
    mut target: size_t,
) {
    if target <= (*arr).capacity {
        return;
    }
    if (*arr).capacity < __CARYLL_VECTOR_INITIAL_SIZE as size_t {
        (*arr).capacity = __CARYLL_VECTOR_INITIAL_SIZE as size_t;
    }
    while (*arr).capacity < target {
        (*arr).capacity = (*arr)
            .capacity
            .wrapping_add((*arr).capacity.wrapping_div(2 as size_t));
    }
    if !(*arr).items.is_null() {
        (*arr).items = realloc(
            (*arr).items as *mut ::core::ffi::c_void,
            (*arr)
                .capacity
                .wrapping_mul(::core::mem::size_of::<otl_GposSingleEntry>() as size_t),
        ) as *mut otl_GposSingleEntry;
    } else {
        (*arr).items = calloc(
            (*arr).capacity,
            ::core::mem::size_of::<otl_GposSingleEntry>() as size_t,
        ) as *mut otl_GposSingleEntry;
    };
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_pop(
    mut arr: *mut subtable_gpos_single,
) -> otl_GposSingleEntry {
    let mut t: otl_GposSingleEntry = *(*arr)
        .items
        .offset((*arr).length.wrapping_sub(1 as size_t) as isize);
    (*arr).length = (*arr).length.wrapping_sub(1 as size_t);
    return t;
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_copyReplace(
    mut dst: *mut subtable_gpos_single,
    src: subtable_gpos_single,
) {
    subtable_gpos_single_dispose(dst);
    subtable_gpos_single_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_copy(
    mut dst: *mut subtable_gpos_single,
    mut src: *const subtable_gpos_single,
) {
    subtable_gpos_single_init(dst);
    subtable_gpos_single_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if gss_typeinfo.copy.is_some() {
        let mut j: size_t = 0 as size_t;
        while j < (*src).length {
            gss_typeinfo.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut otl_GposSingleEntry,
                (*src).items.offset(j as isize) as *mut otl_GposSingleEntry
                    as *const otl_GposSingleEntry,
            );
            j = j.wrapping_add(1);
        }
    } else {
        let mut j_0: size_t = 0 as size_t;
        while j_0 < (*src).length {
            *(*dst).items.offset(j_0 as isize) = *(*src).items.offset(j_0 as isize);
            j_0 = j_0.wrapping_add(1);
        }
    };
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_dispose(mut arr: *mut subtable_gpos_single) {
    if arr.is_null() {
        return;
    }
    if gss_typeinfo.dispose.is_some() {
        let mut j: size_t = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            gss_typeinfo.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut otl_GposSingleEntry,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<otl_GposSingleEntry>();
    (*arr).length = 0 as size_t;
    (*arr).capacity = 0 as size_t;
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_replace(
    mut dst: *mut subtable_gpos_single,
    src: subtable_gpos_single,
) {
    subtable_gpos_single_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<subtable_gpos_single>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_initCapN(
    mut arr: *mut subtable_gpos_single,
    mut n: size_t,
) {
    subtable_gpos_single_init(arr);
    subtable_gpos_single_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_growToN(
    mut arr: *mut subtable_gpos_single,
    mut target: size_t,
) {
    if target <= (*arr).capacity {
        return;
    }
    if (*arr).capacity < __CARYLL_VECTOR_INITIAL_SIZE as size_t {
        (*arr).capacity = __CARYLL_VECTOR_INITIAL_SIZE as size_t;
    }
    if (*arr).capacity < target {
        (*arr).capacity = target.wrapping_add(1 as size_t);
    }
    if !(*arr).items.is_null() {
        (*arr).items = realloc(
            (*arr).items as *mut ::core::ffi::c_void,
            (*arr)
                .capacity
                .wrapping_mul(::core::mem::size_of::<otl_GposSingleEntry>() as size_t),
        ) as *mut otl_GposSingleEntry;
    } else {
        (*arr).items = calloc(
            (*arr).capacity,
            ::core::mem::size_of::<otl_GposSingleEntry>() as size_t,
        ) as *mut otl_GposSingleEntry;
    };
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_initN(mut arr: *mut subtable_gpos_single, mut n: size_t) {
    subtable_gpos_single_init(arr);
    subtable_gpos_single_growToN(arr, n);
    subtable_gpos_single_fill(arr, n);
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_free(mut x: *mut subtable_gpos_single) {
    if x.is_null() {
        return;
    }
    subtable_gpos_single_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_createN(mut n: size_t) -> *mut subtable_gpos_single {
    let mut t: *mut subtable_gpos_single =
        malloc(::core::mem::size_of::<subtable_gpos_single>() as size_t)
            as *mut subtable_gpos_single;
    subtable_gpos_single_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_create() -> *mut subtable_gpos_single {
    let mut x: *mut subtable_gpos_single =
        malloc(::core::mem::size_of::<subtable_gpos_single>() as size_t)
            as *mut subtable_gpos_single;
    subtable_gpos_single_init(x);
    return x;
}
#[no_mangle]
pub static mut iSubtable_gpos_single: __caryll_vectorinterface_subtable_gpos_single = unsafe {
    __caryll_vectorinterface_subtable_gpos_single {
        init: Some(
            subtable_gpos_single_init as unsafe extern "C" fn(*mut subtable_gpos_single) -> (),
        ),
        copy: Some(
            subtable_gpos_single_copy
                as unsafe extern "C" fn(
                    *mut subtable_gpos_single,
                    *const subtable_gpos_single,
                ) -> (),
        ),
        move_0: Some(
            subtable_gpos_single_move
                as unsafe extern "C" fn(*mut subtable_gpos_single, *mut subtable_gpos_single) -> (),
        ),
        dispose: Some(
            subtable_gpos_single_dispose as unsafe extern "C" fn(*mut subtable_gpos_single) -> (),
        ),
        replace: Some(
            subtable_gpos_single_replace
                as unsafe extern "C" fn(*mut subtable_gpos_single, subtable_gpos_single) -> (),
        ),
        copyReplace: Some(
            subtable_gpos_single_copyReplace
                as unsafe extern "C" fn(*mut subtable_gpos_single, subtable_gpos_single) -> (),
        ),
        create: Some(subtable_gpos_single_create),
        free: Some(
            subtable_gpos_single_free as unsafe extern "C" fn(*mut subtable_gpos_single) -> (),
        ),
        initN: Some(
            subtable_gpos_single_initN
                as unsafe extern "C" fn(*mut subtable_gpos_single, size_t) -> (),
        ),
        initCapN: Some(
            subtable_gpos_single_initCapN
                as unsafe extern "C" fn(*mut subtable_gpos_single, size_t) -> (),
        ),
        createN: Some(
            subtable_gpos_single_createN
                as unsafe extern "C" fn(size_t) -> *mut subtable_gpos_single,
        ),
        fill: Some(
            subtable_gpos_single_fill
                as unsafe extern "C" fn(*mut subtable_gpos_single, size_t) -> (),
        ),
        clear: Some(
            subtable_gpos_single_dispose as unsafe extern "C" fn(*mut subtable_gpos_single) -> (),
        ),
        push: Some(
            subtable_gpos_single_push
                as unsafe extern "C" fn(*mut subtable_gpos_single, otl_GposSingleEntry) -> (),
        ),
        shrinkToFit: Some(
            subtable_gpos_single_shrinkToFit
                as unsafe extern "C" fn(*mut subtable_gpos_single) -> (),
        ),
        pop: Some(
            subtable_gpos_single_pop
                as unsafe extern "C" fn(*mut subtable_gpos_single) -> otl_GposSingleEntry,
        ),
        disposeItem: Some(
            subtable_gpos_single_disposeItem
                as unsafe extern "C" fn(*mut subtable_gpos_single, size_t) -> (),
        ),
        filterEnv: Some(
            subtable_gpos_single_filterEnv
                as unsafe extern "C" fn(
                    *mut subtable_gpos_single,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_GposSingleEntry,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            subtable_gpos_single_sort
                as unsafe extern "C" fn(
                    *mut subtable_gpos_single,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_GposSingleEntry,
                            *const otl_GposSingleEntry,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn subtable_gpos_single_shrinkToFit(mut arr: *mut subtable_gpos_single) {
    subtable_gpos_single_resizeTo(arr, (*arr).length);
}
#[no_mangle]
pub unsafe extern "C" fn otl_read_gpos_single(
    data: font_file_pointer,
    mut tableLength: uint32_t,
    mut offset: uint32_t,
    maxGlyphs: glyphid_t,
    mut options: *const otfcc_Options,
) -> *mut otl_Subtable {
    let mut subtableFormat: uint16_t = 0;
    let mut current_block: u64;
    let mut subtable: *mut subtable_gpos_single =
        (
            iSubtable_gpos_single
                .create
                .expect("non-null function pointer"))();
    let mut targets: *mut otl_Coverage = ::core::ptr::null_mut::<otl_Coverage>();
    if !(tableLength < offset.wrapping_add(6 as uint32_t)) {
        subtableFormat = read_16u(data.offset(offset as isize) as *const uint8_t);
        targets = otl_iCoverage.read.expect("non-null function pointer")(
            data as *const uint8_t,
            tableLength,
            offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const uint8_t,
            ) as uint32_t),
        );
        if !(targets.is_null()
            || (*targets).numGlyphs as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
        {
            if subtableFormat as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                let mut v: otl_PositionValue = read_gpos_value(
                    data,
                    tableLength,
                    offset.wrapping_add(6 as uint32_t),
                    read_16u(
                        data.offset(offset as isize)
                            .offset(4 as ::core::ffi::c_int as isize)
                            as *const uint8_t,
                    ),
                );
                let mut j: glyphid_t = 0 as glyphid_t;
                while (j as ::core::ffi::c_int) < (*targets).numGlyphs as ::core::ffi::c_int {
                    iSubtable_gpos_single
                        .push
                        .expect("non-null function pointer")(
                        subtable,
                        otl_GposSingleEntry {
                            target: otfcc_iHandle.dup.expect("non-null function pointer")(
                                *(*targets).glyphs.offset(j as isize) as otfcc_Handle,
                            ) as otfcc_GlyphHandle,
                            value: v,
                        },
                    );
                    j = j.wrapping_add(1);
                }
                current_block = 6009453772311597924;
            } else {
                let mut valueFormat: uint16_t = read_16u(
                    data.offset(offset as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        as *const uint8_t,
                );
                let mut valueCount: uint16_t = read_16u(
                    data.offset(offset as isize)
                        .offset(6 as ::core::ffi::c_int as isize)
                        as *const uint8_t,
                );
                if tableLength
                    < offset.wrapping_add(8 as uint32_t).wrapping_add(
                        (position_format_length(valueFormat) as ::core::ffi::c_int
                            * valueCount as ::core::ffi::c_int) as uint32_t,
                    )
                {
                    current_block = 18154618883129817269;
                } else if valueCount as ::core::ffi::c_int
                    != (*targets).numGlyphs as ::core::ffi::c_int
                {
                    current_block = 18154618883129817269;
                } else {
                    let mut j_0: glyphid_t = 0 as glyphid_t;
                    while (j_0 as ::core::ffi::c_int) < (*targets).numGlyphs as ::core::ffi::c_int {
                        iSubtable_gpos_single
                            .push
                            .expect("non-null function pointer")(
                            subtable,
                            otl_GposSingleEntry {
                                target: otfcc_iHandle.dup.expect("non-null function pointer")(
                                    *(*targets).glyphs.offset(j_0 as isize) as otfcc_Handle,
                                ) as otfcc_GlyphHandle,
                                value: read_gpos_value(
                                    data,
                                    tableLength,
                                    offset.wrapping_add(8 as uint32_t).wrapping_add(
                                        (j_0 as ::core::ffi::c_int
                                            * position_format_length(valueFormat)
                                                as ::core::ffi::c_int)
                                            as uint32_t,
                                    ),
                                    valueFormat,
                                ),
                            },
                        );
                        j_0 = j_0.wrapping_add(1);
                    }
                    current_block = 6009453772311597924;
                }
            }
            match current_block {
                18154618883129817269 => {}
                _ => {
                    if !targets.is_null() {
                        otl_iCoverage.free.expect("non-null function pointer")(targets);
                    }
                    return subtable as *mut otl_Subtable;
                }
            }
        }
    }
    if !targets.is_null() {
        otl_iCoverage.free.expect("non-null function pointer")(targets);
    }
    iSubtable_gpos_single
        .free
        .expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<otl_Subtable>();
}
#[no_mangle]
pub unsafe extern "C" fn otl_gpos_dump_single(
    mut _subtable: *const otl_Subtable,
) -> *mut json_value {
    let mut subtable: *const subtable_gpos_single = &raw const (*_subtable).gpos_single;
    let mut st: *mut json_value = json_object_new((*subtable).length);
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as size_t) < (*subtable).length {
        json_object_push(
            st,
            (*(*subtable).items.offset(j as isize)).target.name as *const ::core::ffi::c_char,
            gpos_dump_value((*(*subtable).items.offset(j as isize)).value),
        );
        j = j.wrapping_add(1);
    }
    return st;
}
#[no_mangle]
pub unsafe extern "C" fn otl_gpos_parse_single(
    mut _subtable: *const json_value,
    mut options: *const otfcc_Options,
) -> *mut otl_Subtable {
    let mut subtable: *mut subtable_gpos_single =
        (
            iSubtable_gpos_single
                .create
                .expect("non-null function pointer"))();
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as ::core::ffi::c_uint) < (*_subtable).u.object.length {
        if !(*(*_subtable).u.object.values.offset(j as isize))
            .value
            .is_null()
            && (*(*(*_subtable).u.object.values.offset(j as isize)).value).type_0
                as ::core::ffi::c_uint
                == json_object as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut gname: sds = sdsnewlen(
                (*(*_subtable).u.object.values.offset(j as isize)).name
                    as *const ::core::ffi::c_void,
                (*(*_subtable).u.object.values.offset(j as isize)).name_length as size_t,
            );
            iSubtable_gpos_single
                .push
                .expect("non-null function pointer")(
                subtable,
                otl_GposSingleEntry {
                    target: otfcc_iHandle.fromName.expect("non-null function pointer")(gname)
                        as otfcc_GlyphHandle,
                    value: gpos_parse_value(
                        (*(*_subtable).u.object.values.offset(j as isize)).value as *mut json_value,
                    ),
                },
            );
        }
        j = j.wrapping_add(1);
    }
    return subtable as *mut otl_Subtable;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_build_gpos_single(
    mut _subtable: *const otl_Subtable,
    mut heuristics: otl_BuildHeuristics,
) -> *mut caryll_Buffer {
    let mut subtable: *const subtable_gpos_single = &raw const (*_subtable).gpos_single;
    let mut isConst: bool = (*subtable).length > 0 as size_t;
    let mut format: uint16_t = 0 as uint16_t;
    if (*subtable).length > 0 as size_t {
        let mut j: glyphid_t = 0 as glyphid_t;
        while (j as size_t) < (*subtable).length {
            isConst = isConst as ::core::ffi::c_int != 0
                && (*(*subtable).items.offset(j as isize)).value.dx
                    == (*(*subtable).items.offset(0 as ::core::ffi::c_int as isize))
                        .value
                        .dx
                && (*(*subtable).items.offset(j as isize)).value.dy
                    == (*(*subtable).items.offset(0 as ::core::ffi::c_int as isize))
                        .value
                        .dy
                && (*(*subtable).items.offset(j as isize)).value.dWidth
                    == (*(*subtable).items.offset(0 as ::core::ffi::c_int as isize))
                        .value
                        .dWidth
                && (*(*subtable).items.offset(j as isize)).value.dHeight
                    == (*(*subtable).items.offset(0 as ::core::ffi::c_int as isize))
                        .value
                        .dHeight;
            format = (format as ::core::ffi::c_int
                | required_position_format((*(*subtable).items.offset(j as isize)).value)
                    as ::core::ffi::c_int) as uint16_t;
            j = j.wrapping_add(1);
        }
    }
    let mut cov: *mut otl_Coverage = (
        otl_iCoverage.create.expect("non-null function pointer"))();
    let mut j_0: glyphid_t = 0 as glyphid_t;
    while (j_0 as size_t) < (*subtable).length {
        otl_iCoverage.push.expect("non-null function pointer")(
            cov,
            otfcc_iHandle.dup.expect("non-null function pointer")(
                (*(*subtable).items.offset(j_0 as isize)).target as otfcc_Handle,
            ) as otfcc_GlyphHandle,
        );
        j_0 = j_0.wrapping_add(1);
    }
    let mut coverageBuf: *mut caryll_Buffer =
        otl_iCoverage.build.expect("non-null function pointer")(cov);
    if isConst {
        let mut b: *mut bk_Block = bk_new_Block(
            b16 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            p16 as ::core::ffi::c_int,
            bk_newBlockFromBuffer(coverageBuf),
            b16 as ::core::ffi::c_int,
            format as ::core::ffi::c_int,
            bkembed as ::core::ffi::c_int,
            bk_gpos_value(
                (*(*subtable).items.offset(0 as ::core::ffi::c_int as isize)).value,
                format,
            ),
            bkover as ::core::ffi::c_int,
        );
        otl_iCoverage.free.expect("non-null function pointer")(cov);
        return bk_build_Block(b);
    } else {
        let mut b_0: *mut bk_Block = bk_new_Block(
            b16 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            p16 as ::core::ffi::c_int,
            bk_newBlockFromBuffer(coverageBuf),
            b16 as ::core::ffi::c_int,
            format as ::core::ffi::c_int,
            b16 as ::core::ffi::c_int,
            (*subtable).length,
            bkover as ::core::ffi::c_int,
        );
        let mut k: glyphid_t = 0 as glyphid_t;
        while (k as size_t) < (*subtable).length {
            bk_push(
                b_0,
                bkembed as ::core::ffi::c_int,
                bk_gpos_value((*(*subtable).items.offset(k as isize)).value, format),
                bkover as ::core::ffi::c_int,
            );
            k = k.wrapping_add(1);
        }
        otl_iCoverage.free.expect("non-null function pointer")(cov);
        return bk_build_Block(b_0);
    };
}
pub const __CARYLL_VECTOR_INITIAL_SIZE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
