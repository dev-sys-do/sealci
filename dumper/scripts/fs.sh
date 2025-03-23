#!/usr/bin/bash

curl -O https://dl-cdn.alpinelinux.org/alpine/v3.14/releases/x86_64/alpine-minirootfs-3.14.2-x86_64.tar.gz

mkdir alpine-minirootfs
tar xf alpine-minirootfs-3.14.2-x86_64.tar.gz -C alpine-minirootfs

pushd alpine-minirootfs
cat > init <<EOF
#! /bin/sh
#
# /init executable file in the initramfs 
#
mount -t devtmpfs dev /dev
mount -t proc proc /proc
mount -t sysfs sysfs /sys
ip link set up dev lo

exec /sbin/getty -n -l /bin/sh 115200 /dev/console
poweroff -f
EOF

chmod +x init

find . -print0 |
    cpio --null --create --verbose --owner root:root --format=newc |
    xz -9 --format=lzma  > ../initramfs.img

popd
