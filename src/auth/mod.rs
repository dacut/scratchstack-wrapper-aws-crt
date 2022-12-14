mod credentials;

use crate::{
    common::{aws_error_enum_begin_range, aws_error_enum_end_range, aws_log_subject_begin_range, aws_log_subject_end_range, AwsCAllocator},
    sdkutils::AwsSdkUtilsErrors,
};

pub use credentials::*;

const AWS_C_AUTH_PACKAGE_ID: isize = 6;

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum AwsAuthErrors {
    AWS_AUTH_PROFILE_PARSE_RECOVERABLE_ERROR = AwsSdkUtilsErrors::AWS_ERROR_SDKUTILS_PARSE_RECOVERABLE as isize,
    AWS_AUTH_PROFILE_PARSE_FATAL_ERROR = AwsSdkUtilsErrors::AWS_ERROR_SDKUTILS_PARSE_FATAL as isize,
    AWS_AUTH_SIGNING_UNSUPPORTED_ALGORITHM = aws_error_enum_begin_range(AWS_C_AUTH_PACKAGE_ID),
    AWS_AUTH_SIGNING_MISMATCHED_CONFIGURATION,
    AWS_AUTH_SIGNING_NO_CREDENTIALS,
    AWS_AUTH_SIGNING_ILLEGAL_REQUEST_QUERY_PARAM,
    AWS_AUTH_SIGNING_ILLEGAL_REQUEST_HEADER,
    AWS_AUTH_SIGNING_INVALID_CONFIGURATION,
    AWS_AUTH_CREDENTIALS_PROVIDER_INVALID_ENVIRONMENT,
    AWS_AUTH_CREDENTIALS_PROVIDER_INVALID_DELEGATE,
    AWS_AUTH_CREDENTIALS_PROVIDER_PROFILE_SOURCE_FAILURE,
    AWS_AUTH_CREDENTIALS_PROVIDER_IMDS_SOURCE_FAILURE,
    AWS_AUTH_CREDENTIALS_PROVIDER_STS_SOURCE_FAILURE,
    AWS_AUTH_CREDENTIALS_PROVIDER_HTTP_STATUS_FAILURE,
    AWS_AUTH_PROVIDER_PARSER_UNEXPECTED_RESPONSE,
    AWS_AUTH_CREDENTIALS_PROVIDER_ECS_SOURCE_FAILURE,
    AWS_AUTH_CREDENTIALS_PROVIDER_X509_SOURCE_FAILURE,
    AWS_AUTH_CREDENTIALS_PROVIDER_PROCESS_SOURCE_FAILURE,
    AWS_AUTH_CREDENTIALS_PROVIDER_STS_WEB_IDENTITY_SOURCE_FAILURE,
    AWS_AUTH_SIGNING_UNSUPPORTED_SIGNATURE_TYPE,
    AWS_AUTH_SIGNING_MISSING_PREVIOUS_SIGNATURE,
    AWS_AUTH_SIGNING_INVALID_CREDENTIALS,
    AWS_AUTH_CANONICAL_REQUEST_MISMATCH,
    AWS_AUTH_SIGV4A_SIGNATURE_VALIDATION_FAILURE,
    AWS_AUTH_CREDENTIALS_PROVIDER_COGNITO_SOURCE_FAILURE,
    AWS_AUTH_ERROR_END_RANGE = aws_error_enum_end_range(AWS_C_AUTH_PACKAGE_ID),
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum aws_auth_log_subject {
    AWS_LS_AUTH_GENERAL = aws_log_subject_begin_range(AWS_C_AUTH_PACKAGE_ID),
    AWS_LS_AUTH_PROFILE,
    AWS_LS_AUTH_CREDENTIALS_PROVIDER,
    AWS_LS_AUTH_SIGNING,
    AWS_LS_IMDS_CLIENT,
    AWS_LS_AUTH_LAST = aws_log_subject_end_range(AWS_C_AUTH_PACKAGE_ID)
}

#[link(name = "aws-c-auth")]
extern "C" {
    pub fn aws_auth_library_init(allocator: *const AwsCAllocator);
    pub fn aws_auth_library_clean_up();
}
