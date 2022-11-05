use {
    crate::common::{AwsCAllocator, AwsCArrayList, AWS_C_COMMON_PACKAGE_ID},
    std::ffi::c_void,
};

pub type AwsCCrtStatisticsCategory = u32;

pub const AWS_CRT_STATISTICS_CATEGORY_STRIDE_BITS: usize = 8;
pub const AWS_CRT_STATISTICS_CATEGORY_STRIDE: u32 = 1 << AWS_CRT_STATISTICS_CATEGORY_STRIDE_BITS;
pub const fn aws_crt_statistics_category_begin_range(x: isize) -> u32 {
    x as u32 * AWS_CRT_STATISTICS_CATEGORY_STRIDE
}

pub const fn aws_crt_statistics_category_end_range(x: isize) -> u32 {
    (x as u32 + 1) * AWS_CRT_STATISTICS_CATEGORY_STRIDE - 1
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AwsCCrtCommonStatisticsCategory {
    AWSCRT_STAT_CAT_INVALID = aws_crt_statistics_category_begin_range(AWS_C_COMMON_PACKAGE_ID),
}

#[repr(C)]
pub struct AwsCCrtStatisticsBase {
    pub category: AwsCCrtStatisticsCategory,
}

#[repr(C)]
pub struct AwsCCrtStatisticsSampleInterval {
    pub begin_time_ms: u64,
    pub end_time_ms: u64,
}

pub type AwsCCrtStatisticsHandlerProcessStatisticsFn = extern "C" fn(
    handler: *mut AwsCCrtStatisticsHandler,
    interval: *mut AwsCCrtStatisticsSampleInterval,
    stats: *mut AwsCArrayList,
    context: *mut c_void,
);

pub type AwsCCrtStatisticsHandlerDestroyFn = extern "C" fn(handler: *mut AwsCCrtStatisticsHandler);

pub type AwsCCrtStatisticsHandlerGetReportIntervalMsFn = extern "C" fn(handler: *mut AwsCCrtStatisticsHandler) -> u64;

#[repr(C)]
pub struct AwsCCrtStatisticsHandlerVtable {
    pub process_statistics: AwsCCrtStatisticsHandlerProcessStatisticsFn,
    pub destroy: AwsCCrtStatisticsHandlerDestroyFn,
    pub get_report_interval_ms: AwsCCrtStatisticsHandlerGetReportIntervalMsFn,
}

#[repr(C)]
pub struct AwsCCrtStatisticsHandler {
    pub vtable: *mut AwsCCrtStatisticsHandlerVtable,
    pub allocator: *const AwsCAllocator,
    pub r#impl: *mut c_void,
}

#[link(name = "aws-crt-common")]
extern "C" {
    pub fn aws_crt_statistics_handler_process_statistics(
        handler: *mut AwsCCrtStatisticsHandler,
        interval: *mut AwsCCrtStatisticsSampleInterval,
        stats: *mut AwsCArrayList,
        context: *mut c_void,
    );

    pub fn aws_crt_statistics_handler_get_report_interval_ms(handler: *mut AwsCCrtStatisticsHandler) -> u64;

    pub fn aws_crt_statistics_handler_destroy(handler: *mut AwsCCrtStatisticsHandler);
}
