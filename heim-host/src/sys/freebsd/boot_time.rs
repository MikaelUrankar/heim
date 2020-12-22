use heim_common::{
    sys::freebsd::sysctl,
    units::Time,
    Result,
};

use heim_common::units::time;

pub async fn boot_time() -> Result<Time> {
    let value: libc::timeval = sysctl::sysctl(&mut [libc::CTL_KERN, libc::KERN_BOOTTIME])?;

    Ok(Time::new::<time::second>(value.tv_sec as f64)
            + Time::new::<time::microsecond>(value.tv_usec as f64))
    //Ok(value.into_time())
}
