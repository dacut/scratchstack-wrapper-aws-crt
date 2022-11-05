use {
    crate::{
        common::{AwsCAllocator, AwsCByteBuf, AwsCByteCursor, AwsCRefCount, AwsCString},
        http::{AwsCHttpHeader, AwsCHttpHeaderBlock, AwsCHttpMessage, AwsCHttpStatusCode},
        io::AwsCTlsConnectionOptions,
    },
    std::ffi::c_void,
};

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[deprecated(since = "0.1.0", note = "Use proxy strategy instead")]
pub enum AwsCHttpProxyAuthenticationType {
    AWS_HPAT_NONE = 0,
    AWS_HPAT_BASIC,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AwsCHttpProxyEnvVarType {
    AWS_HPEV_DISABLE = 0,
    AWS_HPEV_ENABLE,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AwsCHttpProxyConnectionType {
    AWS_HPCT_HTTP_LEGACY = 0,
    AWS_HPCT_HTTP_FORWARD,
    AWS_HPCT_HTTP_TUNNEL,
}

#[repr(C)]
pub struct CProxyEnvVarSettings {
    pub env_var_type: AwsCHttpProxyEnvVarType,
    pub connection_type: AwsCHttpProxyConnectionType,
    pub tls_options: *const AwsCTlsConnectionOptions,
}

#[repr(C)]
pub struct AwsCHttpProxyOptions {
    pub connection_type: AwsCHttpProxyConnectionType,
    pub host: AwsCByteCursor,
    pub port: u16,
    pub tls_options: *const AwsCTlsConnectionOptions,
    pub proxy_strategy: *const AwsCHttpProxyStrategy,

    #[deprecated(since = "0.1.0", note = "Use proxy_strategy instead")]
    #[allow(deprecated)]
    pub auth_type: AwsCHttpProxyAuthenticationType,

    #[deprecated(since = "0.1.0", note = "Use proxy_strategy instead")]
    pub auth_username: AwsCByteCursor,

    #[deprecated(since = "0.1.0", note = "Use proxy_strategy instead")]
    pub auth_password: AwsCByteCursor,
}

pub type AwsCHttpProxyNegotiationGetTokenSyncFn =
    extern "C" fn(user_data: *mut c_void, out_error_code: *mut i32) -> *mut AwsCString;

pub type AwsCHttpProxyNegotiationGetChallengeTokenSyncFn = extern "C" fn(
    user_data: *mut c_void,
    challenge_context: *const AwsCByteCursor,
    out_error_code: *mut i32,
) -> *mut AwsCString;

pub type AwsCHttpProxyNegotiationTerminateFn =
    extern "C" fn(message: *mut AwsCHttpMessage, error_code: i32, internal_proxy_user_data: *mut c_void);

pub type AwsCHttpProxyNegotiationHttpRequestForwardFn =
    extern "C" fn(message: *mut AwsCHttpMessage, internal_proxy_user_data: *mut c_void);

pub type AwsCHttpProxyNegotiationHttpRequestTransformAsyncFn = extern "C" fn(
    proxy_negotiator: *mut AwsCHttpProxyNegotiator,
    message: *mut AwsCHttpMessage,
    negotiation_termination_callback: *const AwsCHttpProxyNegotiationTerminateFn,
    negotiation_http_request_forward_callback: *const AwsCHttpProxyNegotiationHttpRequestForwardFn,
    internal_proxy_user_data: *mut c_void,
) -> i32;

pub type AwsCHttpProxyNegotiationHttpRequestTransformFn =
    extern "C" fn(proxy_negotiator: *mut AwsCHttpProxyNegotiator, message: *mut AwsCHttpMessage) -> i32;

pub type AwsCHttpProxyNegotiationConnectOnIncomingHeadersFn = extern "C" fn(
    proxy_negotiator: *mut AwsCHttpProxyNegotiator,
    header_block: AwsCHttpHeaderBlock,
    header_array: *const AwsCHttpHeader,
    num_headers: usize,
) -> i32;

pub type AwsCHttpProxyNegotiatorConnectStatusFn =
    extern "C" fn(proxy_negotiator: *mut AwsCHttpProxyNegotiator, status_code: AwsCHttpStatusCode) -> i32;

pub type AwsCHttpProxyNegotiatorConnectOnIncomingBodyFn =
    extern "C" fn(proxy_negotiator: *mut AwsCHttpProxyNegotiator, data: *const AwsCByteCursor) -> i32;

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AwsCHttpProxyNegotiationRetryDirective {
    AWS_HPNRD_STOP,
    AWS_HPNRD_NEW_CONNECTION,
    AWS_HPNRD_CURRENT_CONNECTION,
}

pub type AwsCHttpProxyNegotiatorGetRetryDirectiveFn =
    extern "C" fn(proxy_negotiator: *mut AwsCHttpProxyNegotiator) -> AwsCHttpProxyNegotiationRetryDirective;

#[repr(C)]
pub struct AwsCHttpProxyNegotiatorForwardingVtable {
    pub forward_request_transform: *const AwsCHttpProxyNegotiationHttpRequestTransformFn,
}

#[repr(C)]
pub struct AwsCHttpProxyNegotiatorTunnellingVtable {
    pub connect_request_transform: *const AwsCHttpProxyNegotiationHttpRequestTransformAsyncFn,
    pub on_incoming_headers_callback: *const AwsCHttpProxyNegotiationConnectOnIncomingHeadersFn,
    pub on_status_callback: *const AwsCHttpProxyNegotiatorConnectStatusFn,
    pub on_incoming_body_callback: *const AwsCHttpProxyNegotiatorConnectOnIncomingBodyFn,
    pub get_retry_directive_callback: *const AwsCHttpProxyNegotiatorGetRetryDirectiveFn,
}

#[repr(C)]
pub union AwsHttpProxyNegotiatorVtable {
    pub forwarding_vtable: *mut AwsCHttpProxyNegotiatorForwardingVtable,
    pub tunnelling_vtable: *mut AwsCHttpProxyNegotiatorTunnellingVtable,
}

#[repr(C)]
pub struct AwsCHttpProxyNegotiator {
    pub ref_count: AwsCRefCount,
    pub r#impl: *mut c_void,
    pub strategy_vtable: AwsHttpProxyNegotiatorVtable,
}

pub type AwsCHttpProxyStrategyCreateNegotiatorFn = extern "C" fn(
    proxy_strategy: *mut AwsCHttpProxyStrategy,
    allocator: *const AwsCAllocator,
) -> *mut AwsCHttpProxyNegotiator;

#[repr(C)]
pub struct AwsCHttpProxyStrategyVtable {
    pub create_negotiator: *const AwsCHttpProxyStrategyCreateNegotiatorFn,
}

#[repr(C)]
pub struct AwsCHttpProxyStrategy {
    pub ref_count: AwsCRefCount,
    pub vtable: *mut AwsCHttpProxyStrategyVtable,
    pub r#impl: *mut c_void,
    pub proxy_connection_type: AwsCHttpProxyConnectionType,
}

#[repr(C)]
pub struct AwsCHttpProxyStrategyBasicAuthOptions {
    pub proxy_connection_type: AwsCHttpProxyConnectionType,
    pub user_name: AwsCByteCursor,
    pub password: AwsCByteCursor,
}

#[repr(C)]
pub struct AwsCHttpProxyStrategyTunnelingKerberosOptions {
    pub get_token: *const AwsCHttpProxyNegotiationGetTokenSyncFn,
    pub get_token_user_data: *mut c_void,
}

#[repr(C)]
pub struct AwsCHttpProxyStrategyTunnelingNtlmOptions {
    pub get_token: *const AwsCHttpProxyNegotiationGetTokenSyncFn,
    pub get_challenge_token: *const AwsCHttpProxyNegotiationGetChallengeTokenSyncFn,
    pub get_challenge_token_user_data: *mut c_void,
}

#[repr(C)]
pub struct AwsCHttpProxyStrategyTunnelingAdaptiveOptions {
    pub kerberos_options: *mut AwsCHttpProxyStrategyTunnelingKerberosOptions,
    pub ntlm_options: *mut AwsCHttpProxyStrategyTunnelingNtlmOptions,
}

#[repr(C)]
pub struct AwsCHttpProxyStrategyTunnelingSequenceOptions {
    pub strategies: *mut *mut AwsCHttpProxyStrategy,
    pub strategy_count: usize,
}

#[repr(C)]
pub struct AwsCHttpProxyConfig {
    // Private implementation
    allocator: *const AwsCAllocator,
    connection_type: AwsCHttpProxyConnectionType,
    host: AwsCByteBuf,
    port: u16,
    tls_options: AwsCTlsConnectionOptions,
    proxy_strategy: *mut AwsCHttpProxyStrategy,
}
