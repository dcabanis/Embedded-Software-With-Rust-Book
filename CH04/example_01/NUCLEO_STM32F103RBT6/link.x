MEMORY
{
    FLASH : ORIGIN = 0x08000000, LENGTH = 128K
    RAM   : ORIGIN = 0x20000000, LENGTH = 20K
}

SECTIONS
{
  _stack_top = (ORIGIN(RAM) + LENGTH(RAM)) & ~7;

  .vector_table :
  {
    LONG(_stack_top);
    KEEP(*(.vector_table.reset_vector))
    KEEP(*(.vector_table.exceptions))
  } > FLASH

  .text :
  {
    *(.text .text.*)
  } > FLASH

  .rodata :
  {
    *(.rodata .rodata.*)
  } > FLASH

  .data : ALIGN(8)
  {
    _glob_RW = .;
    *(.data .data.*)
    . = ALIGN(8);
    _eglob_RW = .;
  } > RAM AT > FLASH
  _sidata = LOADADDR(.data);

  .bss (NOLOAD) : ALIGN(8)
  {
    _glob_ZI = .;
    *(.bss .bss.*)
    *(COMMON)
    . = ALIGN(8);
    _eglob_ZI = .;
  } > RAM
}
