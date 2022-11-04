#pragma once

#ifdef __cplusplus
#include <libcamera/controls.h>

typedef libcamera::ControlList libcamera_control_list_t;

extern "C" {
#else
typedef struct libcamera_control_list_t libcamera_control_list_t;
#endif

#ifdef __cplusplus
}
#endif
