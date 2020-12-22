use heim_common::prelude::*;
use heim_common::units::{time, Time};

pub async fn uptime() -> Result<Time> {
    let nano_secs = 1024.0;

    Ok(Time::new::<time::nanosecond>(nano_secs))
}
