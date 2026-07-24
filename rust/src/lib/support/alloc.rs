// Shared zero-initialized alloc/realloc helpers, factored out of the ~50
// per-file private copies c2rust emitted (one per translation unit that
// #included c/lib/support/mem.h's `NEW_CLEAN`/`RENEW_CLEAN` macros). These
// were never externally linked (no #[no_mangle]) even in their per-file
// form, so consolidating them changes no ABI -- only support/bk (the files
// already reviewed for the idiomatization pass) have been migrated to use
// this module so far; the remaining ~47 files still carry their own private
// copy pending a future, wider pass.
pub type size_t = usize;

// fprintf/stderr must be declared with the exact same signature used
// elsewhere in the crate (*mut FILE, not an opaque *mut c_void) — extern
// "C" declarations are resolved by symbol name across the whole crate, and
// rustc's clashing_extern_declarations lint flags any mismatch, even
// between unrelated files that never call into each other directly. FILE's
// full layout is never touched here (stderr is only ever passed through to
// fprintf), so this mirrors the boilerplate every other translation unit
// already carries rather than introducing a distinct-but-compatible type.
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}

#[cfg(target_os = "macos")]
extern "C" {
    #[link_name = "__stderrp"]
    static mut stderr: *mut FILE;
}
#[cfg(not(target_os = "macos"))]
extern "C" {
    static mut stderr: *mut FILE;
}
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
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
pub type FILE = _IO_FILE;

const EXIT_FAILURE: ::core::ffi::c_int = 1;

#[inline]
pub(crate) unsafe fn __caryll_allocate_clean(
    n: size_t,
    line: ::core::ffi::c_ulong,
) -> *mut ::core::ffi::c_void {
    if n == 0 {
        return ::core::ptr::null_mut();
    }
    let p = calloc(n, 1);
    if p.is_null() {
        fprintf(
            stderr,
            b"[%ld]Out of memory(%ld bytes)\n\0" as *const u8 as *const ::core::ffi::c_char,
            line,
            n as ::core::ffi::c_ulong,
        );
        exit(EXIT_FAILURE);
    }
    p
}

#[inline]
pub(crate) unsafe fn __caryll_reallocate(
    ptr: *mut ::core::ffi::c_void,
    n: size_t,
    line: ::core::ffi::c_ulong,
) -> *mut ::core::ffi::c_void {
    if n == 0 {
        free(ptr);
        return ::core::ptr::null_mut();
    }
    if ptr.is_null() {
        return __caryll_allocate_clean(n, line);
    }
    let p = realloc(ptr, n);
    if p.is_null() {
        fprintf(
            stderr,
            b"[%ld]Out of memory(%ld bytes)\n\0" as *const u8 as *const ::core::ffi::c_char,
            line,
            n as ::core::ffi::c_ulong,
        );
        exit(EXIT_FAILURE);
    }
    p
}
