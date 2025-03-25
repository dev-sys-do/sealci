// SPDX-License-Identifier: Apache-2.0

use std::io::{stdout, Error, Result, Stdout};
use std::ops::Deref;

use vm_superio::serial::NoEvents;
use vm_superio::{Serial, Trigger};
use vmm_sys_util::eventfd::EventFd;

pub const SERIAL_PORT_BASE: u16 = 0x3f8;
pub const SERIAL_PORT_LAST_REGISTER: u16 = SERIAL_PORT_BASE + 0x8;

pub struct EventFdTrigger(EventFd);

impl Trigger for EventFdTrigger {
    type E = Error;

    fn trigger(&self) -> Result<()> {
        self.write(1)
    }
}

impl Deref for EventFdTrigger {
    type Target = EventFd;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl EventFdTrigger {
    pub fn new(flag: i32) -> Result<Self> {
        Ok(EventFdTrigger(EventFd::new(flag)?))
    }
    pub fn try_clone(&self) -> Result<Self> {
        Ok(EventFdTrigger((**self).try_clone()?))
    }
}

pub(crate) struct DumperSerial {
    // evenfd allows for the device to send interrupts to the guest.
    eventfd: EventFdTrigger,

    // serial is the actual serial device.
    serial: Serial<EventFdTrigger, NoEvents, Stdout>,
}

impl DumperSerial {
    pub fn new() -> Result<Self> {
        let eventfd = EventFdTrigger::new(libc::EFD_NONBLOCK).unwrap();

        Ok(DumperSerial {
            eventfd: eventfd.try_clone()?,
            serial: Serial::new(eventfd.try_clone()?, stdout()),
        })
    }

    pub fn eventfd(&self) -> Result<EventFd> {
        Ok(self.eventfd.try_clone()?.0)
    }
}
