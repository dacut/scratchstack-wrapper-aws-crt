use {
    crate::common::{AwsAllocator, AwsByteBuf, AwsByteCursor, AwsCByteBuf},
    std::{
        fmt::{Debug, Display, Formatter, Result as FmtResult},
        ffi::CString,
        mem::zeroed,
    },
};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum AwsDateFormat {
    Rfc822,
    Iso8601,
    Iso8601Basic,
    AutoDetect,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum AwsDateMonth {
    January = 0,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    Septemer,
    October,
    November,
    December,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum AwsDateDayOfWeek {
    Sunday = 0,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

pub struct AwsDateTime {
    pub(crate) inner: AwsCDateTime,
}

impl AwsDateTime {
    pub fn now() -> AwsDateTime {
        let mut dt = AwsCDateTime::default();
        unsafe { aws_date_time_init_now(&mut dt) };
        AwsDateTime {
            inner: dt,
        }
    }

    pub fn from_epoch_millis(ms_since_epoch: u64) -> AwsDateTime {
        let mut dt = AwsCDateTime::default();
        unsafe { aws_date_time_init_epoch_millis(&mut dt, ms_since_epoch) };
        AwsDateTime {
            inner: dt,
        }
    }

    pub fn from_epoch_secs(sec_ms: f64) -> AwsDateTime {
        let mut dt = AwsCDateTime::default();
        unsafe { aws_date_time_init_epoch_secs(&mut dt, sec_ms) };
        AwsDateTime {
            inner: dt,
        }
    }

    pub fn from_str(
        date_str: &str,
        fmt: AwsDateFormat,
    ) -> Result<AwsDateTime, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let date_str = AwsByteBuf::from_str(&AwsAllocator::default(), date_str)?;
        let mut dt = AwsCDateTime::default();
        unsafe { aws_date_time_init_from_str(&mut dt, &date_str.inner, fmt) };
        Ok(AwsDateTime {
            inner: dt,
        })
    }

    pub fn get_timestamp(&self) -> libc::time_t {
        self.inner.timestamp
    }

    pub fn to_local_time_str(&self, format: AwsDateFormat) -> String {
        let mut buf = AwsByteBuf::new(&AwsAllocator::default(), 32).unwrap();
        unsafe { aws_date_time_to_local_time_str(&self.inner, format, &mut buf.inner) };
        buf.to_string_lossy()
    }

    pub fn to_utc_time_str(&self, format: AwsDateFormat) -> String {
        let mut buf = AwsByteBuf::new(&AwsAllocator::default(), 32).unwrap();
        unsafe { aws_date_time_to_utc_time_str(&self.inner, format, &mut buf.inner) };
        buf.to_string_lossy()
    }
}

fn debug_tm(tm: &libc::tm) -> String {
    let tm_zone = if tm.tm_zone.is_null() {
        "null".to_string()
    } else {
        String::from_utf8_lossy(&unsafe { CString::from_raw(tm.tm_zone as *mut i8) }.into_bytes()).into_owned()
    };

    format!(
        "tm_year={}, tm_mon={}, tm_mday={}, tm_hour={}, tm_min={}, tm_sec={}, tm_wday={}, tm_yday={}, tm_isdst={} tm_gmtoff={} tm_zone={}",
        tm.tm_year, tm.tm_mon, tm.tm_mday, tm.tm_hour, tm.tm_min, tm.tm_sec, tm.tm_wday, tm.tm_yday, tm.tm_isdst, tm.tm_gmtoff, tm_zone,
    )
}

impl Debug for AwsDateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("AwsDateTime")
            .field("timestamp", &self.inner.timestamp)
            .field("milliseconds", &self.inner.milliseconds)
            .field("tz", &self.inner.tz)
            .field("gmt_time", &debug_tm(&self.inner.gmt_time))
            .field("local_time", &debug_tm(&self.inner.local_time))
            .field("utc_assumed", &self.inner.utc_assumed)
            .finish()
    }
}

impl Display for AwsDateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(&self.to_utc_time_str(AwsDateFormat::Iso8601))
    }
}

#[repr(C)]
pub struct AwsCDateTime {
    timestamp: libc::time_t,
    milliseconds: u16,
    tz: [u8; 6],
    gmt_time: libc::tm,
    local_time: libc::tm,
    utc_assumed: bool,
}

impl Default for AwsCDateTime {
    fn default() -> Self {
        AwsCDateTime {
            timestamp: 0,
            milliseconds: 0,
            tz: [0; 6],
            gmt_time: unsafe { zeroed() },
            local_time: unsafe { zeroed() },
            utc_assumed: false,
        }
    }
}

#[link(name = "aws-c-common")]
extern "C" {
    pub fn aws_date_time_init_now(dt: *mut AwsCDateTime);

    pub fn aws_date_time_init_epoch_millis(dt: *mut AwsCDateTime, ms_since_epoch: u64);

    pub fn aws_date_time_init_epoch_secs(dt: *mut AwsCDateTime, sec_ms: f64);

    #[must_use]
    pub(crate) fn aws_date_time_init_from_str(
        dt: *mut AwsCDateTime,
        date_str: *const AwsCByteBuf,
        fmt: AwsDateFormat,
    ) -> i32;

    pub fn aws_date_time_init_from_str_cursor(
        dt: *mut AwsCDateTime,
        date_str_cursor: *const AwsByteCursor,
        fmt: AwsDateFormat,
    ) -> i32;

    #[must_use]
    #[allow(dead_code)]
    pub(crate) fn aws_date_time_to_local_time_str(
        dt: *const AwsCDateTime,
        fmt: AwsDateFormat,
        output_buf: *mut AwsCByteBuf,
    ) -> i32;

    #[must_use]
    #[allow(dead_code)]
    pub(crate) fn aws_date_time_to_utc_time_str(
        dt: *const AwsCDateTime,
        fmt: AwsDateFormat,
        output_buf: *mut AwsCByteBuf,
    ) -> i32;
}

#[cfg(test)]
mod tests {
    use {
        crate::common::{AwsDateFormat, AwsDateTime},
        log::debug,
        pretty_assertions::assert_eq,
    };

    #[test_log::test]
    fn test_conversions() {
        let now = AwsDateTime::now();
        let then = AwsDateTime::from_str("2010-01-01T00:00:00Z", AwsDateFormat::Iso8601).unwrap();

        debug!("now={}, then={}", now.get_timestamp(), then.get_timestamp());
        assert!(now.get_timestamp() > then.get_timestamp());

        assert_eq!(format!("{}", then), "2010-01-01T00:00:00Z");
    }
}
