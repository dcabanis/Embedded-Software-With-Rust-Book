#include <stdint.h>
#include "embedded_rust_lib.h"

#define RCC_APB2ENR (*(volatile uint32_t *)0x40021018u)
#define GPIOC_CRH   (*(volatile uint32_t *)0x40011004u)
#define GPIOC_ODR   (*(volatile uint32_t *)0x4001100Cu)
#define SYST_CSR    (*(volatile uint32_t *)0xE000E010u)
#define SYST_RVR    (*(volatile uint32_t *)0xE000E014u)
#define SYST_CVR    (*(volatile uint32_t *)0xE000E018u)

static void gpio_init_pc13(void) {
    RCC_APB2ENR |= (1u << 4);                       /* enable GPIOC clock */
    GPIOC_CRH = (GPIOC_CRH & ~(0xFu << 20)) | (0x2u << 20); /* PC13 push-pull, 2 MHz */
    GPIOC_ODR |= (1u << 13);                        /* active-low LED: start OFF */
}

static void led_toggle(void) {
    GPIOC_ODR ^= (1u << 13);
}

static void led_on(void) {
    GPIOC_ODR &= ~(1u << 13); /* active-low */
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
    gpio_init_pc13();
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
