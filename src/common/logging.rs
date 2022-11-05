use {
    crate::common::{AwsCAllocator, AwsCString, AwsDateFormat, AWS_C_COMMON_PACKAGE_ID},
    std::ffi::c_void,
};

pub const AWS_LOG_LEVEL_NONE: isize = 0;
pub const AWS_LOG_LEVEL_FATAL: isize = 1;
pub const AWS_LOG_LEVEL_ERROR: isize = 2;
pub const AWS_LOG_LEVEL_WARN: isize = 3;
pub const AWS_LOG_LEVEL_INFO: isize = 4;
pub const AWS_LOG_LEVEL_DEBUG: isize = 5;
pub const AWS_LOG_LEVEL_TRACE: isize = 6;

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AwsLogLevel {
    AWS_LL_NONE = AWS_LOG_LEVEL_NONE,
    AWS_LL_FATAL = AWS_LOG_LEVEL_FATAL,
    AWS_LL_ERROR = AWS_LOG_LEVEL_ERROR,
    AWS_LL_WARN = AWS_LOG_LEVEL_WARN,
    AWS_LL_INFO = AWS_LOG_LEVEL_INFO,
    AWS_LL_DEBUG = AWS_LOG_LEVEL_DEBUG,
    AWS_LL_TRACE = AWS_LOG_LEVEL_TRACE,
    AWS_LL_COUNT,
}

pub type AwsLogSubject = u32;

pub const AWS_LOG_SUBJECT_STRIDE_BITS: usize = 10;
pub const AWS_LOG_SUBJECT_STRIDE: isize = 1 << AWS_LOG_SUBJECT_STRIDE_BITS;
pub const fn aws_log_subject_begin_range(x: isize) -> isize {
    x * AWS_LOG_SUBJECT_STRIDE
}

pub const fn aws_log_subject_end_range(x: isize) -> isize {
    (x + 1) * AWS_LOG_SUBJECT_STRIDE - 1
}

#[repr(C)]
pub struct AwsCLogSubjectInfo {
    pub subject_id: AwsLogSubject,
    pub subject_name: *const u8,
    pub subject_description: *const u8,
}

#[repr(C)]
pub struct AwsCLogSubjectInfoList {
    pub subject_list: *const AwsCLogSubjectInfo,
    pub count: usize,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum AwsCommonLogSubject {
    AWS_LS_COMMON_GENERAL = aws_log_subject_begin_range(AWS_C_COMMON_PACKAGE_ID),
    AWS_LS_COMMON_TASK_SCHEDULER,
    AWS_LS_COMMON_THREAD,
    AWS_LS_COMMON_MEMTRACE,
    AWS_LS_COMMON_XML_PARSER,
    AWS_LS_COMMON_IO,
    AWS_LS_COMMON_BUS,
    AWS_LS_COMMON_TEST,
    AWS_LS_COMMON_JSON_PARSER,
    AWS_LS_COMMON_LAST = aws_log_subject_end_range(AWS_C_COMMON_PACKAGE_ID),
}

#[repr(C)]
pub struct AwsCLoggerVtable {
    pub log: *const extern "C" fn(*const AwsCLogger, AwsLogLevel, AwsLogSubject, *const u8, ...) -> i32,
    pub get_log_level: *const extern "C" fn(*const AwsCLogger, AwsLogSubject) -> AwsLogLevel,
    pub clean_up: *const extern "C" fn(*const AwsCLogger),
    pub set_log_level: *const extern "C" fn(*const AwsCLogger, AwsLogLevel) -> i32,
}

#[repr(C)]
pub struct AwsCLogger {
    vtable: *const AwsCLoggerVtable,
    allocator: *const AwsCAllocator,
    p_impl: *mut c_void,
}

#[repr(C)]
pub struct AwsCLoggerStandardOptions {
    pub level: AwsLogLevel,
    pub filename: *const u8,
    pub file: *mut libc::FILE,
}

#[repr(C)]
pub struct AwsLogWriterVtable {
    pub write: *const extern "C" fn(writer: *const AwsCLogWriter, output: *const AwsCString) -> i32,
    pub clean_up: *const extern "C" fn(writer: *const AwsCLogWriter),
}

#[repr(C)]
pub struct AwsCLogChannelVtable {
    pub send: *const extern "C" fn(channel: *const AwsCLogChannel, output: *const AwsCString) -> i32,
    pub clean_up: *const extern "C" fn(channel: *const AwsCLogChannel),
}

#[repr(C)]
pub struct AwsCLogChannel {
    pub vtable: *const AwsCLogChannelVtable,
    pub allocator: *const AwsCAllocator,
    pub writer: *const AwsCLogWriter,
    pub r#impl: *mut c_void,
}

#[repr(C)]
pub struct AwsCLogFormatterVtable {
    pub format: *const extern "C" fn(
        formatter: *const AwsCLogFormatter,
        formatted_output: *const *mut AwsCString,
        level: AwsLogLevel,
        subject: AwsLogSubject,
        format: *const u8,
        args: *mut c_void,
    ) -> i32,
    pub clean_up: *const extern "C" fn(formatter: *const AwsCLogFormatter),
}

#[repr(C)]
pub struct AwsCLogFormatter {
    pub vtable: *const AwsCLogFormatterVtable,
    pub allocator: *const AwsCAllocator,
    pub r#impl: *mut c_void,
}

#[repr(C)]
pub struct AwsLogFormatterStandardOptions {
    pub date_format: AwsDateFormat,
}

#[repr(C)]
pub struct AwsCLoggingStandardFormattingData {
    pub log_line_format: *mut u8,
    pub total_length: usize,
    pub level: AwsLogLevel,
    pub suject_name: *const u8,
    pub format: *const u8,
    pub date_format: AwsDateFormat,
    pub allocator: *const AwsCAllocator,
    pub amount_written: usize,
}

#[repr(C)]
pub struct AwsCLogWriter {
    pub vtable: *const AwsLogWriterVtable,
    pub allocator: *const AwsCAllocator,
    pub r#impl: *mut c_void,
}

#[repr(C)]
pub struct AwsCLogWriterFileOptions {
    pub filename: *const u8,
    pub file: *mut libc::FILE,
}

#[link(name = "aws-c-common")]
extern "C" {
    pub fn aws_logger_set(logger: *const AwsCLogger);

    pub fn aws_logger_get() -> *const AwsCLogger;

    pub fn aws_logger_get_conditional(subject: AwsLogSubject, level: AwsLogLevel) -> *const AwsCLogger;

    pub fn aws_logger_clean_up(logger: *const AwsCLogger);

    #[must_use = "returns an i32 that contains a result code (AWS_OP_SUCCESS or AWS_OP_ERR)"]
    pub fn aws_logger_set_log_level(logger: *const AwsCLogger, level: AwsLogLevel) -> i32;

    #[must_use = "returns an i32 that contains a result code (AWS_OP_SUCCESS or AWS_OP_ERR)"]
    pub fn aws_log_level_to_string(log_level: AwsLogLevel, level_string: *mut *const u8) -> i32;

    #[must_use = "returns an i32 that contains a result code (AWS_OP_SUCCESS or AWS_OP_ERR)"]
    pub fn aws_string_to_log_level(level_string: *const u8, log_level: *mut AwsLogLevel) -> i32;

    // pub fn aws_thread_id_t_to_string(thread_id: AwsThreadId, buffer: *mut u8, bufsz: usize) -> i32;

    pub fn aws_log_subject_name(subject: AwsLogSubject) -> *const u8;

    pub fn aws_register_log_subject_info_list(log_subject_list: *const AwsCLogSubjectInfoList);

    pub fn aws_unregister_log_subject_info_list(log_subject_list: *const AwsCLogSubjectInfoList);

    #[must_use = "returns an i32 that contains a result code (AWS_OP_SUCCESS or AWS_OP_ERR)"]
    pub fn aws_logger_init_standard(
        logger: *const AwsCLogger,
        allocator: *const AwsCAllocator,
        options: *const AwsCLoggerStandardOptions,
    ) -> i32;

    #[must_use = "returns an i32 that contains a result code (AWS_OP_SUCCESS or AWS_OP_ERR)"]
    pub fn aws_logger_init_from_external(
        logger: *const AwsCLogger,
        allocator: *const AwsCAllocator,
        formatter: *const AwsCLogFormatter,
        channel: *const AwsCLogChannel,
        writer: *const AwsCLogWriter,
        level: AwsLogLevel,
    ) -> i32;

    #[must_use = "returns an i32 that contains a result code (AWS_OP_SUCCESS or AWS_OP_ERR)"]
    pub fn aws_logger_init_noalloc(
        logger: *const AwsCLogger,
        allocator: *const AwsCAllocator,
        options: *const AwsCLoggerStandardOptions,
    ) -> i32;

    #[must_use = "returns an i32 that contains a result code (AWS_OP_SUCCESS or AWS_OP_ERR)"]
    pub fn aws_log_channel_init_foreground(
        channel: *const AwsCLogChannel,
        allocator: *const AwsCAllocator,
        writer: *const AwsCLogWriter,
    ) -> i32;

    #[must_use = "returns an i32 that contains a result code (AWS_OP_SUCCESS or AWS_OP_ERR)"]
    pub fn aws_log_channel_init_background(
        channel: *const AwsCLogChannel,
        allocator: *const AwsCAllocator,
        writer: *const AwsCLogWriter,
    ) -> i32;

    pub fn aws_log_channel_clean_up(channel: *const AwsCLogChannel);

    #[must_use = "returns an i32 that contains a result code (AWS_OP_SUCCESS or AWS_OP_ERR)"]
    pub fn aws_log_formatter_init_default(
        formatter: *const AwsCLogFormatter,
        allocator: *const AwsCAllocator,
        options: *const AwsLogFormatterStandardOptions,
    ) -> i32;

    pub fn aws_log_formatter_clean_up(formatter: *const AwsCLogFormatter);

    #[must_use = "returns an i32 that contains a result code (AWS_OP_SUCCESS or AWS_OP_ERR)"]
    pub fn aws_format_standard_log_line(
        formatting_data: *mut AwsCLoggingStandardFormattingData,
        args: *mut c_void,
    ) -> i32;

    #[must_use = "returns an i32 that contains a result code (AWS_OP_SUCCESS or AWS_OP_ERR)"]
    pub fn aws_log_writer_init_stdout(writer: *const AwsCLogWriter, allocator: *const AwsCAllocator) -> i32;

    #[must_use = "returns an i32 that contains a result code (AWS_OP_SUCCESS or AWS_OP_ERR)"]
    pub fn aws_log_writer_init_stderr(writer: *const AwsCLogWriter, allocator: *const AwsCAllocator) -> i32;

    #[must_use = "returns an i32 that contains a result code (AWS_OP_SUCCESS or AWS_OP_ERR)"]
    pub fn aws_log_writer_init_file(
        writer: *const AwsCLogWriter,
        allocator: *const AwsCAllocator,
        options: *const AwsCLogWriterFileOptions,
    ) -> i32;

    pub fn aws_log_writer_clean_up(writer: *const AwsCLogWriter);
}
