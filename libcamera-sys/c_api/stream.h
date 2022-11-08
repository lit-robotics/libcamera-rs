#ifndef __LIBCAMERA_C_CAMERA__
#define __LIBCAMERA_C_CAMERA__

#include "controls.h"

#include <stddef.h>

#ifdef __cplusplus
#include <libcamera/camera.h>

typedef std::unique_ptr<libcamera::StreamConfiguration> libcamera_stream_configuration_t;

extern "C" {
#else
typedef struct libcamera_stream_configuration_t libcamera_stream_configuration_t;
#endif

#ifdef __cplusplus
}
#endif

#endif
