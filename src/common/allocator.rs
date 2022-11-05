use std::ffi::c_void;

pub enum AwsAllocator {
    Owned(AwsCAllocator),
    Borrowed(*const AwsCAllocator),
}

impl AwsAllocator {
    pub fn as_ptr(&self) -> *const AwsCAllocator {
        match self {
            Self::Owned(alloc) => alloc,
            Self::Borrowed(alloc) => *alloc,
        }
    }
}

unsafe impl Send for AwsAllocator {}
unsafe impl Sync for AwsAllocator {}

impl Default for AwsAllocator {
    fn default() -> Self {
        Self::Borrowed(unsafe { aws_default_allocator() })
    }
}

#[repr(C)]
pub struct AwsCAllocator {
    pub mem_acquire: extern "C" fn(*const AwsCAllocator, usize) -> *mut c_void,
    pub mem_release: extern "C" fn(*const AwsCAllocator, *mut c_void),
    pub mem_realloc: extern "C" fn(*const AwsCAllocator, *mut c_void, usize, usize) -> *mut c_void,
    pub mem_calloc: extern "C" fn(*const AwsCAllocator, usize, usize) -> *mut c_void,
    r#impl: *mut c_void,
}

unsafe impl Send for AwsCAllocator {}
unsafe impl Sync for AwsCAllocator {}

#[repr(C)]
pub enum AwsMemTraceLevel {
    None = 0,
    Bytes = 1,
    Stacks = 2,
}

#[link(name = "aws-c-common")]
extern "C" {
    pub fn aws_allocator_is_valid(allocator: *const AwsCAllocator) -> bool;
    pub fn aws_default_allocator() -> *const AwsCAllocator;
    pub fn aws_mem_acquire(allocator: *const AwsCAllocator, size: usize) -> *mut c_void;
    pub fn aws_mem_calloc(allocator: *const AwsCAllocator, num: usize, size: usize) -> *mut c_void;
    pub fn aws_mem_acquire_many(allocator: *const AwsCAllocator, count: usize, ...) -> *mut c_void;
    pub fn aws_mem_release(allocator: *const AwsCAllocator, ptr: *mut c_void);
    pub fn aws_mem_realloc(
        allocator: *const AwsCAllocator,
        ptr: *mut *mut c_void,
        oldsize: usize,
        newsize: usize,
    ) -> i32;
    pub fn aws_mem_tracer_new(
        allocator: *const AwsCAllocator,
        deprecated: *const c_void,
        level: AwsMemTraceLevel,
        frames_per_stack: usize,
    ) -> *const AwsCAllocator;
    pub fn aws_mem_tracer_destroy(allocator: *const AwsCAllocator) -> *const AwsCAllocator;
    pub fn aws_mem_tracer_dump(allocator: *const AwsCAllocator);
    pub fn aws_mem_tracer_bytes(allocator: *const AwsCAllocator) -> usize;
    pub fn aws_mem_tracer_count(allocator: *const AwsCAllocator) -> usize;
    pub fn aws_small_block_allocator_new(allocator: *const AwsCAllocator, multi_threaded: bool)
        -> *const AwsCAllocator;
    pub fn aws_small_block_allocator_destroy(allocator: *const AwsCAllocator);
    pub fn aws_small_block_allocator_bytes_active(allocator: *const AwsCAllocator) -> usize;
    pub fn aws_small_block_allocator_bytes_reserved(allocator: *const AwsCAllocator) -> usize;
    pub fn aws_small_block_allocator_page_size(allocator: *const AwsCAllocator) -> usize;
}
