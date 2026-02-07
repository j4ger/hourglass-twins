# Hourglass Twins

A split keyboard with 46 keys, a TPS43 trackpad and a comfortable layout. Generated using Cosmos and powered by RMK.

## Compilation

```
cargo build --release
```

## Flashing

Both sides need to be put into BOOTSEL mode manually by holding the BOOTSEL button while plugging in the USB cable.

Right side (central):

```
picotool load --update --verify --execute -t elf target/thumbv6m-none-eabi/release/central
```

Left side (peripheral):

```
picotool load --update --verify --execute -t elf target/thumbv6m-none-eabi/release/peripheral
```

## Keymap

TBD
