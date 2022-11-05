use {
    crate::{
        cal::AwsCEccKeyPair,
        common::{AwsCAllocator, AwsCAtomicVar, AwsCByteCursor, AwsCString},
        http::{AwsCHttpProxyOptions, AwsCHttpConnectionManagerOptions, AwsCHttpConnectionManager, AwsCHttpConnectionManagerOnConnectionSetupFn, AwsCHttpConnection, AwsCHttpMakeRequestOptions, AwsCHttpStream},
        io::{AwsCClientBootstrap, AwsCIoClockFn, AwsCTlsConnectionOptions, AwsCTlsCtx},
    },
    std::ffi::c_void,
};

type AwsCHttpConnectionManagerNewFn = extern "C" fn(allocator: *const AwsCAllocator, options: *mut AwsCHttpConnectionManagerOptions) -> *mut AwsCHttpConnectionManager;
type AwsCHttpConnectionManagerReleaseFn = extern "C" fn(manager: *mut AwsCHttpConnectionManager);
type AwsCHttpConnectionManagerAcquireConnectionFn = extern "C" fn(manager: *mut AwsCHttpConnectionManager, on_connection_setup: *const AwsCHttpConnectionManagerOnConnectionSetupFn, user_data: *mut c_void) -> i32;
type AwsCHttpConnectionManagerReleaseConnectionFn = extern "C" fn(manager: *mut AwsCHttpConnectionManager, connection: *mut AwsCHttpConnection) -> i32;
type AwsCHttpConnectionMakeRequestFn = extern "C" fn(client_connection: *mut AwsCHttpConnection, options: *const AwsCHttpMakeRequestOptions) -> *mut AwsCHttpStream;
type AwsCHttpStreamActivateFn = extern "C" fn(stream: *mut AwsCHttpStream) -> i32;
type AwsCHttpStreamGetConnectionFn = extern "C" fn(stream: *const AwsCHttpStream) -> *mut AwsCHttpConnection;
type AwsCHttpStreamGetIncomingResponseStatusFn = extern "C" fn(stream: *const AwsCHttpStream, out_status: *mut i32) -> i32;
type AwsCHttpStreamReleaseFn = extern "C" fn(stream: *mut AwsCHttpStream);
type AwsCHttpConnectionCloseFn = extern "C" fn(connection: *mut AwsCHttpConnection);

#[repr(C)]
pub struct AwsCAuthHttpSystemVtable {
    // Private implementation
    aws_http_connection_manager_new: *const AwsCHttpConnectionManagerNewFn,
    aws_http_connection_manager_release: *const AwsCHttpConnectionManagerReleaseFn,
    aws_http_connection_manager_acquire_connection: *const AwsCHttpConnectionManagerAcquireConnectionFn,
    aws_http_connection_manager_release_connection: *const AwsCHttpConnectionManagerReleaseConnectionFn,
    aws_http_connection_make_request: *const AwsCHttpConnectionMakeRequestFn,
    aws_http_stream_activate: *const AwsCHttpStreamActivateFn,
    aws_http_stream_get_connection: *const AwsCHttpStreamGetConnectionFn,
    aws_http_stream_get_incoming_response_status: *const AwsCHttpStreamGetIncomingResponseStatusFn,
    aws_http_stream_release: *const AwsCHttpStreamReleaseFn,
    aws_http_connection_close: *const AwsCHttpConnectionCloseFn,
}

#[repr(C)]
pub struct AwsCCredentials {
    // Private implementation
    allocator: *const AwsCAllocator,
    ref_count: AwsCAtomicVar,
    access_key_id: *mut AwsCString,
    secret_access_key: *mut AwsCString,
    session_token: *mut AwsCString,
    expiration_timepoint_seconds: u64,
    ecc_key: *mut AwsCEccKeyPair,
}

pub type AwsCOnGetCredentialsCallbackFn =
    extern "C" fn(credentials: *mut AwsCCredentials, error_code: i32, user_data: *mut c_void);

pub type AwsCCredentialsProviderGetCredentialsFn = extern "C" fn(
    provider: *const AwsCCredentialsProvider,
    callback: AwsCOnGetCredentialsCallbackFn,
    user_data: *mut c_void,
) -> i32;

pub type AwsCCredentialsProviderDestroyFn = extern "C" fn(provider: *mut AwsCCredentialsProvider);

#[repr(C)]
pub struct AwsCCredentialsProviderVtable {
    pub get_credentials: *const AwsCCredentialsProviderGetCredentialsFn,
    pub destroy: *const AwsCCredentialsProviderDestroyFn,
}

type AwsCCredentialsProviderShutdownCompletedFn = extern "C" fn(user_data: *mut c_void);

#[repr(C)]
pub struct AwsCCredentialsProviderShutdownOptions {
    pub shutdown_callback: *const AwsCCredentialsProviderShutdownCompletedFn,
    pub shutdown_user_data: *mut c_void,
}

#[repr(C)]
pub struct AwsCCredentialsProvider {
    pub vtable: *const AwsCCredentialsProviderVtable,
    pub allocator: *const AwsCAllocator,
    pub shutdown_options: *const AwsCCredentialsProviderShutdownOptions,
    pub r#impl: *mut c_void,
    pub ref_count: AwsCAtomicVar,
}

#[repr(C)]
pub struct AwsCCredentialsProviderStaticOptions {
    pub shutdown_options: AwsCCredentialsProviderShutdownOptions,
    pub access_key_id: AwsCByteCursor,
    pub secret_access_key: AwsCByteCursor,
    pub session_token: AwsCByteCursor,
}

#[repr(C)]
pub struct AwsCCredentialsProviderEnvironmentOptions {
    pub shutdown_options: AwsCCredentialsProviderShutdownOptions,
}

#[repr(C)]
pub struct AwsCCredentialsProviderProfileOptions {
    pub shutdown_options: AwsCCredentialsProviderShutdownOptions,
    pub profile_name_override: AwsCByteCursor,
    pub config_file_name_override: AwsCByteCursor,
    pub credentials_file_name_override: AwsCByteCursor,
    pub boostrap: *const AwsCClientBootstrap,
    pub tls_ctx: *const AwsCTlsCtx,
    pub function_table: *const AwsCAuthHttpSystemVtable,
}

#[repr(C)]
pub struct AwsCCredentialsProviderCachedOptions {
    pub shutdown_options: AwsCCredentialsProviderShutdownOptions,
    pub source: *const *const AwsCCredentialsProvider,
    pub refresh_time_in_milliseconds: u64,
    high_res_clock_fn: AwsCIoClockFn,
    system_clock_fn: AwsCIoClockFn,
}

#[repr(C)]
pub struct AwsCCredentialsProviderChainOptions {
    pub shutdown_options: AwsCCredentialsProviderShutdownOptions,
    pub providers: *const *const AwsCCredentialsProvider,
    pub provider_count: usize,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
pub enum AwsImdsProtocolVersion {
    IMDS_PROTOCOL_V2,
    IMDS_PROTOCOL_V1,
}

#[repr(C)]
pub struct AwsCCredentialsProviderImdsOptions {
    pub shutdown_options: AwsCCredentialsProviderShutdownOptions,
    pub bootstrap: *const AwsCClientBootstrap,
    pub imds_version: AwsImdsProtocolVersion,
    pub function_table: *const AwsCAuthHttpSystemVtable,
}

#[repr(C)]
pub struct AwsCCredentialsProviderEcsOptions {
    pub shutdown_options: AwsCCredentialsProviderShutdownOptions,
    pub bootstrap: *const AwsCClientBootstrap,
    pub host: AwsCByteCursor,
    pub path_and_query: AwsCByteCursor,
    pub auth_token: AwsCByteCursor,
    pub tls_ctx: *const AwsCTlsCtx,
    pub function_table: *const AwsCAuthHttpSystemVtable,
    pub port: u16,
}

#[repr(C)]
pub struct AwsCCredentialsProviderX509Options {
    pub shutdown_options: AwsCCredentialsProviderShutdownOptions,
    pub bootstrap: *const AwsCClientBootstrap,
    pub tls_connection_options: *const AwsCTlsConnectionOptions,
    pub thing_name: AwsCByteCursor,
    pub role_alias: AwsCByteCursor,
    pub endpoint: AwsCByteCursor,
    pub proxy_options: *const AwsCHttpProxyOptions,
    pub function_table: *const AwsCAuthHttpSystemVtable,
}

#[repr(C)]
pub struct AwsCCredentialsProviderStsWebIdentityOptions {
    pub shutdown_options: AwsCCredentialsProviderShutdownOptions,
    pub bootstrap: *const AwsCClientBootstrap,
    pub tls_ctx: *const AwsCTlsCtx,
    pub function_table: *const AwsCAuthHttpSystemVtable,
}

#[repr(C)]
pub struct AwsCCredentialsProviderStsOptions {
    pub bootstrap: *const AwsCClientBootstrap,
    pub tls_ctx: *const AwsCTlsCtx,
    pub creds_provider: *const AwsCCredentialsProvider,
    pub role_arn: AwsCByteCursor,
    pub session_name: AwsCByteCursor,
    pub duration_seconds: u16,
    pub http_proxy_options: *const AwsCHttpProxyOptions,
    pub shutdown_options: AwsCCredentialsProviderShutdownOptions,
    pub function_table: *const AwsCAuthHttpSystemVtable,
    pub system_clock_fn: AwsCIoClockFn,
}

#[repr(C)]
pub struct AwsCCredentialsProviderProcessOptions {
    pub shutdown_options: AwsCCredentialsProviderShutdownOptions,
    pub profile_to_use: AwsCByteCursor,
}

#[repr(C)]
pub struct AwsCCredentialsProviderChainDefaultOptions {
    pub shutdown_options: AwsCCredentialsProviderShutdownOptions,
    pub bootstrap: *const AwsCClientBootstrap,
    pub tls_ctx: *const AwsCTlsCtx,
}

pub type AwsCCredentialsProviderDeleteGetCredentialsFn = extern "C" fn(
    delegate_user_data: *mut c_void,
    callback: AwsCOnGetCredentialsCallbackFn,
    callback_user_data: *mut c_void,
) -> i32;

#[repr(C)]
pub struct AwsCCredentialsProviderDelegateOptions {
    pub shutdown_options: AwsCCredentialsProviderShutdownOptions,
    pub delegate_get_credentials_fn: *const AwsCCredentialsProviderDeleteGetCredentialsFn,
    pub delegate_user_data: *mut c_void,
}

#[repr(C)]
pub struct AwsCCognitoIdentityProviderTokenPair {
    pub identity_provider_name: AwsCByteCursor,
    pub identity_provider_token: AwsCByteCursor,
}

#[repr(C)]
pub struct AwsCCredentialsProviderCognitoOptions {
    pub shutdown_options: AwsCCredentialsProviderShutdownOptions,
    pub endpoint: AwsCByteCursor,
    pub identity: AwsCByteCursor,
    pub logins: *const AwsCCognitoIdentityProviderTokenPair,
    pub login_count: usize,
    pub custom_role_arn: *const AwsCByteCursor,
    pub bootstrap: *const AwsCClientBootstrap,
    pub tls_ctx: *const AwsCTlsCtx,
    pub http_proxy_options: *const AwsCHttpProxyOptions,
    pub function_table: *const AwsCAuthHttpSystemVtable,
}

#[link(name = "aws-c-auth")]
extern "C" {
    pub fn aws_credentials_new(
        allocator: *const AwsCAllocator,
        access_key_id_cursor: AwsCByteCursor,
        secret_access_key_cusor: AwsCByteCursor,
        session_token_cursor: AwsCByteCursor,
        expiration_timepoint_seconds: u64,
    ) -> *mut AwsCCredentials;
    pub fn aws_credentials_new_anonymous(allocator: *const AwsCAllocator) -> *mut AwsCCredentials;
    pub fn aws_credentials_new_from_string(
        allocator: *const AwsCAllocator,
        access_key_id: *const AwsCString,
        secret_access_key: *const AwsCString,
        session_token: *const AwsCString,
        expiration_timepoint_seconds: u64,
    ) -> *mut AwsCCredentials;
    pub fn aws_credentials_new_ecc(
        allocator: *const AwsCAllocator,
        access_key_id: AwsCByteCursor,
        ecc_key: *const AwsCEccKeyPair,
        session_token: AwsCByteCursor,
        expiration_timepoint_seconds: u64,
    ) -> *mut AwsCCredentials;
    pub fn aws_credentials_new_ecc_from_aws_credentials(
        allocator: *const AwsCAllocator,
        credentials: *const AwsCCredentials,
    ) -> *mut AwsCCredentials;
    pub fn aws_credentials_acquire(credentials: *const AwsCCredentials);
    pub fn aws_credentials_release(credentials: *const AwsCCredentials);
    pub fn aws_credentials_get_access_key_id(credentials: *const AwsCCredentials) -> AwsCByteCursor;
    pub fn aws_credentials_get_secret_access_key(credentials: *const AwsCCredentials) -> AwsCByteCursor;
    pub fn aws_credentials_get_session_token(credentials: *const AwsCCredentials) -> AwsCByteCursor;
    pub fn aws_credentials_get_expiration_timepoint_seconds(credentials: *const AwsCCredentials) -> u64;
    pub fn aws_credentials_get_ecc_key_pair(credentials: *const AwsCCredentials) -> *const AwsCEccKeyPair;
    pub fn aws_credentials_is_anonymous(credentials: *const AwsCCredentials) -> bool;
    pub fn aws_ecc_key_pair_new_ecdsa_p256_key_from_aws_credentials(
        allocator: *const AwsCAllocator,
        credentials: *const AwsCCredentials,
    ) -> *mut AwsCEccKeyPair;
    pub fn aws_credentials_provider_release(provider: *mut AwsCCredentialsProvider) -> *mut AwsCCredentialsProvider;
    pub fn aws_credentials_provider_acquire(provider: *mut AwsCCredentialsProvider) -> *mut AwsCCredentialsProvider;

    #[must_use = "returns an i32 that contains a result code (AWS_OP_SUCCESS or AWS_OP_ERR)"]
    pub fn aws_credentials_provider_get_credentials(
        provider: *const AwsCCredentialsProvider,
        callback: AwsCOnGetCredentialsCallbackFn,
        user_data: *mut c_void,
    ) -> i32;

    pub fn aws_credentials_provider_new_static(
        allocator: *const AwsCAllocator,
        options: *const AwsCCredentialsProviderStaticOptions,
    ) -> *mut AwsCCredentialsProvider;

    pub fn aws_credentials_provider_new_anonymous(
        allocator: *const AwsCAllocator,
        shutdown_options: *const AwsCCredentialsProviderShutdownOptions,
    ) -> *mut AwsCCredentialsProvider;

    pub fn aws_credentials_provider_new_environment(
        allocator: *const AwsCAllocator,
        options: *const AwsCCredentialsProviderEnvironmentOptions,
    ) -> *mut AwsCCredentialsProvider;

    pub fn aws_credentials_provider_new_cached(
        allocator: *const AwsCAllocator,
        options: *const AwsCCredentialsProviderCachedOptions,
    ) -> *mut AwsCCredentialsProvider;

    pub fn aws_credentials_provider_new_profile(
        allocator: *const AwsCAllocator,
        options: *const AwsCCredentialsProviderProfileOptions,
    ) -> *mut AwsCCredentialsProvider;

    pub fn aws_credentials_provider_new_sts(
        allocator: *const AwsCAllocator,
        options: *const AwsCCredentialsProviderStsOptions,
    ) -> *mut AwsCCredentialsProvider;

    pub fn aws_credentials_provider_new_chain(
        allocator: *const AwsCAllocator,
        options: *const AwsCCredentialsProviderChainOptions,
    ) -> *mut AwsCCredentialsProvider;

    pub fn aws_credentials_provider_new_imds(
        allocator: *const AwsCAllocator,
        options: *const AwsCCredentialsProviderImdsOptions,
    ) -> *mut AwsCCredentialsProvider;

    pub fn aws_credentials_provider_new_ecs(
        allocator: *const AwsCAllocator,
        options: *const AwsCCredentialsProviderEcsOptions,
    ) -> *mut AwsCCredentialsProvider;

    pub fn aws_credentials_provider_new_x509(
        allocator: *const AwsCAllocator,
        options: *const AwsCCredentialsProviderX509Options,
    ) -> *mut AwsCCredentialsProvider;

    pub fn aws_credentials_provider_new_sts_web_identity(
        allocator: *const AwsCAllocator,
        options: *const AwsCCredentialsProviderStsWebIdentityOptions,
    ) -> *mut AwsCCredentialsProvider;

    pub fn aws_credentials_provider_new_process(
        allocator: *const AwsCAllocator,
        options: *const AwsCCredentialsProviderProcessOptions,
    ) -> *mut AwsCCredentialsProvider;

    pub fn aws_credentials_provider_new_delegate(
        allocator: *const AwsCAllocator,
        options: *const AwsCCredentialsProviderDelegateOptions,
    ) -> *mut AwsCCredentialsProvider;

    pub fn aws_credentials_provider_new_cognito(
        allocator: *const AwsCAllocator,
        options: *const AwsCCredentialsProviderCognitoOptions,
    ) -> *mut AwsCCredentialsProvider;

    pub fn aws_credentials_provider_new_cognito_caching(
        allocator: *const AwsCAllocator,
        options: *const AwsCCredentialsProviderCognitoOptions,
    ) -> *mut AwsCCredentialsProvider;

    pub fn aws_credentials_provider_new_chain_default(
        allocator: *const AwsCAllocator,
        options: *const AwsCCredentialsProviderChainDefaultOptions,
    ) -> *mut AwsCCredentialsProvider;
}
