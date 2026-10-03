#include "device_status.h"

int32_t add(int32_t a, int32_t b) {
    return a + b;
}

void update_status(DeviceStatus *status, float delta_temp) {
    status->temperature += delta_temp;

    if (status->state == DEVICE_STATE_IDLE) {
        status->state = DEVICE_STATE_ACTIVE;
    }

    if (status->temperature > 60.0f) {
        status->state = DEVICE_STATE_ERROR;
    }
}
