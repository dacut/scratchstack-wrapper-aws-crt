use {crate::common::AWS_C_COMMON_PACKAGE_ID, std::ffi::c_void};

pub const AWS_OP_SUCCESS: i32 = 0;
pub const AWS_OP_ERR: i32 = -1;

const AWS_ERROR_ENUM_STRIDE_BITS: usize = 10;
const AWS_ERROR_ENUM_STRIDE: isize = 1 << AWS_ERROR_ENUM_STRIDE_BITS;

pub const fn aws_error_enum_begin_range(x: isize) -> isize {
    x * AWS_ERROR_ENUM_STRIDE
}

pub const fn aws_error_enum_end_range(x: isize) -> isize {
    (x + 1) * AWS_ERROR_ENUM_STRIDE - 1
}

#[repr(C)]
pub struct AwsCErrorInfo {
    pub error_code: i32,
    pub literal_name: *const u8,
    pub error_str: *const u8,
    pub lib_name: *const u8,
    pub formatted_name: *const u8,
}

#[repr(C)]
pub struct AwsCErrorInfoList {
    pub error_list: *const AwsCErrorInfo,
    pub count: u16,
}

type AwsErrorHandlerFn = extern "C" fn(err: i32, ctx: *mut c_void);

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum AwsCommonError {
    AWS_ERROR_SUCCESS = aws_error_enum_begin_range(AWS_C_COMMON_PACKAGE_ID),
    AWS_ERROR_OOM,
    AWS_ERROR_NO_SPACE,
    AWS_ERROR_UNKNOWN,
    AWS_ERROR_SHORT_BUFFER,
    AWS_ERROR_OVERFLOW_DETECTED,
    AWS_ERROR_UNSUPPORTED_OPERATION,
    AWS_ERROR_INVALID_BUFFER_SIZE,
    AWS_ERROR_INVALID_HEX_STR,
    AWS_ERROR_INVALID_BASE64_STR,
    AWS_ERROR_INVALID_INDEX,
    AWS_ERROR_THREAD_INVALID_SETTINGS,
    AWS_ERROR_THREAD_INSUFFICIENT_RESOURCE,
    AWS_ERROR_THREAD_NO_PERMISSIONS,
    AWS_ERROR_THREAD_NOT_JOINABLE,
    AWS_ERROR_THREAD_NO_SUCH_THREAD_ID,
    AWS_ERROR_THREAD_DEADLOCK_DETECTED,
    AWS_ERROR_MUTEX_NOT_INIT,
    AWS_ERROR_MUTEX_TIMEOUT,
    AWS_ERROR_MUTEX_CALLER_NOT_OWNER,
    AWS_ERROR_MUTEX_FAILED,
    AWS_ERROR_COND_VARIABLE_INIT_FAILED,
    AWS_ERROR_COND_VARIABLE_TIMED_OUT,
    AWS_ERROR_COND_VARIABLE_ERROR_UNKNOWN,
    AWS_ERROR_CLOCK_FAILURE,
    AWS_ERROR_LIST_EMPTY,
    AWS_ERROR_DEST_COPY_TOO_SMALL,
    AWS_ERROR_LIST_EXCEEDS_MAX_SIZE,
    AWS_ERROR_LIST_STATIC_MODE_CANT_SHRINK,
    AWS_ERROR_PRIORITY_QUEUE_FULL,
    AWS_ERROR_PRIORITY_QUEUE_EMPTY,
    AWS_ERROR_PRIORITY_QUEUE_BAD_NODE,
    AWS_ERROR_HASHTBL_ITEM_NOT_FOUND,
    AWS_ERROR_INVALID_DATE_STR,
    AWS_ERROR_INVALID_ARGUMENT,
    AWS_ERROR_RANDOM_GEN_FAILED,
    AWS_ERROR_MALFORMED_INPUT_STRING,
    AWS_ERROR_UNIMPLEMENTED,
    AWS_ERROR_INVALID_STATE,
    AWS_ERROR_ENVIRONMENT_GET,
    AWS_ERROR_ENVIRONMENT_SET,
    AWS_ERROR_ENVIRONMENT_UNSET,
    AWS_ERROR_STREAM_UNSEEKABLE,
    AWS_ERROR_NO_PERMISSION,
    AWS_ERROR_FILE_INVALID_PATH,
    AWS_ERROR_MAX_FDS_EXCEEDED,
    AWS_ERROR_SYS_CALL_FAILURE,
    AWS_ERROR_C_STRING_BUFFER_NOT_NULL_TERMINATED,
    AWS_ERROR_STRING_MATCH_NOT_FOUND,
    AWS_ERROR_DIVIDE_BY_ZERO,
    AWS_ERROR_INVALID_FILE_HANDLE,
    AWS_ERROR_OPERATION_INTERUPTED,
    AWS_ERROR_DIRECTORY_NOT_EMPTY,
    AWS_ERROR_PLATFORM_NOT_SUPPORTED,
    AWS_ERROR_END_COMMON_RANGE = aws_error_enum_end_range(AWS_C_COMMON_PACKAGE_ID),
}

#[link(name = "aws-c-common")]
extern "C" {
    pub fn aws_last_error() -> i32;
    pub fn aws_error_str(err: i32) -> *const i8;
    pub fn aws_error_name(err: i32) -> *const i8;
    pub fn aws_error_lib_name(err: i32) -> *const i8;
    pub fn aws_error_debug_str(err: i32) -> *const i8;
    pub fn aws_raise_error(err: i32) -> i32;
    pub fn aws_reset_error();
    pub fn aws_restore_error(err: i32);
    pub fn aws_set_global_error_handler_fn(
        handler: *const AwsErrorHandlerFn,
        ctx: *mut c_void,
    ) -> *const AwsErrorHandlerFn;
    pub fn aws_set_thread_local_error_handler_fn(
        handler: *const AwsErrorHandlerFn,
        ctx: *mut c_void,
    ) -> *const AwsErrorHandlerFn;
    pub fn aws_translate_and_raise_io_error(error_no: i32) -> i32;
}
