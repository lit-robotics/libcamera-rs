#include "geometry.h"

#include <libcamera/libcamera.h>

extern "C" {

void libcamera_sizes_destroy(libcamera_sizes_t *sizes) {
    delete sizes;
}

size_t libcamera_sizes_size(const libcamera_sizes_t *sizes) {
    return sizes->size();
}

const libcamera_size_t *libcamera_sizes_at(const libcamera_sizes_t *sizes, size_t index) {
    return &sizes->at(index);
}

}
