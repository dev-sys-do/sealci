use std::path::PathBuf;

use kvm_bindings::kvm_userspace_memory_region;
use kvm_ioctls::{Kvm, VmFd};
use vm_memory::{Address, GuestAddress, GuestMemory, GuestMemoryMmap, GuestMemoryRegion};

use crate::{common::error::Error, config::vmm::VmmConfig, kernel};

pub struct VMM {
    vm_fd: VmFd,
    kvm: Kvm,
    guest_memory: GuestMemoryMmap,
}

impl VMM {
    pub fn new() -> Result<Self, Error> {
        let kvm = Kvm::new().map_err(Error::KvmIoctl)?;
        let vm_fd = kvm.create_vm().map_err(Error::KvmIoctl)?;

        let vmm = VMM {
            vm_fd,
            kvm,
            guest_memory: GuestMemoryMmap::default(),
        };

        Ok(vmm)
    }

    pub fn configure_memory(&mut self, mem_size_mb: u32) -> Result<(), Error> {
        // Convert memory size from MBytes to bytes.
        let mem_size = ((mem_size_mb as u64) << 20) as usize;

        // Create one single memory region, from zero to mem_size.
        let mem_regions = vec![(GuestAddress(0), mem_size)];

        // Allocate the guest memory from the memory region.
        let guest_memory = GuestMemoryMmap::from_ranges(&mem_regions).map_err(Error::Memory)?;

        // For each memory region in guest_memory:
        // 1. Create a KVM memory region mapping the memory region guest physical address to the host virtual address.
        // 2. Register the KVM memory region with KVM. EPTs are created then.
        for (index, region) in guest_memory.iter().enumerate() {
            let kvm_memory_region = kvm_userspace_memory_region {
                slot: index as u32,
                guest_phys_addr: region.start_addr().raw_value(),
                memory_size: region.len() as u64,
                // It's safe to unwrap because the guest address is valid.
                userspace_addr: guest_memory.get_host_address(region.start_addr()).unwrap() as u64,
                flags: 0,
            };

            // Register the KVM memory region with KVM.
            unsafe { self.vm_fd.set_user_memory_region(kvm_memory_region) }
                .map_err(Error::KvmIoctl)?;
        }

        self.guest_memory = guest_memory;

        Ok(())
    }
    pub fn configure(&mut self, config: VmmConfig) -> Result<(), Error> {
        self.configure_memory(config.mem_size_mb)?;
        kernel::kernel_setup(&self.guest_memory, PathBuf::from(config.kernel_path))?;
        Ok(())
    }
}
