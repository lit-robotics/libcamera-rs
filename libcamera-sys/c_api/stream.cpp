#include "stream.h"

#include <libcamera/libcamera.h>

extern "C" {

libcamera_pixel_formats_t *libcamera_stream_formats_pixel_formats(const libcamera_stream_formats_t* formats) {
    return new libcamera_pixel_formats_t(std::move(formats->pixelformats()));
}

libcamera_sizes_t *libcamera_stream_formats_sizes(const libcamera_stream_formats_t* formats, const libcamera_pixel_format_t *pixel_format) {
    return new libcamera_sizes_t(std::move(formats->sizes(*pixel_format)));
}

libcamera_size_range_t libcamera_stream_formats_range(const libcamera_stream_formats_t* formats, const libcamera_pixel_format_t *pixel_format) {
    return formats->range(*pixel_format);
}

const libcamera_stream_formats_t *libcamera_stream_configuration_formats(const libcamera_stream_configuration_t *config) {
    return &config->formats();
}

libcamera_stream_t *libcamera_stream_configuration_stream(const libcamera_stream_configuration_t *config) {
    return config->stream();
}

}
