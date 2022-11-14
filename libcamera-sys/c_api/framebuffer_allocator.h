#ifndef __LIBCAMERA_C_FRAMEBUFFER_ALLOCATOR__
#define __LIBCAMERA_C_FRAMEBUFFER_ALLOCATOR__

#include "camera.h"
#include "framebuffer.h"

#include <stdint.h>

#ifdef __cplusplus
#include <libcamera/framebuffer_allocator.h>

typedef libcamera::FrameBufferAllocator libcamera_framebuffer_allocator_t;
typedef std::vector<std::unique_ptr<libcamera::FrameBuffer>> libcamera_framebuffer_list_t;

extern "C" {
#else
typedef struct libcamera_framebuffer_allocator libcamera_framebuffer_allocator_t;
typedef struct libcamera_framebuffer_list libcamera_framebuffer_list_t;
#endif

libcamera_framebuffer_allocator_t *libcamera_framebuffer_allocator_create(libcamera_camera_t *cam);
void libcamera_framebuffer_allocator_destroy(libcamera_framebuffer_allocator_t *alloc);
int libcamera_framebuffer_allocator_allocate(libcamera_framebuffer_allocator_t *alloc, libcamera_stream_t *stream);
int libcamera_framebuffer_allocator_free(libcamera_framebuffer_allocator_t *alloc, libcamera_stream_t *stream);
const libcamera_framebuffer_list_t *libcamera_framebuffer_allocator_buffers(libcamera_framebuffer_allocator_t *alloc, libcamera_stream_t *stream);

size_t libcamera_framebuffer_list_size(const libcamera_framebuffer_list_t *list);
const libcamera_framebuffer_t *libcamera_framebuffer_list_get(const libcamera_framebuffer_list_t *list, size_t index);

#ifdef __cplusplus
}
#endif

#endif
