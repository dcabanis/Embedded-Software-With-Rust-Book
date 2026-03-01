MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 256K
  CCMRAM (rwx) : ORIGIN = 0x10000000, LENGTH = 8K
  RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 40K
}




/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* NOTE Do NOT modify `_stack_start` unless you know what you are doing */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);
