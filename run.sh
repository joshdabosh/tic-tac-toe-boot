#!/bin/bash

uefi-run \
    --boot ./uefi_app.efi \
    --add-file grub.efi:/bzImage \
    --add-file grub.cfg:/grub.cfg \
    --size 350 \
    -- -m 4G \
        -enable-kvm \
        -nographic