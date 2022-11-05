use {
    crate::{
        common::{AwsCAllocator, AwsCByteBuf, AwsCRefCount},
        io::{AwsCChannel, AwsCChannelHandler, AwsCChannelOnShutdownCompletedFn, AwsCChannelSlot, AwsCEventLoopGroup, AwsCHostResolver, AwsCHostResolutionConfig},
    },
    std::ffi::c_void,
};

pub type AwsCClientBootstrapOnChannelEventFn =
    extern "C" fn(bootstrap: *const AwsCClientBootstrap, error_code: i32, channel: AwsCChannel, user_data: *mut c_void);

pub type AwsCChannelOnProtocolNegotiatedFn = extern "C" fn(
    new_slot: *mut AwsCChannelSlot,
    protocol: *const AwsCByteBuf,
    user_data: *mut c_void,
) -> *mut AwsCChannelHandler;

pub type AwsCClientBootstrapShutdownCompleteFn = extern "C" fn(user_data: *mut c_void);

#[repr(C)]
pub struct AwsCClientBootstrap {
    pub allocator: *const AwsCAllocator,
    pub event_loop_group: *const AwsCEventLoopGroup,
    pub host_resolver: *const AwsCHostResolver,
    pub host_resolver_config: AwsCHostResolutionConfig,
    pub on_protocol_negotiated: *const AwsCChannelOnProtocolNegotiatedFn,
    pub ref_count: AwsCRefCount,
    pub on_shutdown_complete: *const AwsCChannelOnShutdownCompletedFn,
    pub user_data: *mut c_void,
}

#[repr(C)]
pub struct AwsCClientBootstrapOptions {
    pub event_loop_group: *const AwsCEventLoopGroup,
    pub host_resolver: *const AwsCHostResolver,
    pub host_resolution_config: *const AwsCHostResolutionConfig,
    pub on_shutdown_complete: *const AwsCClientBootstrapShutdownCompleteFn,
    pub user_data: *mut c_void,
}

pub type AwsCServerBootstrapOnAcceptChannelSetupFn = extern "C" fn(
    bootstrap: *const AwsCServerBootstrap,
    error_code: i32,
    channel: *mut AwsCChannel,
    user_data: *mut c_void,
);

pub type AwsCServerBootstrapOnAcceptChannelShutdownFn = extern "C" fn(
    bootstrap: *const AwsCServerBootstrap,
    error_code: i32,
    channel: *mut AwsCChannel,
    user_data: *mut c_void,
);

pub type AwsCServerBootstrapOnServerListenerDestroyFn =
    extern "C" fn(bootstrap: *const AwsCServerBootstrap, user_data: *mut c_void);

#[repr(C)]
pub struct AwsCServerBootstrap {
    allocator: *const AwsCAllocator,
    event_loop_group: *const AwsCEventLoopGroup,
    on_protocol_negotiated: *const AwsCChannelOnProtocolNegotiatedFn,
    ref_count: AwsCRefCount,
}
