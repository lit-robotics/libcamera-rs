#include "framebuffer_allocator.h"

#include <libcamera/libcamera.h>

extern "C" {

libcamera_framebuffer_allocator_t *libcamera_framebuffer_allocator_create(libcamera_camera_t *cam) {
    return new libcamera::FrameBufferAllocator(*cam);
}

void libcamera_framebuffer_allocator_destroy(libcamera_framebuffer_allocator_t *alloc) {
    delete alloc;
}

int libcamera_framebuffer_allocator_allocate(libcamera_framebuffer_allocator_t *alloc, libcamera_stream_t *stream) {
    return alloc->allocate(stream);
}

int libcamera_framebuffer_allocator_free(libcamera_framebuffer_allocator_t *alloc, libcamera_stream_t *stream) {
    return alloc->free(stream);
}

const libcamera_framebuffer_list_t *libcamera_framebuffer_allocator_buffers(libcamera_framebuffer_allocator_t *alloc, libcamera_stream_t *stream) {
    return &alloc->buffers(stream);
}

size_t libcamera_framebuffer_list_size(const libcamera_framebuffer_list_t *list) {
    return list->size();
}

const libcamera_framebuffer_t *libcamera_framebuffer_list_get(const libcamera_framebuffer_list_t *list, size_t index) {
    return list->at(index).get();
}

}
