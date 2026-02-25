#include "request.h"
#include "fence.h"
#include <libcamera/libcamera.h>
#include <cstring>

extern "C" {

void libcamera_request_destroy(libcamera_request_t *request) {
    delete request;
}

libcamera_control_list_t *libcamera_request_controls(libcamera_request_t *request) {
    return &request->controls();
}

const libcamera_control_list_t *libcamera_request_metadata(libcamera_request_t *request) {
    return &request->metadata();
}

const libcamera_request_buffer_map_t *libcamera_request_buffers(const libcamera_request_t *request) {
    return &request->buffers();
}

int libcamera_request_add_buffer(libcamera_request_t *request, const libcamera_stream_t *stream, libcamera_framebuffer_t *buffer) {
    return request->addBuffer(stream, buffer);
}

int libcamera_request_add_buffer_with_fence(libcamera_request_t *request, const libcamera_stream_t *stream, libcamera_framebuffer_t *buffer, libcamera_fence_t *fence) {
    std::unique_ptr<libcamera::Fence> acquire_fence;
    if (fence) {
        acquire_fence.reset(fence);
    }
    return request->addBuffer(stream, buffer, std::move(acquire_fence));
}

libcamera_framebuffer_t *libcamera_request_find_buffer(const libcamera_request_t *request, const libcamera_stream_t *stream) {
    return request->findBuffer(stream);
}

uint32_t libcamera_request_sequence(const libcamera_request_t *request) {
    return request->sequence();
}

uint64_t libcamera_request_cookie(const libcamera_request_t *request) {
    return request->cookie();
}

libcamera_request_status_t libcamera_request_status(const libcamera_request_t *request) {
    return request->status();
}

void libcamera_request_reuse(libcamera_request_t *request, libcamera_request_reuse_flag_t flags) {
    return request->reuse(flags);
}

bool libcamera_request_has_pending_buffers(const libcamera_request_t *request) {
    return request->hasPendingBuffers();
}

char *libcamera_request_to_string(const libcamera_request_t *request) {
    return ::strdup(request->toString().c_str());
}

libcamera_framebuffer_t *libcamera_request_buffer_map_get(libcamera_request_buffer_map_t* buffer_map, const libcamera_stream_t *stream) {
	const auto it = buffer_map->find(stream);
	if (it == buffer_map->end())
		return nullptr;

	return it->second;
}

libcamera_request_buffer_map_iter_t *libcamera_request_buffer_map_iter(libcamera_request_buffer_map_t* buffer_map) {
    auto it = buffer_map->begin();
    return new libcamera_request_buffer_map_iter_t { buffer_map, it };
}

void libcamera_request_buffer_map_iter_destroy(libcamera_request_buffer_map_iter_t *iter) {
    delete iter;
}

bool libcamera_request_buffer_map_iter_end(const libcamera_request_buffer_map_iter_t *iter) {
    return iter->it == iter->buffer_map->end();
}

void libcamera_request_buffer_map_iter_next(libcamera_request_buffer_map_iter_t *iter) {
    if (iter->it != iter->buffer_map->end()) {
        ++(iter->it);
    }
}

const libcamera_stream_t *libcamera_request_buffer_map_iter_stream(libcamera_request_buffer_map_iter_t *iter) {
    return iter->it->first;
}

libcamera_framebuffer_t *libcamera_request_buffer_map_iter_buffer(libcamera_request_buffer_map_iter_t *iter) {
    return iter->it->second;
}

}
