#!/usr/bin/bash

LINUX_REPO=linux-cloud-hypervisor

if [ ! -d $LINUX_REPO ]
then
    git clone --depth 1 "https://github.com/cloud-hypervisor/linux.git" -b "ch-6.2" $LINUX_REPO
fi

pushd $LINUX_REPO
cp ../../scripts/config .config
KCFLAGS="-Wa,-mx86-used-note=no" make bzImage -j `nproc`
popd