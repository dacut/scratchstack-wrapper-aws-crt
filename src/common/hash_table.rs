use {
    std::ffi::c_void,
    crate::common::AwsCAllocator,
};

pub const AWS_COMMON_HASH_TABLE_ITER_CONTINUE: usize = 1 << 0;
pub const AWS_COMMON_HASH_TABLE_ITER_DELETE: usize = 1 << 1;
pub const AWS_COMMON_HASH_TABLE_ITER_ERROR: usize = 1 << 2;

#[repr(C)]
struct AwsCHashTableEntry {
    element: AwsCHashElement,
    hash_code: u64,
}

#[repr(C)]
pub struct AwsCHashTableState {
    hash_fn: *const AwsCHashFn,
    equals_fn: *const AwsCHashCallbackEqFn,
    destroy_key_fn: *const AwsCHashCallbackDestroyFn,
    destroy_value_fn: *const AwsCHashCallbackDestroyFn,
    alloc: *const AwsCAllocator,
    size: usize,
    entry_count: usize,
    max_load: usize,
    mask: usize,
    max_load_factor: f64,
    slots: [AwsCHashTableEntry; 1],
}

#[repr(C)]
pub struct AwsCHashTable {
    p_impl: *mut AwsCHashTableState,
}

#[repr(C)]
pub struct AwsCHashElement {
    pub key: *const c_void,
    pub value: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AwsCHashIterStatus {
    Done,
    DeleteCalled,
    ReadyForUse,
}

#[repr(C)]
pub struct AwsCHashIter {
    pub map: *const AwsCHashTable,
    pub element: AwsCHashElement,
    pub slot: usize,
    pub status: AwsCHashIterStatus,
    unused_0: i32,
    unused_1: *mut c_void,
    unused_2: *mut c_void,
}

pub type AwsCHashFn = extern "C" fn(key: *const c_void) -> u64;
pub type AwsCHashCallbackEqFn = extern "C" fn(a: *const c_void, b: *const c_void) -> bool;
pub type AwsCHashCallbackDestroyFn = extern "C" fn(key_or_value: *mut c_void);

#[link(name = "aws-c-common")]
extern "C" {
    #[must_use = "returns an i32 that contains a result code (AWS_OP_SUCCESS or AWS_OP_ERR)"]
    pub fn aws_hash_table_init(map: *mut AwsCHashTable, alloc: *const AwsCAllocator, size: usize, hash_fn: *const AwsCHashFn, equals_fn: *const AwsCHashCallbackEqFn, destroy_key_fn: *const AwsCHashCallbackDestroyFn, destroy_value_fn: *const AwsCHashCallbackDestroyFn) -> i32;

    pub fn aws_hash_table_clean_up(map: *mut AwsCHashTable);

    pub fn aws_hash_table_swap(a: *mut AwsCHashTable, b: *mut AwsCHashTable);

    pub fn aws_hash_table_move(to: *mut AwsCHashTable, from: *mut AwsCHashTable);

    pub fn aws_hash_table_get_entry_count(map: *const AwsCHashTable) -> usize;

    pub fn aws_hash_iter_begin(map: *const AwsCHashTable) -> AwsCHashIter;

    pub fn aws_hash_iter_done(iter: *const AwsCHashIter) -> bool;

    pub fn aws_hash_iter_next(iter: *mut AwsCHashIter);

    pub fn aws_hash_iter_delete(iter: *mut AwsCHashIter, destroy_contents: bool);

    #[must_use = "returns an i32 that contains a result code (AWS_OP_SUCCESS or AWS_OP_ERR)"]
    pub fn aws_hash_table_find(map: *const AwsCHashTable, key: *const c_void, p_elem: *mut *mut AwsCHashElement) -> i32;

    #[must_use = "returns an i32 that contains a result code (AWS_OP_SUCCESS or AWS_OP_ERR)"]
    pub fn aws_hash_table_create(map: *mut AwsCHashTable, key: *const c_void, p_elem: *mut *mut AwsCHashElement, was_created: *mut i32) -> i32;

    #[must_use = "returns an i32 that contains a result code (AWS_OP_SUCCESS or AWS_OP_ERR)"]
    pub fn aws_hash_table_put(map: *mut AwsCHashTable, key: *const c_void, value: *mut c_void, was_created: *mut i32) -> i32;

    pub fn aws_hash_table_remove(map: *mut AwsCHashTable, key: *const c_void, p_value: *mut AwsCHashElement, was_present: *mut i32) -> i32;

    pub fn aws_hash_table_remove_element(map: *mut AwsCHashTable, p_value: *mut AwsCHashElement) -> i32;

    pub fn aws_hash_table_foreach(map: *mut AwsCHashTable, callback: *const extern "C" fn(context: *mut c_void, p_element: *mut AwsCHashElement) -> i32, context: *mut c_void) -> i32;

    pub fn aws_hash_table_eq(a: *const AwsCHashTable, b: *const AwsCHashTable, value_eq: *const AwsCHashCallbackEqFn) -> bool;

    pub fn aws_hash_table_clear(map: *mut AwsCHashTable);

    pub fn aws_hash_c_string(item: *const c_void) -> u64;

    pub fn aws_hash_string(item: *const c_void) -> u64;

    pub fn aws_hash_byte_cursor_ptr(item: *const c_void) -> u64;

    pub fn aws_hash_ptr(item: *const c_void) -> u64;

    pub fn aws_hash_combine(item1: u64, item2: u64) -> u64;

    pub fn aws_hash_callback_c_str_eq(a: *const c_void, b: *const c_void) -> bool;

    pub fn aws_hash_callback_string_eq(a: *const c_void, b: *const c_void) -> bool;

    pub fn aws_hash_callback_string_destroy(a: *mut c_void);

    pub fn aws_ptr_eq(a: *const c_void, b: *const c_void) -> bool;

    pub fn aws_hash_table_is_valid(map: *const AwsCHashTable) -> bool;

    pub fn aws_hash_iter_is_valid(iter: *const AwsCHashIter) -> bool;
}