#include "request.h"

extern "C" {

void libcamera_request_destroy(libcamera_request_t *request) {
    delete request;
}

libcamera_control_list_t *libcamera_request_controls(libcamera_request_t *request) {
    return &request->controls();
}

libcamera_control_list_t *libcamera_request_metadata(libcamera_request_t *request) {
    return &request->metadata();
}

int libcamera_request_add_buffer(libcamera_request_t *request, const libcamera_stream_t *stream, libcamera_framebuffer_t *buffer) {
    return request->addBuffer(stream, buffer);
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

}
