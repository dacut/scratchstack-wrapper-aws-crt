#[repr(C)]
pub struct AwsCMutex {
    #[cfg(windows)]
    pub mutex_handle: *mut std::ffi::c_void,

    #[cfg(not(windows))]
    pub mutex_handle: libc::pthread_mutex_t,

    initialized: bool,
}

#[link(name = "aws-c-common")]
extern "C" {
    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_mutex_init(mutex: *mut AwsCMutex) -> i32;

    pub fn aws_mutex_clean_up(mutex: *mut AwsCMutex);

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_mutex_lock(mutex: *mut AwsCMutex) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_mutex_try_lock(mutex: *mut AwsCMutex) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_mutex_unlock(mutex: *mut AwsCMutex) -> i32;
}
