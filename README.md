# rust-thumbv7em-align-bug

This repository contains a minimal example for reproducing a Rust thumbv7em-none-eabi memory
alignment bug.

The example code is tested on a Nucleo-F401RE evaluation kit with a STM32F401RE MCU
and a XMC4800 Relax Kit with a XMC4800 MCU.
For this minimal example, the targets are switched by (un)commenting the respective blocks in
`device.x` and `memory.x`.

For both targets the same assembly is generated. However, not all ARMv7em MCUs allow unaligned
memory access. Unaligned memory support is indicated by the `CCR` register in the system control
block (SCB). In my test, the register contains the following values:

* XMC4800: 0x218, i.e. UNALIGN\_TRP | DIV0\_TRP | STKALIGN
* SRM32F401RE: 0x200, i.e. STKALIGN

The rustc flag `+strict-align` works around this bug but generates a
`unknown and unstable feature specified` warning.

For reference, the generated assembly for all four variants (targets and strict-align) can be
found in the `.lss` files.
