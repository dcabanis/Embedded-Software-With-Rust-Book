#include <stdint.h>
#include "embedded_rust_lib.h"

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

    static const uint8_t payload[] = {0x10, 0x20, 0x30, 0x40}; /* sums to 0xA0 */

    int checks_ok = 1;
    checks_ok = checks_ok && (checksum(payload, sizeof(payload)) == 0xA0);
    checks_ok = checks_ok && (checksum(payload, 0) == 0);       /* empty buffer */
    checks_ok = checks_ok && (checksum((const uint8_t *)0, 4) == 0); /* null pointer */

    if (checks_ok) {
        /* The generated header matched what the Rust library actually
         * exports: blink steadily forever. */
        while (1) {
            wait_tick();
            led_toggle();
        }
    }

    /* A mismatch here would mean the header cbindgen generated no longer
     * matches the compiled library -- latch the LED on solid instead. */
    led_on();
    while (1) {
    }
}
