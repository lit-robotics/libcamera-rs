#ifndef __LIBCAMERA_C_STREAM__
#define __LIBCAMERA_C_STREAM__

#include <stddef.h>

#ifdef __cplusplus
#include <libcamera/camera.h>

typedef libcamera::StreamConfiguration libcamera_stream_configuration_t;

extern "C" {
#else
typedef struct libcamera_stream_configuration_t libcamera_stream_configuration_t;
#endif

enum libcamera_stream_role {
    LIBCAMERA_STREAM_ROLE_RAW = 0,
    LIBCAMERA_STREAM_ROLE_STILL_CAPTURE = 1,
    LIBCAMERA_STREAM_ROLE_VIDEO_RECORDING = 2,
    LIBCAMERA_STREAM_ROLE_VIEW_FINDER = 3,
};

void libcamera_stream_configuration_destroy(libcamera_stream_configuration_t *config);

#ifdef __cplusplus
}
#endif

#endif
