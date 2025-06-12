# Dumper Architecture

![Dumper Logo](./docs/images/DUMPER.png)

## Contributors

DURAT Mathias, PONTHIEU Hugo, TCHILINGUIRIAN Théo.

## What is Dumper?

Dumper is a lightweight virtual machine manager (VMM) based on [lumper](<https://github.com/virt-do/lumper>) and [rust-vmm's reference implementation](<https://github.com/rust-vmm/vmm-reference>), designed to run SealCI's Release Agent in a virtualized environment to minimize the attack surface over this critical component.

Dumper:

- Leverages QEMU/KVM for virtualization.
- Takes paths to a filesystem image and compiled kernel binary as input.
  - Dumplet can generate a filesystem image from a container image (local or remote), but requires Docker.
  - Dumplet can copy a file from the host to the filesystem image during generation, this is required to transmit secret keys from the host to the Release Agent inside the Dumper VM.
  - Compactor is a CLI frontend to Dumplet and Dumper to launch VMs quickly.
- Configures network connection via virtio-net.
  - This is done to allow the release agent to clone remote Git repositories.

## Why Dumper?

SealCI's Release Agent needs to run inside a virtual machine for security reasons, yet be able to fetch a git repository from the network, and obtain a secret key from the host.  
From this need arised Dumper, SealCI's own purpose-built VMM. Apart from an amazing learning opportunity, its design is simple, we are able to understand it and build on it to enhance it. As a purpose-built VMM, it aims to fill its needs only, reducing attack surface and improving performance.

## How does Dumper work?

Dumper works on Linux by leveraging QEMU and KVM to create ../scripts/kernel.sh
virtual machines from a given kernel and filesystem image.  
Givent as inputs to Dumper are:

- A path to a filesystem image.
- A compiled kernel.
- Optionally, the name of the configured host TAP network device to configure virtio-net in the VM.

### Core Components

1. **VMM (`src/vmm`)**:
   - The main entry point for managing virtual machines.
   - Handles configuration, initialization, and execution of virtual CPUs (vCPUs), memory, and devices.
   - Provides methods for configuring memory, CPUs, and networking.

2. **CPU (`src/cpu`)**:
   - Manages vCPU creation and configuration.
   - Implements features like CPUID filtering, MSR configuration, and guest memory management.
   - Handles VM exits and emulation loops.

3. **Devices (`src/devices`)**:
   - Provides support for virtual devices such as serial ports, virtio devices, and networking.
   - Includes abstractions for MMIO and PIO device management.

4. **Kernel (`src/kernel`)**:
   - Handles kernel loading and initialization.
   - Configures boot parameters and memory mappings.

5. **Common Utilities (`src/common`)**:
   - Contains shared utilities such as error handling and logging.

### VM creation workflow

1. **Initialization**:
   - Dumper is initialized with configuration parameters such as memory size, number of vCPUs, and kernel paths.
   - Memory regions are allocated and registered with KVM.

2. **Device Setup**:
   - Virtual devices are configured and registered with Dumper.
   - Networking devices are initialized using TAP interfaces.

3. **vCPU Configuration**:
   - vCPUs are created and configured with CPUID, MSRs, and memory mappings.
   - The emulation loop is started for each vCPU.

4. **Execution**:
   - Dumper runs the virtual machine, handling VM exits and device events.

## Example Usage

See [Dumper's README.md](<../../dumper/README.md>) for a quick setup and more information.
