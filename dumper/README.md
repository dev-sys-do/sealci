# Dumper

![Dumper Logo](https://cdn.discordapp.com/attachments/1351126477852119060/1352683557264294069/dumper_lumper_logo.jpg?ex=67dee83b&is=67dd96bb&hm=36c450eb3ee634e3bb54f632ea9ade34cbe24b988237c27f442cd87c55f751d5&)


## Quick start

If you are not already in the dumper directory:
```sh
mkdir dumper/vm
cd dumper/vm
```

Get an alpine linux filesystem:
```sh
../scripts/fs.sh
```

Setup a kernel:
```sh
../scripts/kernel.sh
```
This one can take a while be patient.

Let's go to example to run the VM:
```sh
cd ../examples/quickstart/
```

Then you can run the VM:
```sh
cargo run -- --kernel-path ../../vm/linux-cloud-hypervisor/vmlinux
```