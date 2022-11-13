#ifndef __LIBCAMERA_C_PIXEL_FORMAT__
#define __LIBCAMERA_C_PIXEL_FORMAT__

#include <stdint.h>

#ifdef __cplusplus
#include <libcamera/camera.h>

typedef libcamera::PixelFormat libcamera_pixel_format_t;
typedef std::vector<libcamera::PixelFormat> libcamera_pixel_formats_t;

extern "C" {
#else
typedef struct libcamera_pixel_format libcamera_pixel_format_t;
typedef struct libcamera_pixel_formats libcamera_pixel_formats_t;
#endif

uint32_t libcamera_pixel_format_fourcc(const libcamera_pixel_format_t *format);
uint64_t libcamera_pixel_format_modifier(const libcamera_pixel_format_t *format);
// Works like strncpy
void libcamera_pixel_format_str(const libcamera_pixel_format_t *format, char* buf, size_t max_len);

void libcamera_pixel_formats_destroy(libcamera_pixel_formats_t *formats);
size_t libcamera_pixel_formats_size(const libcamera_pixel_formats_t *formats);
const libcamera_pixel_format_t *libcamera_pixel_formats_get(const libcamera_pixel_formats_t *formats, size_t index);

#ifdef __cplusplus
}
#endif

#endif
