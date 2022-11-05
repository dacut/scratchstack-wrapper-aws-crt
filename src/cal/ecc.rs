use {
    crate::common::{AwsCAllocator, AwsCByteBuf, AwsCByteCursor, AwsCRefCount},
    std::ffi::c_void,
};

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AwsCEccCurveName {
    AWS_CAL_ECDSA_P256,
    AWS_CAL_ECDSA_P384,
}

pub type AwsCEccKeyPairDestroyFn = extern "C" fn(key_pair: *mut AwsCEccKeyPair);
pub type AwsCEccKeyPairSignMessageFn = extern "C" fn(key_pair: *const AwsCEccKeyPair, message: *const AwsCByteCursor, signature_output: *mut AwsCByteBuf) -> i32;
pub type AwsCEccKeyPairDerivePublicKeyFn = extern "C" fn(key_pair: *mut AwsCEccKeyPair) -> i32;
pub type AwsCEccKeyPairVerifySignatureFn = extern "C" fn(key_pair: *const AwsCEccKeyPair, message: *const AwsCByteCursor, signature: *const AwsCByteCursor) -> i32;
pub type AwsCEccKeyPairSignatureLengthFn = extern "C" fn(key_pair: *const AwsCEccKeyPair) -> usize;

#[repr(C)]
pub struct AwsCEccKeyPairVtable {
    pub destroy: *const AwsCEccKeyPairDestroyFn,
    pub derive_pub_key: *const AwsCEccKeyPairDerivePublicKeyFn,
    pub sign_message: *const AwsCEccKeyPairSignMessageFn,
    pub verify_signature: *const AwsCEccKeyPairVerifySignatureFn,
    pub signature_length: *const AwsCEccKeyPairSignatureLengthFn,
}

#[repr(C)]
pub struct AwsCEccKeyPair {
    pub allocator: *const AwsCAllocator,
    pub ref_count: AwsCRefCount,
    pub curve_name: AwsCEccCurveName,
    pub key_buf: AwsCByteBuf,
    pub pub_x: AwsCByteBuf,
    pub pub_y: AwsCByteBuf,
    pub priv_d: AwsCByteBuf,
    pub vtable: *mut AwsCEccKeyPairVtable,
    pub r#impl: *mut c_void,
}

// TODO: Add functions