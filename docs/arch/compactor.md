# Dumplet Architecture

## What?

Compactor is a Rust-based tool designed to streamline the process of exporting Docker images and creating initramfs images. It leverages the Dumplet and Dumper libraries to provide a seamless experience for managing virtual machine configurations and kernel initialization.

Compactor is a "happy default" way of running Dumplet and Dumper together to run a VM. It's an easy way for developers to improve and iterate upon Dumplet and Dumper.

## Why Compactor?

Modern containerized applications often require efficient ways to package and deploy environments. Compactor simplifies this process by automating the creation of initramfs images from Docker containers, enabling lightweight and portable virtual machine setups. This tool is ideal for developers and system administrators looking to bridge the gap between containerization and virtualization.

## How does Compactor work?

Compactor operates in two main steps:

1. **Docker Image Export**: Using the Dumplet library, it extracts the specified Docker image and generates an initramfs image. Environment variables and file transfers can be specified to customize the image.

2. **Virtual Machine Initialization**: The Dumper library is used to configure and launch a virtual machine with the generated initramfs and kernel image (`vmlinux`).

## Components

**Kernel:**

The `kernel.rs` module includes the kernel image (`vmlinux`) as a static byte array. This kernel is used to boot the virtual machine.

**Core Logic:**

The `lib.rs` module contains the main logic for Compactor. It:

- Generates the initramfs image using Dumplet
- Configures the virtual machine using Dumper
- Provides methods to initialize and run the virtual machine
