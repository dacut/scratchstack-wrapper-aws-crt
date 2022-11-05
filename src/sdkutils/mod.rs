use crate::common::{
    aws_error_enum_begin_range, aws_error_enum_end_range, aws_log_subject_begin_range, aws_log_subject_end_range,
    AwsCAllocator,
};

pub const AWS_C_SDKUTILS_PACKAGE_ID: isize = 15;

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum AwsSdkUtilsErrors {
    AWS_ERROR_SDKUTILS_GENERAL = aws_error_enum_begin_range(AWS_C_SDKUTILS_PACKAGE_ID),
    AWS_ERROR_SDKUTILS_PARSE_FATAL,
    AWS_ERROR_SDKUTILS_PARSE_RECOVERABLE,
    AWS_ERROR_SDKUTILS_ENDPOINTS_UNSUPPORTED_RULESET,
    AWS_ERROR_SDKUTILS_ENDPOINTS_PARSE_FAILED,
    AWS_ERROR_SDKUTILS_ENDPOINTS_RESOLVE_INIT_FAILED,
    AWS_ERROR_SDKUTILS_ENDPOINTS_RESOLVE_FAILED,
    AWS_ERROR_SDKUTILS_ENDPOINTS_EMPTY_RULESET,
    AWS_ERROR_SDKUTILS_ENDPOINTS_RULESET_EXHAUSTED,
    AWS_ERROR_SDKUTILS_PARTITIONS_UNSUPPORTED,
    AWS_ERROR_SDKUTILS_PARTITIONS_PARSE_FAILED,
    AWS_ERROR_SDKUTILS_END_RANGE = aws_error_enum_end_range(AWS_C_SDKUTILS_PACKAGE_ID),
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum AwsSdkUtilsLogSubject {
    AWS_LS_SDKUTILS_GENERAL = aws_log_subject_begin_range(AWS_C_SDKUTILS_PACKAGE_ID),
    AWS_LS_SDKUTILS_PROFILE,
    AWS_LS_SDKUTILS_ENDPOINTS_PARSING,
    AWS_LS_SDKUTILS_ENDPOINTS_RESOLVE,
    AWS_LS_SDKUTILS_ENDPOINTS_GENERAL,
    AWS_LS_SDKUTILS_PARTITIONS_PARSING,
    AWS_LS_SDKUTILS_LAST = aws_log_subject_end_range(AWS_C_SDKUTILS_PACKAGE_ID),
}

#[link(name = "aws-c-sdkutils")]
extern "C" {
    pub fn aws_sdkutils_library_init(allocator: *const AwsCAllocator);
    pub fn aws_sdkutils_library_clean_up();
}
