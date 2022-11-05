use crate::common::{AwsCAllocator, AwsCByteBuf, AwsCByteCursor};

#[repr(C)]
pub struct AwsCString {
    pub allocator: *const AwsCAllocator,
    pub len: usize,
    pub bytes: [u8; 1],
}

#[link(name = "aws-c-common")]
extern "C" {
    pub fn aws_string_eq(a: *const AwsCString, b: *const AwsCString) -> bool;
    pub fn aws_string_eq_ignore_case(a: *const AwsCString, b: *const AwsCString) -> bool;
    pub fn aws_string_eq_byte_cursor(str: *const AwsCString, cursor: *const AwsCByteCursor) -> bool;
    pub fn aws_string_eq_byte_cursor_ignore_case(str: *const AwsCString, cursor: *const AwsCByteCursor) -> bool;
    pub fn aws_string_eq_byte_buf(str: *const AwsCString, buf: *const AwsCByteBuf) -> bool;
    pub fn aws_string_eq_byte_buf_ignore_case(str: *const AwsCString, buf: *const AwsCByteBuf) -> bool;
    pub fn aws_string_eq_c_str(str: *const AwsCString, c_str: *const u8) -> bool;
    pub fn aws_string_eq_c_str_ignore_case(str: *const AwsCString, c_str: *const u8) -> bool;
    pub fn aws_string_new_from_c_str(allocator: *const AwsCAllocator, c_str: *const u8) -> *mut AwsCString;
    pub fn aws_string_new_from_array(allocator: *const AwsCAllocator, bytes: *const u8, len: usize) -> *mut AwsCString;
    pub fn aws_string_new_from_cursor(
        allocator: *const AwsCAllocator,
        cursor: *const AwsCByteCursor,
    ) -> *mut AwsCString;
    pub fn aws_string_new_from_buf(allocator: *const AwsCAllocator, buf: *const AwsCByteBuf) -> *mut AwsCString;
    pub fn aws_string_destroy(str: *mut AwsCString);
    pub fn aws_string_destroy_secure(str: *mut AwsCString);
    #[must_use]
    pub fn aws_string_compare(a: *const AwsCString, b: *const AwsCString) -> i32;
    #[must_use]
    pub fn aws_byte_buf_write_from_whole_string(buf: *mut AwsCByteBuf, str: *const AwsCString) -> bool;
    pub fn aws_byte_cursor_from_string(src: *const AwsCString) -> AwsCByteCursor;
    pub fn aws_string_clone_or_reuse(allocator: *const AwsCAllocator, str: *const AwsCString) -> *mut AwsCString;
    #[must_use]
    pub fn aws_secure_strlen(str: *const u8, max_read_len: usize, str_len: *mut usize) -> i32;
    pub fn aws_string_bytes(str: *const AwsCString) -> *const u8;
    pub fn aws_string_c_str(str: *const AwsCString) -> *const u8;
    pub fn aws_string_is_valid(str: *const AwsCString) -> bool;
    pub fn aws_c_string_is_valid(str: *const u8) -> bool;
    pub fn aws_char_is_space(c: u8) -> bool;
}
