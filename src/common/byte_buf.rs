use {
    crate::common::{AwsAllocator, AwsCAllocator},
    std::{
        error::Error,
        ffi::{c_void, CString},
        ptr::null_mut,
        slice::{from_raw_parts, from_raw_parts_mut},
    },
};

pub struct AwsByteBuf {
    pub(crate) inner: AwsCByteBuf,
}

impl AwsByteBuf {
    /// Create a new byte buffer with the given capacity.
    pub fn new(
        allocator: &AwsAllocator,
        capacity: usize,
    ) -> Result<AwsByteBuf, Box<dyn Error + Send + Sync + 'static>> {
        let mut buf = AwsCByteBuf {
            len: 0,
            buffer: null_mut(),
            capacity,
            allocator: allocator.inner,
        };

        let init_ok = unsafe { aws_byte_buf_init(&mut buf, allocator.inner, capacity) };
        if init_ok != 0 {
            return Err("Failed to initialize byte buffer".into());
        }

        Ok(AwsByteBuf {
            inner: buf,
        })
    }

    /// Create a new byte buffer from the given string.
    pub fn from_str(allocator: &AwsAllocator, s: &str) -> Result<AwsByteBuf, Box<dyn Error + Send + Sync + 'static>> {
        let s = CString::new(s.as_bytes())?;
        let s_bytes = s.into_bytes_with_nul();

        let mut buf = AwsCByteBuf {
            len: 0,
            buffer: null_mut(),
            capacity: 0,
            allocator: allocator.inner,
        };

        let init_ok = unsafe { aws_byte_buf_init(&mut buf, allocator.inner, s_bytes.len()) };
        if init_ok != 0 {
            return Err("Failed to initialize byte buffer".into());
        }

        let dst = unsafe { from_raw_parts_mut(buf.buffer, buf.capacity) };
        dst[..s_bytes.len()].copy_from_slice(&s_bytes[..s_bytes.len()]);
        buf.len = s_bytes.len();
        Ok(AwsByteBuf {
            inner: buf,
        })
    }

    pub fn secure_zero(&mut self) {
        unsafe { aws_byte_buf_secure_zero(&mut self.inner) };
    }

    pub fn is_valid(&self) -> bool {
        unsafe { aws_byte_buf_is_valid(&self.inner) }
    }

    pub fn to_string_lossy(&self) -> String {
        let len = self.inner.len;
        let mut s = unsafe { from_raw_parts(self.inner.buffer, len) };
        if s[len - 1] == b'\0' {
            s = s[..len - 1].as_ref();
        }

        String::from_utf8_lossy(s).into_owned()
    }
}

impl Clone for AwsByteBuf {
    fn clone(&self) -> Self {
        let allocator = self.inner.allocator;
        let mut buf = AwsCByteBuf {
            len: 0,
            buffer: null_mut(),
            capacity: 0,
            allocator,
        };

        let init_ok = unsafe { aws_byte_buf_init_copy(&mut buf, allocator, &self.inner) };
        if init_ok != 0 {
            panic!("Failed to initialize byte buffer");
        }

        AwsByteBuf {
            inner: buf,
        }
    }
}

impl Drop for AwsByteBuf {
    fn drop(&mut self) {
        unsafe { aws_byte_buf_clean_up(&mut self.inner) };
    }
}

#[repr(C)]
pub(crate) struct AwsCByteBuf {
    pub len: usize,
    pub buffer: *mut u8,
    pub capacity: usize,
    pub allocator: *const AwsCAllocator,
}

unsafe impl Send for AwsCByteBuf {}
unsafe impl Sync for AwsCByteBuf {}

#[repr(C)]
pub struct AwsByteCursor {
    pub len: usize,
    pub ptr: *const u8,
}

unsafe impl Send for AwsByteCursor {}
unsafe impl Sync for AwsByteCursor {}

#[link(name = "aws-c-common")]
extern "C" {
    pub fn aws_array_eq(array_a: *const c_void, len_a: usize, array_b: *const c_void, len_b: usize) -> bool;
    pub fn aws_array_eq_ignore_case(array_a: *const c_void, len_a: usize, array_b: *const c_void, len_b: usize)
        -> bool;
    pub fn aws_array_eq_c_str(array: *const c_void, array_len: usize, c_str: *const u8) -> bool;
    pub fn aws_array_eq_c_str_ignore_case(array: *const c_void, array_len: usize, c_str: *const u8) -> bool;
    #[must_use]
    pub(crate) fn aws_byte_buf_init(buf: *mut AwsCByteBuf, allocator: *const AwsCAllocator, capacity: usize) -> i32;
    #[must_use]
    pub(crate) fn aws_byte_buf_init_copy(
        dest: *mut AwsCByteBuf,
        allocator: *const AwsCAllocator,
        src: *const AwsCByteBuf,
    ) -> i32;

    #[must_use]
    #[allow(dead_code)]
    pub(crate) fn aws_byte_buf_init_from_file(
        out_buf: *mut AwsCByteBuf,
        allocator: *const AwsCAllocator,
        filename: *const u8,
    ) -> i32;

    pub(crate) fn aws_byte_buf_is_valid(buf: *const AwsCByteBuf) -> bool;

    pub fn aws_byte_cursor_is_valid(cursor: *const AwsByteCursor) -> bool;

    #[must_use]
    #[allow(dead_code)]
    pub(crate) fn aws_byte_buf_init_copy_from_cursor(
        buf: *mut AwsCByteBuf,
        allocator: *const AwsCAllocator,
        src: AwsByteCursor,
    ) -> i32;
    #[must_use]
    #[allow(dead_code)]
    pub(crate) fn aws_byte_buf_init_cache_and_update_cursors(
        buf: *mut AwsCByteBuf,
        allocator: *const AwsCAllocator,
        ...
    ) -> i32;

    pub(crate) fn aws_byte_buf_clean_up(buf: *mut AwsCByteBuf);

    #[allow(dead_code)]
    pub(crate) fn aws_byte_buf_clean_up_secure(buf: *mut AwsCByteBuf);

    #[allow(dead_code)]
    pub(crate) fn aws_byte_buf_reset(buf: *mut AwsCByteBuf, zero: bool);

    pub(crate) fn aws_byte_buf_secure_zero(buf: *mut AwsCByteBuf);

    #[allow(dead_code)]
    pub(crate) fn aws_byte_buf_eq(a: *const AwsCByteBuf, b: *const AwsCByteBuf) -> bool;

    #[allow(dead_code)]
    pub(crate) fn aws_byte_buf_eq_ignore_case(a: *const AwsCByteBuf, b: *const AwsCByteBuf) -> bool;

    #[allow(dead_code)]
    pub(crate) fn aws_byte_buf_eq_c_str(a: *const AwsCByteBuf, b: *const u8) -> bool;

    #[allow(dead_code)]
    pub(crate) fn aws_byte_buf_eq_c_str_ignore_case(a: *const AwsCByteBuf, b: *const u8) -> bool;

    pub fn aws_byte_cursor_next_split(
        input_str: *const AwsByteCursor,
        split_on: u8,
        substr: *const AwsByteCursor,
    ) -> bool;
}
