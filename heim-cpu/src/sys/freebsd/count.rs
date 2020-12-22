use heim_common::prelude::Result;
use heim_common::sys::freebsd::sysctl;

pub async fn logical_count() -> Result<u64> {
    //unsafe { sysctl::sysctlbyname(b"hw.logicalcpu\0") }
    unsafe { sysctl::sysctlbyname(b"hw.ncpu\0") }
}

pub async fn physical_count() -> Result<Option<u64>> {
    //unsafe { sysctl::sysctlbyname(b"hw.physicalcpu\0").map(Some) }
    unsafe { sysctl::sysctlbyname(b"hw.ncpu\0") }
}
