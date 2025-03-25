use std::error::Error as StdError;
use std::fmt;
use std::io;

use linux_loader::loader;

#[derive(Debug)]
pub enum Error {
    /// Failed to write boot parameters to guest memory.
    BootConfigure(linux_loader::configurator::Error),
    /// Error configuring the kernel command line.
    Cmdline(linux_loader::cmdline::Error),
    /// Failed to load kernel.
    KernelLoad(loader::Error),
    /// Invalid E820 configuration.
    E820Configuration,
    /// Highmem start address is past the guest memory end.
    HimemStartPastMemEnd,
    /// I/O error.
    IO(io::Error),
    /// Error issuing an ioctl to KVM.
    KvmIoctl(kvm_ioctls::Error),
    /// vCPU errors.
    // Vcpu(cpu::Error),
    /// Memory error.
    Memory(vm_memory::Error),
    /// Serial creation error
    SerialCreation(io::Error),
    /// IRQ registration error
    IrqRegister(io::Error),
    /// epoll creation error
    EpollError(io::Error),
    /// STDIN read error
    StdinRead(kvm_ioctls::Error),
    /// STDIN write error
    StdinWrite(vm_superio::serial::Error<io::Error>),
    /// Terminal configuration error
    TerminalConfigure(kvm_ioctls::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::BootConfigure(e) => write!(f, "Failed to configure boot parameters: {}", e),
            Error::Cmdline(e) => write!(f, "Error configuring the kernel command line: {}", e),
            Error::KernelLoad(e) => write!(f, "Failed to load kernel: {}", e),
            Error::E820Configuration => write!(f, "Invalid E820 configuration"),
            Error::HimemStartPastMemEnd => {
                write!(f, "Highmem start address is past the guest memory end")
            }
            Error::IO(e) => write!(f, "I/O error: {}", e),
            Error::KvmIoctl(e) => write!(f, "Error issuing an ioctl to KVM: {}", e),
            Error::Memory(e) => write!(f, "Memory error: {}", e),
            Error::SerialCreation(e) => write!(f, "Serial creation error: {}", e),
            Error::IrqRegister(e) => write!(f, "IRQ registration error: {}", e),
            Error::EpollError(e) => write!(f, "epoll creation error: {}", e),
            Error::StdinRead(e) => write!(f, "STDIN read error: {}", e),
            Error::StdinWrite(_) => write!(f, "STDIN write error"),
            Error::TerminalConfigure(e) => write!(f, "Terminal configuration error: {}", e),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::BootConfigure(e) => Some(e),
            Error::Cmdline(e) => Some(e),
            Error::KernelLoad(e) => Some(e),
            Error::IO(e) => Some(e),
            Error::KvmIoctl(e) => Some(e),
            Error::Memory(e) => Some(e),
            Error::SerialCreation(e) => Some(e),
            Error::IrqRegister(e) => Some(e),
            Error::EpollError(e) => Some(e),
            Error::StdinRead(e) => Some(e),
            Error::StdinWrite(_) => None,
            Error::TerminalConfigure(e) => Some(e),
            _ => None,
        }
    }
}
