#ifndef __LIBCAMERA_C_STREAM__
#define __LIBCAMERA_C_STREAM__

#include "geometry.h"
#include "pixel_format.h"

#include <stddef.h>

#ifdef __cplusplus
#include <libcamera/camera.h>

typedef libcamera::StreamFormats libcamera_stream_formats_t;
typedef libcamera::StreamConfiguration libcamera_stream_configuration_t;

extern "C" {
#else
typedef struct libcamera_stream_formats libcamera_stream_formats_t;
typedef struct libcamera_stream_configuration libcamera_stream_configuration_t;
#endif

enum libcamera_stream_role {
    LIBCAMERA_STREAM_ROLE_RAW = 0,
    LIBCAMERA_STREAM_ROLE_STILL_CAPTURE = 1,
    LIBCAMERA_STREAM_ROLE_VIDEO_RECORDING = 2,
    LIBCAMERA_STREAM_ROLE_VIEW_FINDER = 3,
};

libcamera_pixel_formats_t *libcamera_stream_formats_pixel_formats(const libcamera_stream_formats_t* formats);
libcamera_sizes_t *libcamera_stream_formats_sizes(const libcamera_stream_formats_t* formats, const libcamera_pixel_format_t *pixel_format);
libcamera_size_range_t libcamera_stream_formats_range(const libcamera_stream_formats_t* formats, const libcamera_pixel_format_t *pixel_format);

void libcamera_stream_configuration_destroy(libcamera_stream_configuration_t *config);
libcamera_pixel_format_t *libcamera_stream_configuration_pixel_format(libcamera_stream_configuration_t *config);
libcamera_size_t *libcamera_stream_configuration_size(libcamera_stream_configuration_t *config);
unsigned int *libcamera_stream_configuration_stride(libcamera_stream_configuration_t *config);
unsigned int *libcamera_stream_configuration_frame_size(libcamera_stream_configuration_t *config);
unsigned int *libcamera_stream_configuration_buffer_count(libcamera_stream_configuration_t *config);
unsigned int *libcamera_stream_configuration_color_space(libcamera_stream_configuration_t *config);
const libcamera_stream_formats_t *libcamera_stream_configuration_formats(const libcamera_stream_configuration_t *config);

#ifdef __cplusplus
}
#endif

#endif
