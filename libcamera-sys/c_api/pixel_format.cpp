#include "pixel_format.h"

#include <libcamera/libcamera.h>
#include <cstring>

extern "C" {

char *libcamera_pixel_format_str(const libcamera_pixel_format_t *format) {
    return strdup(format->toString().c_str());
}

void libcamera_pixel_formats_destroy(libcamera_pixel_formats_t *formats) {
    delete formats;
}

size_t libcamera_pixel_formats_size(const libcamera_pixel_formats_t *formats) {
    return formats->size();
}

libcamera_pixel_format_t libcamera_pixel_formats_get(const libcamera_pixel_formats_t *formats, size_t index) {
    return (*formats)[index];
}

}
