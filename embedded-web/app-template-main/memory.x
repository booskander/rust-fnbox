/* Minimal memory layout for lm3s6965evb QEMU board */
MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 128K
  RAM   : ORIGIN = 0x20000000, LENGTH = 16K
}

_estack = ORIGIN(RAM) + LENGTH(RAM);