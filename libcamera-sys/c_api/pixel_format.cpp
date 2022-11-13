#include "pixel_format.h"

#include <libcamera/libcamera.h>
#include <cstring>

extern "C" {

uint32_t libcamera_pixel_format_fourcc(const libcamera_pixel_format_t *format) {
    return format->fourcc();
}

uint64_t libcamera_pixel_format_modifier(const libcamera_pixel_format_t *format) {
    return format->modifier();
}

void libcamera_pixel_format_str(const libcamera_pixel_format_t *format, char* buf, size_t max_len) {
    strncpy(buf, format->toString().c_str(), max_len);
}

void libcamera_pixel_formats_destroy(libcamera_pixel_formats_t *formats) {
    delete formats;
}

size_t libcamera_pixel_formats_size(const libcamera_pixel_formats_t *formats) {
    return formats->size();
}

const libcamera_pixel_format_t *libcamera_pixel_formats_get(const libcamera_pixel_formats_t *formats, size_t index) {
    if (formats->size() <= index) {
        return nullptr;
    } else {
        return &(*formats)[index];
    }
}

}
