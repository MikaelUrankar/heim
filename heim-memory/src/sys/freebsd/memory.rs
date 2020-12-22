use heim_common::prelude::*;
use heim_common::units::{information, Information};

#[derive(Debug)]
pub struct Memory {
    total: Information,
    available: Information,
    used: Information,
    free: Information,
    active: Information,
    inactive: Information,
    wire: Information,
}

impl Memory {
    pub fn total(&self) -> Information {
        self.total
    }

    pub fn available(&self) -> Information {
        self.available
    }

    pub fn used(&self) -> Information {
        self.used
    }

    pub fn free(&self) -> Information {
        self.free
    }

    pub fn active(&self) -> Information {
        self.active
    }

    pub fn inactive(&self) -> Information {
        self.inactive
    }

    pub fn wire(&self) -> Information {
        self.wire
    }
}

pub async fn memory() -> Result<Memory> {
    let total =     Information::new::<information::byte>(1024);
    let available = Information::new::<information::byte>(1024);
    let free =      Information::new::<information::byte>(1024);
    let used =      Information::new::<information::byte>(1024);
    let active =    Information::new::<information::byte>(1024);
    let inactive =  Information::new::<information::byte>(1024);
    let wire =      Information::new::<information::byte>(1024);

    Ok(Memory {
        total,
        available,
        free,
        used,
        active,
        inactive,
        wire,
    })
}
