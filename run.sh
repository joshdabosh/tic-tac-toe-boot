#!/bin/bash

uefi-run \
    --boot ./target/x86_64-unknown-uefi/debug/uefi_app.efi \
    --add-file bzImage:/bzImage \
    --size 30 \
    -- -m 512M \
        -enable-kvm
