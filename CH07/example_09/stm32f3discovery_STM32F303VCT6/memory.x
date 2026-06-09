MEMORY
{
  FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 256K
  RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 40K
}
_stack_start = ORIGIN(RAM) + LENGTH(RAM);

/* Lowest address of the stack region.  Used as the MPU guard base address.
   Must be aligned to the guard region size (32 bytes).
   0x2000_0000 is 32-byte aligned.                                          */
_stack_bottom = ORIGIN(RAM);
