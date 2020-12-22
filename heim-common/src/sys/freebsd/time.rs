use crate::sys::IntoTime;
use crate::units::{time, Time};

impl IntoTime for libc::timeval {
    fn into_time(self) -> Time {
        Time::new::<time::second>(self.tv_sec as f64)
            + Time::new::<time::microsecond>(self.tv_usec as f64)
    }
}
