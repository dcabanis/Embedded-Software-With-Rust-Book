/* Minimal Cortex-M3 startup: vector table + Reset_Handler.
 * This is what cortex-m-rt generates automatically elsewhere in the book
 * (see CH04); this example is a plain C application, so it is written
 * by hand here instead. */

    .syntax unified
    .cpu cortex-m3
    .thumb

    .section .vector_table, "a", %progbits
    .global vector_table
vector_table:
    .word _stack_top          /* initial SP */
    .word Reset_Handler       /* Reset */
    .word Default_Handler     /* NMI */
    .word Default_Handler     /* HardFault */
    .word Default_Handler     /* MemManage */
    .word Default_Handler     /* BusFault */
    .word Default_Handler     /* UsageFault */
    .word 0
    .word 0
    .word 0
    .word 0
    .word Default_Handler     /* SVCall */
    .word Default_Handler     /* Debug Monitor */
    .word 0
    .word Default_Handler     /* PendSV */
    .word Default_Handler     /* SysTick */

    .section .text.Reset_Handler, "ax", %progbits
    .thumb_func
    .global Reset_Handler
Reset_Handler:
    /* Copy .data from its Flash load address to RAM. */
    ldr r0, =_sidata
    ldr r1, =_sdata
    ldr r2, =_edata
copy_data_loop:
    cmp r1, r2
    bcs copy_data_done
    ldr r3, [r0], #4
    str r3, [r1], #4
    b copy_data_loop
copy_data_done:

    /* Zero-fill .bss. */
    ldr r1, =_sbss
    ldr r2, =_ebss
    movs r3, #0
zero_bss_loop:
    cmp r1, r2
    bcs zero_bss_done
    str r3, [r1], #4
    b zero_bss_loop
zero_bss_done:

    bl main

hang:
    b hang

    .section .text.Default_Handler, "ax", %progbits
    .thumb_func
    .global Default_Handler
Default_Handler:
    b Default_Handler

    /* Marks this object as not requiring an executable stack, silencing
     * a linker warning that is meaningless for a bare-metal image. */
    .section .note.GNU-stack, "", %progbits
