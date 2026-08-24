#ifndef DEVICE_STATUS_H
#define DEVICE_STATUS_H

#include <stdint.h>

/* A plain `enum` would leave the C compiler free to shrink or grow this
 * type; a typedef over a fixed-width integer has no default left to
 * negotiate. This byte must be exactly one of the three
 * values below Rust receives it as a plain, unchecked u8 wrapper and
 * only treats it as a real enum after validating it. */
typedef uint8_t DeviceState;
#define DEVICE_STATE_IDLE   0
#define DEVICE_STATE_ACTIVE 1
#define DEVICE_STATE_ERROR  2

typedef struct {
    int32_t id;
    DeviceState state;
    float temperature;
} DeviceStatus;

_Static_assert(sizeof(DeviceStatus) == 12, "DeviceStatus size mismatch");

int32_t add(int32_t a, int32_t b);
void update_status(DeviceStatus *status, float delta_temp);

#endif /* DEVICE_STATUS_H */
