/* src/main.c */
#include <stdint.h>
#include <zephyr/kernel.h>
#include <zephyr/drivers/gpio.h>
#include "led_logic.h"

static const struct gpio_dt_spec led =
    GPIO_DT_SPEC_GET(DT_ALIAS(led0), gpios);

int main(void)
{
    if (!gpio_is_ready_dt(&led)) {
        return 0;
    }

    int err = gpio_pin_configure_dt(&led, GPIO_OUTPUT_INACTIVE);
    if (err < 0) {
        return 0;
    }

    uint32_t step = 0;

    for (;;) {
        uint32_t on = led_pattern(step);   /* C or Rust, chosen at build time */

        err = gpio_pin_set_dt(&led, on != 0);
        if (err < 0) {
            return 0;
        }

        step = (step + 1U) % 10U;
        k_msleep(200);
    }
}
