mod ecc;

pub use ecc::*;

use crate::common::{
    aws_error_enum_begin_range, aws_error_enum_end_range, aws_log_subject_begin_range, aws_log_subject_end_range,
    AwsCAllocator,
};

pub const AWS_C_CAL_PACKAGE_ID: isize = 7;

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AwsCCalErrors {
    AWS_ERROR_CAL_SIGNATURE_VALIDATION_FAILED = aws_error_enum_begin_range(AWS_C_CAL_PACKAGE_ID),
    AWS_ERROR_CAL_MISSING_REQUIRED_KEY_COMPONENT,
    AWS_ERROR_CAL_INVALID_KEY_LENGTH_FOR_ALGORITHM,
    AWS_ERROR_CAL_UNKNOWN_OBJECT_IDENTIFIER,
    AWS_ERROR_CAL_MALFORMED_ASN1_ENCOUNTERED,
    AWS_ERROR_CAL_MISMATCHED_DER_TYPE,
    AWS_ERROR_CAL_UNSUPPORTED_ALGORITHM,
    AWS_ERROR_CAL_END_RANGE = aws_error_enum_end_range(AWS_C_CAL_PACKAGE_ID),
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AwsCCalLogSubject {
    AWS_LS_CAL_GENERAL = aws_log_subject_begin_range(AWS_C_CAL_PACKAGE_ID),
    AWS_LS_CAL_ECC,
    AWS_LS_CAL_HASH,
    AWS_LS_CAL_HMAC,
    AWS_LS_CAL_DER,
    AWS_LS_CAL_LIBCRYPTO_RESOLVE,
    AWS_LS_CAL_LAST = aws_log_subject_end_range(AWS_C_CAL_PACKAGE_ID),
}

#[link(name = "aws-c-cal")]
extern "C" {
    pub fn aws_cal_library_init(allocator: *const AwsCAllocator);
    pub fn aws_cal_library_clean_up();
}
