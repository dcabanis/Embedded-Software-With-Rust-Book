#include <stdint.h>
#include "my_rust_lib.h"

#define RCC_APB2ENR (*(volatile uint32_t *)0x40021018u)
#define GPIOA_CRL   (*(volatile uint32_t *)0x40010800u)
#define GPIOA_ODR   (*(volatile uint32_t *)0x4001080Cu)
#define SYST_CSR    (*(volatile uint32_t *)0xE000E010u)
#define SYST_RVR    (*(volatile uint32_t *)0xE000E014u)
#define SYST_CVR    (*(volatile uint32_t *)0xE000E018u)

/* NUCLEO-F103RB user LED (LD2) is on PA5, and is active-high --
 * the opposite polarity from the Blue Pill's PC13. */
static void gpio_init_pa5(void) {
    RCC_APB2ENR |= (1u << 2);                       /* enable GPIOA clock (IOPAEN) */
    GPIOA_CRL = (GPIOA_CRL & ~(0xFu << 20)) | (0x2u << 20); /* PA5 push-pull, 2 MHz */
    GPIOA_ODR &= ~(1u << 5);                        /* active-high LED: start OFF */
}

static void led_toggle(void) {
    GPIOA_ODR ^= (1u << 5);
}

static void led_on(void) {
    GPIOA_ODR |= (1u << 5); /* active-high */
}

static void systick_init(void) {
    SYST_CSR = 0;
    SYST_CVR = 0;
    SYST_RVR = 8000000u / 4u - 1u;  /* ~250 ms tick from the 8 MHz HSI */
    SYST_CSR = 0x5u;                /* ENABLE, core clock, no interrupt */
}

static void wait_tick(void) {
    while ((SYST_CSR & (1u << 16)) == 0) {
    }
}

int main(void) {
    gpio_init_pa5();
    systick_init();

    int32_t result = 0;

    /* Exercise both the success and the error-code path from 10.3.4. */
    int checks_ok = 1;
    checks_ok = checks_ok && (square(9) == 81);
    checks_ok = checks_ok && (divide(84, 12, &result) == 0) && (result == 7);
    checks_ok = checks_ok && (divide(1, 0, &result) == -1);
    checks_ok = checks_ok && (divide(1, 1, (int32_t *)0) == -2);
    /* INT32_MIN / -1 overflows a two's-complement i32. checked_div()
     * catches this and returns -1 instead of panicking; with the old
     * `a / b` implementation this line would have hung the firmware. */
    checks_ok = checks_ok && (divide(INT32_MIN, -1, &result) == -1);

    if (checks_ok) {
        /* Rust calls all checked out: blink steadily forever. */
        while (1) {
            wait_tick();
            led_toggle();
        }
    }

    /* A mismatch would mean the two sides disagree on the ABI: latch
     * the LED on solid instead of blinking so the failure is visible. */
    led_on();
    while (1) {
    }
}
