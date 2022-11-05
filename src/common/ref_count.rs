use {
    std::ffi::c_void,
    crate::common::AwsCAtomicVar,
};

pub type AwsCSimpleCompletionCallback = extern "C" fn(*mut c_void);

#[repr(C)]
pub struct AwsCRefCount {
    pub ref_count: AwsCAtomicVar,
    pub object: *mut c_void,
    pub on_zero_fn: AwsCSimpleCompletionCallback,
}

#[repr(C)]
pub struct AwsCShutdownCallbackOptions {
    pub shutdown_callback_fn: *mut AwsCSimpleCompletionCallback,
    pub shutdown_callback_user_data: *mut c_void,
}

#[link(name = "aws-c-common")]
extern "C" {
    pub fn aws_ref_count_init(
        ref_count: *mut AwsCRefCount,
        object: *mut c_void,
        on_zero_fn: AwsCSimpleCompletionCallback,
    );

    pub fn aws_ref_count_acquire(ref_count: *mut AwsCRefCount) -> *mut c_void;

    pub fn aws_ref_count_release(ref_count: *mut AwsCRefCount) -> usize;
}
