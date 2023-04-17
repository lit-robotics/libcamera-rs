#ifndef __LIBCAMERA_C_STREAM__
#define __LIBCAMERA_C_STREAM__

#include "geometry.h"
#include "pixel_format.h"

#include <stddef.h>

struct libcamera_stream_configuration {
    libcamera_pixel_format_t pixel_format;
    libcamera_size_t size;
    unsigned int stride;
    unsigned int frame_size;
    unsigned int buffer_count;
};

#ifdef __cplusplus
#include <libcamera/camera.h>

typedef libcamera::StreamFormats libcamera_stream_formats_t;

typedef libcamera::StreamConfiguration libcamera_stream_configuration_t;

// Read more about this in https://github.com/google/benchmark/issues/552
#ifdef __GNUC__
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Winvalid-offsetof"
#endif
static_assert(offsetof(struct libcamera_stream_configuration, pixel_format) == offsetof(libcamera_stream_configuration_t, pixelFormat));
static_assert(offsetof(struct libcamera_stream_configuration, size) == offsetof(libcamera_stream_configuration_t, size));
static_assert(offsetof(struct libcamera_stream_configuration, stride) == offsetof(libcamera_stream_configuration_t, stride));
static_assert(offsetof(struct libcamera_stream_configuration, frame_size) == offsetof(libcamera_stream_configuration_t, frameSize));
static_assert(offsetof(struct libcamera_stream_configuration, buffer_count) == offsetof(libcamera_stream_configuration_t, bufferCount));
#ifdef __GNUC__
#pragma GCC diagnostic pop
#endif

typedef libcamera::Stream libcamera_stream_t;

extern "C" {
#else
typedef struct libcamera_stream_formats libcamera_stream_formats_t;
typedef struct libcamera_stream_configuration libcamera_stream_configuration_t;
typedef struct libcamera_stream libcamera_stream_t;
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

const libcamera_stream_formats_t *libcamera_stream_configuration_formats(const libcamera_stream_configuration_t *config);
libcamera_stream_t *libcamera_stream_configuration_stream(const libcamera_stream_configuration_t *config);

#ifdef __cplusplus
}
#endif

#endif
