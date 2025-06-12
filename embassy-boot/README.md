# embassy-boot

An [Embassy](https://embassy.dev) project.

A lightweight bootloader supporting firmware updates in a power-fail-safe way, with trial boots and rollbacks.

The bootloader can be used either as a library or be flashed directly with the default configuration derived from linker scripts.

By design, the bootloader does not provide any network capabilities. Networking capabilities for fetching new firmware can be provided by the user application, using the bootloader as a library for updating the firmware, or by using the bootloader as a library and adding this capability yourself.

## Overview

The bootloader divides the storage into 4 main partitions, configurable when creating the bootloader instance or via linker scripts:

* BOOTLOADER - Where the bootloader is placed. The bootloader itself consumes about 8kB of flash, but if you need to debug it and have space available, increasing this to 24kB will allow you to run the bootloader with probe-rs.
* ACTIVE - Where the main application is placed. The bootloader will attempt to load the application at the start of this partition. The minimum size required for this partition is the size of your application.
* DFU - Where the application-to-be-swapped is placed. This partition is written to by the application. This partition must be at least 1 page bigger than the ACTIVE partition.
* BOOTLOADER STATE - Where the bootloader stores the current state describing if the active and dfu partitions need to be swapped.
* SAFE (optional) - When the `safe` Cargo feature is enabled, this partition holds a known-good firmware image that can be executed if the application signals a safe boot.

For any partition, the following preconditions are required:

* Partitions must be aligned on the page size.
* Partitions must be a multiple of the page size.

The linker scripts for the application and bootloader look similar, but the FLASH region must point to the BOOTLOADER partition for the bootloader, and the ACTIVE partition for the application.

If the `safe` feature is enabled, a safe boot can be triggered by setting the
`__bootloader_safe_flag` symbol (placed in `.bss` or `.noinit`) to a non-zero
value before resetting the device. Writing `SAFE_MAGIC` to the state partition
has the same effect. The application is responsible for clearing the flag after
the safe boot has been handled.

For more details on the bootloader, see [the documentation](https://embassy.dev/book/#_bootloader).

## Hardware support

The bootloader supports different hardware in separate crates:

* `embassy-boot-nrf` - for the nRF microcontrollers.
* `embassy-boot-rp` - for the RP2040 microcontrollers.
* `embassy-boot-stm32` - for the STM32 microcontrollers.

## Reset check

The bootloader keeps track of how many times it has restarted without the
application confirming a successful boot. The counter is stored as an 8‑bit
value in the bootloader state partition and is incremented on every boot. The
value saturates at 255. Applications can inspect and reset this counter through
the `FirmwareState` API. This functionality is behind the optional
`reset-check` feature.


## Backup and Restore
The optional `restore` feature enables power-loss safe backup and restoration
of the active partition.

When enabled, the application can request a backup of the currently running
firmware by calling `mark_backup()` and resetting. On the next boot the
bootloader copies the active partition into the DFU partition while tracking
progress to tolerate power loss before resuming the application.

Similarly, calling `mark_restore()` instructs the bootloader to restore the
active partition from the DFU partition using the same progress tracking.

