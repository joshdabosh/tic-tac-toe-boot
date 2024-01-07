#!/bin/bash

uefi-run \
    --boot ./uefi_app.efi \
    --add-file bzImage:/bzImage \
    --size 350 \
    -- -m 4G \
        -enable-kvm
