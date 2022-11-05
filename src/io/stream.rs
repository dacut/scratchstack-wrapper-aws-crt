use {
    crate::common::{AwsCAllocator, AwsCByteBuf, AwsCByteCursor, AwsCRefCount},
    libc::FILE,
    std::ffi::c_void,
};

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AwsCStreamSeekBasis {
    AWS_SSB_BEGIN = 0,
    AWS_SSB_END = 2,
}

#[repr(C)]
pub struct AwsCStreamStatus {
    pub is_end_of_stream: bool,
    pub is_valid: bool,
}

#[repr(C)]
pub struct AwsCInputStreamVtable {
    pub seek: *const extern "C" fn(stream: *mut AwsCInputStream, offset: i64, basis: AwsCStreamSeekBasis) -> i32,
    pub read: *const extern "C" fn(stream: *mut AwsCInputStream, dest: *mut AwsCByteBuf) -> i32,
    pub get_status: *const extern "C" fn(stream: *mut AwsCInputStream, status: *mut AwsCStreamStatus) -> i32,
    pub get_length: *const extern "C" fn(stream: *mut AwsCInputStream, out_length: *mut i64) -> i32,
    pub acquire: *const extern "C" fn(stream: *mut AwsCInputStream) -> i32,
    pub release: *const extern "C" fn(stream: *mut AwsCInputStream) -> i32,
}

#[repr(C)]
pub struct AwsCInputStream {
    pub r#impl: *mut c_void,
    pub aws_input_stream_vtable: *mut AwsCInputStreamVtable,
    pub ref_count: AwsCRefCount,
}

#[link(name = "aws-c-io")]
extern "C" {
    pub fn aws_input_stream_acquire(stream: *mut AwsCInputStream) -> *mut AwsCInputStream;
    pub fn aws_input_stream_release(stream: *mut AwsCInputStream) -> *mut AwsCInputStream;
    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_input_stream_seek(stream: *mut AwsCInputStream, offset: i64, basis: AwsCStreamSeekBasis) -> i32;
    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_input_stream_read(stream: *mut AwsCInputStream, dest: *mut AwsCByteBuf) -> i32;
    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_input_stream_get_status(stream: *mut AwsCInputStream, status: *mut AwsCStreamStatus) -> i32;
    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_input_stream_get_length(stream: *mut AwsCInputStream, out_length: *mut i64) -> i32;
    pub fn aws_input_stream_destroy(stream: *mut AwsCInputStream);
    pub fn aws_input_stream_new_from_cursor(
        allocator: *const AwsCAllocator,
        cursor: *const AwsCByteCursor,
    ) -> *mut AwsCInputStream;
    pub fn aws_input_stream_new_from_file(
        allocator: *const AwsCAllocator,
        file_name: *const u8,
    ) -> *mut AwsCInputStream;
    pub fn aws_input_stream_new_from_open_file(
        allocator: *const AwsCAllocator,
        file: *mut FILE,
    ) -> *mut AwsCInputStream;
}
