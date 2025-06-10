# Compactor

## Overview

Compactor is a Rust-based tool designed to streamline the process of exporting Docker images and creating initramfs images. It leverages the Dumplet and Dumper libraries to provide a seamless experience for managing virtual machine configurations and kernel initialization.

## Why Compactor?

Modern containerized applications often require efficient ways to package and deploy environments. Compactor simplifies this process by automating the creation of initramfs images from Docker containers, enabling lightweight and portable virtual machine setups. This tool is ideal for developers and system administrators looking to bridge the gap between containerization and virtualization.

## How It Works

Compactor operates in two main steps:

1. **Docker Image Export**: Using the Dumplet library, it extracts the specified Docker image and generates an initramfs image. Environment variables and file transfers can be specified to customize the image.

2. **Virtual Machine Initialization**: The Dumper library is used to configure and launch a virtual machine with the generated initramfs and kernel image (`vmlinux`).

## Components

### Kernel
The `kernel.rs` module includes the kernel image (`vmlinux`) as a static byte array. This kernel is used to boot the virtual machine.


### Core Logic
The `lib.rs` module contains the main logic for Compactor. It:
- Generates the initramfs image using Dumplet
- Configures the virtual machine using Dumper
- Provides methods to initialize and run the virtual machine

## Usage

To use Compactor, run the following command with root capabilities:

```bash
cargo run -- <docker-image-name> --env KEY=VALUE --transfer-files /host/path:/guest/path
```

### Example

Setup a tap interface for networking:
```sh
ip tuntap add mode tap tap0
ip link set tap0 up
```


```bash
cargo run -- alpine:3.14 --env DEBUG=true LOG_LEVEL=info --transfer-files /tmp/data:/app/data --tap-interface-name tap0
```

This command will:
- Export the `alpine:3.14` Docker image
- Pass the environment variables `DEBUG=true` and `LOG_LEVEL=info`
- Transfer the file `/tmp/data` on the host to `/app/data` on the guest

## Requirements

- Rust (latest stable version)
- Docker installed and running
- Tap interface (`tap0`) configured for networking
