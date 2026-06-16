
/* STM32f103C8T6 Memory Map */
MEMORY
{
    FLASH (rx): ORIGIN = 0x08000000, LENGTH = 64K
    SRAM (rwx): ORIGIN = 0x20000000, LENGTH = 20K
}

/* Entry symbol for tools that inspect ELF e_entry (CPU still boots from vector table). */
ENTRY(reset);

SECTIONS
{
  .vector_table ORIGIN(FLASH) :
  {
    /* First entry: initial Stack Pointer value */
    LONG(ORIGIN(SRAM) + LENGTH(SRAM));

    /* Second entry: internal exceptions' table */
    KEEP(*(.vector_table.exceptions));

    /* [TODO] Add the INTERRUPT table */
    KEEP(*(.vector_table.interrupts));
  } > FLASH

  .text :
  {
    . = ALIGN(4);
    *(.text .text.*);
    . = ALIGN(4);
  } > FLASH

  .rodata :
  {
    . = ALIGN(4);
    *(.rodata .rodata.*);
    . = ALIGN(4);
  } > FLASH

  .data :
  {
    _glob_RW = .;
    *(.data .data.*);
    . = ALIGN(4);
    _eglob_RW = .;
  } > SRAM AT>FLASH

  /* Provide the symbol for the data load address in Flash */
  PROVIDE(_sidata = LOADADDR(.data));

 .bss (NOLOAD):
  {
    _glob_ZI = .;
    *(.bss .bss.*);
    . = ALIGN(4);
    _eglob_ZI = .;
  } > SRAM
}
