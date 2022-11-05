use {
    crate::common::AwsCAllocator,
    std::ffi::c_void,
};

#[repr(C)]
pub struct AwsCArrayList {
    pub alloc: *const AwsCAllocator,
    pub current_size: usize,
    pub length: usize,
    pub item_size: usize,
    pub data: *mut c_void,
}

pub type AwsCArrayListComparatorFn = extern "C" fn(a: *const c_void, b: *const c_void) -> i32;

// TODO: Implement functions.
