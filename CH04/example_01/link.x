MEMORY
{
    FLASH : ORIGIN = 0x08000000, LENGTH = 64K
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
    _sdata = .;
    *(.data .data.*)
    . = ALIGN(8);
    _edata = .;
  } > RAM AT > FLASH
  _sidata = LOADADDR(.data);

  .bss (NOLOAD) : ALIGN(8)
  {
    _sbss = .;
    *(.bss .bss.*)
    *(COMMON)
    . = ALIGN(8);
    _ebss = .;
  } > RAM
}
