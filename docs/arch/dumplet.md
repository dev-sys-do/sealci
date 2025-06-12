# Dumplet Architecture

## Contributors

- BRONSIN Baptiste
- GRASSET Dorian
- PLANCHE Benoit
- THEOULLE Sarah

## What is Dumplet?

**Dumplet** is a Rust **CLI tool** that extracts the filesystem of a **Docker image** and creates an **initramfs image**.  
It uses the **Bollard** crate to interact with the Docker API.

Dumpler can pull images or use local images. It can also expose an option to add files or directories to the `initramfs.img` at build time. Moreover, you can specify the DNS server to use in your container image.

## Why Dumplet?

Dumplet is an easy way to deploy the Release Agent service into a Dumper VM.

The Release Agent's development lifecycle can be done in a container, then dumplet is used to build an initramfs image from the Release Agent's container image, or any other container image.

## How does Dumplet work?

Dumpet interfaces with a running Docker daemon via the `bollard` package. It executes commands to retrieve container images (locally or from an external registry), unpack their content, then compiles an initramfs.

