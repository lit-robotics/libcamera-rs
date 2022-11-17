#ifndef __LIBCAMERA_C_PIXEL_FORMAT__
#define __LIBCAMERA_C_PIXEL_FORMAT__

#include <stddef.h>
#include <stdint.h>

struct libcamera_pixel_format {
    uint32_t fourcc;
    uint64_t modifier;
};

#ifdef __cplusplus
#include <libcamera/camera.h>

typedef libcamera::PixelFormat libcamera_pixel_format_t;
static_assert(sizeof(struct libcamera_pixel_format) == sizeof(libcamera_pixel_format_t));
// offsetof does not work on private fields :(
// static_assert(offsetof(struct libcamera_pixel_format, fourcc) == offsetof(libcamera_pixel_format_t, fourcc_));
// static_assert(offsetof(struct libcamera_pixel_format, modifier) == offsetof(libcamera_pixel_format_t, modifier_));

typedef std::vector<libcamera::PixelFormat> libcamera_pixel_formats_t;

extern "C" {
#else
typedef struct libcamera_pixel_format libcamera_pixel_format_t;
typedef struct libcamera_pixel_formats libcamera_pixel_formats_t;
#endif

// Works like strncpy
void libcamera_pixel_format_str(const libcamera_pixel_format_t *format, char* buf, size_t max_len);

void libcamera_pixel_formats_destroy(libcamera_pixel_formats_t *formats);
size_t libcamera_pixel_formats_size(const libcamera_pixel_formats_t *formats);
libcamera_pixel_format_t libcamera_pixel_formats_get(const libcamera_pixel_formats_t *formats, size_t index);

#ifdef __cplusplus
}
#endif

#endif
