MEMORY
{
  FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 64K
  RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 20K
}
_stack_start = ORIGIN(RAM) + LENGTH(RAM);

/* Bottom of the free stack region.  cortex-m-rt places .data and .bss at
   the start of RAM; __ebss is the first free word above them.  There is no
   RTT ring buffer or other .uninit data to skip over (this example uses
   semihosting for output, not defmt+RTT).                                  */
_stack_bottom = __ebss;
