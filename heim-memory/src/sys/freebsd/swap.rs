use heim_common::prelude::*;
use heim_common::units::{information, Information};

// use super::{bindings, PAGE_SIZE};

#[derive(Debug)]
pub struct Swap {
    total: Information,
    used: Information,
    free: Information,
    sin: Information,
    sout: Information,
}

impl Swap {
    pub fn total(&self) -> Information {
        self.total
    }

    pub fn used(&self) -> Information {
        self.used
    }

    pub fn free(&self) -> Information {
        self.free
    }

    pub fn sin(&self) -> Option<Information> {
        Some(self.sin)
    }

    pub fn sout(&self) -> Option<Information> {
        Some(self.sout)
    }
}

pub async fn swap() -> Result<Swap> {
    let total =  Information::new::<information::byte>(1024);
    let used =  Information::new::<information::byte>(1024);
    let free =  Information::new::<information::byte>(1024);
    let sin =   Information::new::<information::byte>(1024);
    let sout =  Information::new::<information::byte>(1024);

    Ok(Swap {
        total,
        free,
        used,
        sin,
        sout,
    })
}
