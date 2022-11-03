mod allocator;
mod byte_buf;
mod date_time;

pub use {
    allocator::*, byte_buf::*, date_time::*,
};

#[link(name = "aws-c-common")]
extern "C" {
    pub fn aws_common_library_init(allocator: *const AwsCAllocator);
    pub fn aws_common_library_clean_up();
    pub fn aws_common_fatal_assert_library_initialized();
}
