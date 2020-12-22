use std::str::{self, FromStr};

use heim_common::prelude::*;
use heim_common::units::{time, Time};

use heim_runtime as rt;

//use super::bindings;

#[derive(Debug, Default, Clone)]
pub struct CpuTime {
    user: Time,
    nice: Time,
    system: Time,
    idle: Time,
}

impl CpuTime {
    pub fn user(&self) -> Time {
        self.user
    }

    pub fn nice(&self) -> Time {
        self.nice
    }

    pub fn system(&self) -> Time {
        self.system
    }

    pub fn idle(&self) -> Time {
        self.idle
    }
}

impl FromStr for CpuTime {
    type Err = Error;

    // Parse one line from the /proc/stat, ex.
    // "cpu1 317865 456 71065 3101075 8645 14938 10567 0 0 0"
    fn from_str(value: &str) -> Result<CpuTime> {
        let mut times = CpuTime::default();

        let parts = value.split_whitespace().skip(1);
        for (idx, part) in parts.enumerate() {
            let value = part.parse::<u32>().map(|value| {
                let value = f64::from(value);
                Time::new::<time::second>(value)
            })?;

            match idx {
                0 => times.user = value,
                1 => times.nice = value,
                2 => times.system = value,
                3 => times.idle = value,
                _ => break,
            };
        }

        Ok(times)
    }
}
 
// impl From<bindings::host_cpu_load_info> for CpuTime {
//     fn from(info: bindings::host_cpu_load_info) -> CpuTime {
//         let ticks = *CLOCK_TICKS;
// 
//         CpuTime {
//             user: Time::new::<time::second>(f64::from(info.user) / ticks),
//             nice: Time::new::<time::second>(f64::from(info.nice) / ticks),
//             system: Time::new::<time::second>(f64::from(info.system) / ticks),
//             idle: Time::new::<time::second>(f64::from(info.idle) / ticks),
//         }
//     }
// }
// 
// impl From<bindings::processor_cpu_load_info> for CpuTime {
//     fn from(info: bindings::processor_cpu_load_info) -> CpuTime {
//         let ticks = *CLOCK_TICKS;
// 
//         CpuTime {
//             user: Time::new::<time::second>(f64::from(info.user) / ticks),
//             nice: Time::new::<time::second>(f64::from(info.nice) / ticks),
//             system: Time::new::<time::second>(f64::from(info.system) / ticks),
//             idle: Time::new::<time::second>(f64::from(info.idle) / ticks),
//         }
//     }
// }

pub async fn time() -> Result<CpuTime> {
//    bindings::cpu_load_info().map(Into::into)
    let user = Time::new::<time::second>(1024.0);
    let nice = Time::new::<time::second>(1024.0);
    let system = Time::new::<time::second>(1024.0);
    let idle = Time::new::<time::second>(1024.0);
    Ok(CpuTime {
        user,
        nice,
        system,
        idle,
    })
}

pub async fn times() -> Result<impl Stream<Item = Result<CpuTime>>> {
    let lines = rt::fs::read_lines("/proc/stat").await?;

    let stream = lines.skip(1).filter_map(|try_line| async move {
        match try_line {
            Ok(line) if line.starts_with("cpu") => Some(CpuTime::from_str(&line)),
            Ok(..) => None,
            Err(e) => Some(Err(e.into())),
        }
    });

    Ok(stream)
}
