use {
    crate::{
        common::{AwsCAllocator, AwsCByteBuf, AwsCRefCount, AwsCString},
        io::{AwsCChannelHandler, AwsCChannelSlot},
    },
    std::ffi::c_void,
};

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AwsCTlsVersions {
    AWS_IO_SSLv3,
    AWS_IO_TLSv1,
    AWS_IO_TLSv1_1,
    AWS_IO_TLSv1_2,
    AWS_IO_TLSv1_3,
    AWS_IO_TLS_VER_SYS_DEFAULTS = 128,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AwsCTlsCipherPref {
    AWS_IO_TLS_CIPHER_PREF_SYSTEM_DEFAULT = 0,

    #[deprecated]
    AWS_IO_TLS_CIPHER_PREF_KMS_PQ_TLSv1_0_2019_06 = 1,

    #[deprecated]
    AWS_IO_TLS_CIPHER_PREF_KMS_PQ_SIKE_TLSv1_0_2019_11 = 2,

    #[deprecated]
    AWS_IO_TLS_CIPHER_PREF_KMS_PQ_TLSv1_0_2020_02 = 3,

    #[deprecated]
    AWS_IO_TLS_CIPHER_PREF_KMS_PQ_SIKE_TLSv1_0_2020_02 = 4,

    #[deprecated]
    AWS_IO_TLS_CIPHER_PREF_KMS_PQ_TLSv1_0_2020_07 = 5,

    AWS_IO_TLS_CIPHER_PREF_PQ_TLSv1_0_2021_05 = 6,

    AWS_IO_TLS_CIPHER_PREF_END_RANGE = 0xFFFF,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AwsCTlsHashAlgorithm {
    AWS_TLS_HASH_UNKNOWN,
    AWS_TLS_HASH_SHA1,
    AWS_TLS_HASH_SHA224,
    AWS_TLS_HASH_SHA256,
    AWS_TLS_HASH_SHA384,
    AWS_TLS_HASH_SHA512,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AwsCTlsSignatureAlgorithm {
    AWS_TLS_SIGNATURE_UNKNOWN,
    AWS_TLS_SIGNATURE_RSA,
    AWS_TLS_SIGNATURE_ECDSA,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AwsCTlsKeyOperationType {
    AWS_TLS_KEY_OPERATION_UNKNOWN,
    AWS_TLS_KEY_OPERATION_SIGN,
    AWS_TLS_KEY_OPERATION_DECRYPT,
}

#[repr(C)]
pub struct AwsCTlsCtx {
    alloc: *const AwsCAllocator,
    r#impl: *mut c_void,
    ref_count: AwsCRefCount,
}

pub type AwsCTlsOnNegotiationResultFn = extern "C" fn(
    handler: *mut AwsCChannelHandler,
    slot: *mut AwsCChannelSlot,
    error_code: i32,
    user_data: *mut c_void,
);

pub type AwsCTlsOnDataReadFn = extern "C" fn(
    handler: *mut AwsCChannelHandler,
    slot: *mut AwsCChannelSlot,
    buffer: *mut AwsCByteBuf,
    user_data: *mut c_void,
);

pub type AwsCTlsOnErrorFn = extern "C" fn(
    handler: *mut AwsCChannelHandler,
    slot: *mut AwsCChannelSlot,
    err: i32,
    message: *const u8,
    user_data: *mut c_void,
);

#[repr(C)]
pub struct AwsCTlsConnectionOptions {
    pub alpn_list: *mut AwsCString,
    pub server_name: *mut AwsCString,
    pub on_negotiation_result: *const AwsCTlsOnNegotiationResultFn,
    pub on_data_read: *const AwsCTlsOnDataReadFn,
    pub on_error: *const AwsCTlsOnErrorFn,
    pub user_data: *mut c_void,
    pub ctx: *mut AwsCTlsCtx,
    pub advertise_alpn_message: bool,
    pub timeout_ms: u32,
}

#[repr(C)]
pub struct AwsCTlsKeyOperation;

#[repr(C)]
pub struct AwsCTlsCtxOptions {
    pub allocator: *const AwsCAllocator,
    pub minimum_tls_version: AwsCTlsVersions,
    pub cipher_pref: AwsCTlsCipherPref,
    pub ca_file: AwsCByteBuf,
    pub ca_path: *mut AwsCString,
    pub alpn_list: *mut AwsCString,
    pub certificate: AwsCByteBuf,
    #[cfg(windows)]
    pub system_certificate_path: *const u8,
    pub private_key: AwsCByteBuf,

    #[cfg(any(target_os = "ios", target_os = "macos"))]
    pub pkcs12: AwsCByteBuf,

    #[cfg(any(target_os = "ios", target_os = "macos"))]
    pub pkcs12_password: AwsCByteBuf,

    #[cfg(target_os = "macos")]
    pub keychain_path: *mut AwsCString,

    pub max_fragment_size: usize,

    pub verify_peer: bool,

    pub ctx_options_extension: *mut c_void,

    pub custom_key_op_handler: *mut AwsCCustomKeyOpHandler,
}

#[repr(C)]
pub struct AwsCTlsNegotiatedProtocolMessage {
    pub protocol: AwsCByteBuf,
}

#[allow(dead_code)]
const AWS_TLS_NEGOTIATED_PROTOCOL_MESSAGE: i32 = 0x01;

pub type AwsCTlsOnProtocolNegotiated = extern "C" fn(
    new_slot: *mut AwsCChannelSlot,
    protocol: *mut AwsCByteBuf,
    user_data: *mut c_void,
) -> *mut AwsCChannelHandler;

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AwsCTlsNegotiationStatus {
    AWS_TLS_NEGOTIATION_STATUS_NONE,
    AWS_TLS_NEGOTIATION_STATUS_ONGOING,
    AWS_TLS_NEGOTIATION_STATUS_SUCCESS,
    AWS_TLS_NEGOTIATION_STATUS_FAILURE
}

#[repr(C)]
pub struct AwsCCustomKeyOpHandlerVtable {
    pub on_key_operation: *const extern "C" fn(key_op_handler: *mut AwsCCustomKeyOpHandler, operation: *mut AwsCTlsKeyOperation),
}

#[repr(C)]
pub struct AwsCCustomKeyOpHandler {
    pub r#impl: *mut c_void,
    pub vtable: *const AwsCCustomKeyOpHandlerVtable,
    pub ref_count: AwsCRefCount,
}

// TODO: Add functions