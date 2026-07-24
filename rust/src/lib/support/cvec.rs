// Generic implementation of otfcc's C-style growable vector
// (`length`/`capacity`/`items: *mut T`), factored out of the per-container
// boilerplate (`subtable_gsub_multi_growTo`, `_resizeTo`, `_grow`, `_init`,
// `_push`, `_pop`, `_move`, and their ~65 siblings across every other
// container type in the crate). c2rust generated one full copy of this
// arithmetic per container element type, since the original C used an
// X-macro to stamp out a distinct named struct + function family per type
// rather than a single generic implementation.
//
// `CVecRaw<T>` is layout-compatible with every one of those per-type
// structs (`#[repr(C)] { length: size_t, capacity: size_t, items: *mut T }`,
// in that field order) -- callers cast their own container's pointer to
// `*mut CVecRaw<ElementType>` to use these functions, without needing to
// change the original struct's definition (which stays independently
// duplicated across dozens of files, exactly as c2rust emitted it; only the
// *implementation* of the handful of functions that operate on it changes).
//
// Only the callback-free, purely structural operations are generic here:
// growth/capacity math, push/pop by value, and move. Operations that need
// per-element behavior (custom dispose via a container's own `typeinfo`
// struct, custom init, sort/filterEnv taking element-specific function
// pointers) stay written out per container, since genericizing callback
// dispatch is a separate, larger design question than this arithmetic.
pub type size_t = usize;

extern "C" {
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
}

const INITIAL_SIZE: size_t = 2;

#[repr(C)]
pub(crate) struct CVecRaw<T> {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut T,
}
// Copy/Clone regardless of T: every field is either a plain integer or a
// raw pointer, both Copy unconditionally -- matches the semantics of the
// original `#[derive(Copy, Clone)]` per-type structs.
impl<T> Clone for CVecRaw<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for CVecRaw<T> {}

#[inline]
pub(crate) unsafe fn cvec_init<T>(arr: *mut CVecRaw<T>) {
    (*arr).length = 0;
    (*arr).capacity = 0;
    (*arr).items = ::core::ptr::null_mut();
}

#[inline]
pub(crate) unsafe fn cvec_grow_to<T>(arr: *mut CVecRaw<T>, target: size_t) {
    if target <= (*arr).capacity {
        return;
    }
    if (*arr).capacity < INITIAL_SIZE {
        (*arr).capacity = INITIAL_SIZE;
    }
    while (*arr).capacity < target {
        (*arr).capacity = (*arr).capacity.wrapping_add((*arr).capacity.wrapping_div(2));
    }
    cvec_realloc_items(arr);
}

#[inline]
pub(crate) unsafe fn cvec_grow_to_n<T>(arr: *mut CVecRaw<T>, target: size_t) {
    if target <= (*arr).capacity {
        return;
    }
    if (*arr).capacity < INITIAL_SIZE {
        (*arr).capacity = INITIAL_SIZE;
    }
    if (*arr).capacity < target {
        (*arr).capacity = target.wrapping_add(1);
    }
    cvec_realloc_items(arr);
}

#[inline]
pub(crate) unsafe fn cvec_resize_to<T>(arr: *mut CVecRaw<T>, target: size_t) {
    (*arr).capacity = target;
    cvec_realloc_items(arr);
}

#[inline]
unsafe fn cvec_realloc_items<T>(arr: *mut CVecRaw<T>) {
    let bytes = (*arr).capacity.wrapping_mul(::core::mem::size_of::<T>());
    (*arr).items = if !(*arr).items.is_null() {
        realloc((*arr).items as *mut ::core::ffi::c_void, bytes) as *mut T
    } else {
        calloc((*arr).capacity, ::core::mem::size_of::<T>()) as *mut T
    };
}

#[inline]
pub(crate) unsafe fn cvec_grow<T>(arr: *mut CVecRaw<T>) {
    cvec_grow_to(arr, (*arr).length.wrapping_add(1));
}

#[inline]
pub(crate) unsafe fn cvec_push<T: Copy>(arr: *mut CVecRaw<T>, elem: T) {
    cvec_grow(arr);
    let fresh = (*arr).length;
    (*arr).length = (*arr).length.wrapping_add(1);
    *(*arr).items.offset(fresh as isize) = elem;
}

#[inline]
pub(crate) unsafe fn cvec_pop<T: Copy>(arr: *mut CVecRaw<T>) -> T {
    let t = *(*arr).items.offset((*arr).length.wrapping_sub(1) as isize);
    (*arr).length = (*arr).length.wrapping_sub(1);
    t
}

#[inline]
pub(crate) unsafe fn cvec_move<T>(dst: *mut CVecRaw<T>, src: *mut CVecRaw<T>) {
    *dst = *src;
    cvec_init(src);
}
