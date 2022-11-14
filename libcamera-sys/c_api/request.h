#ifndef __LIBCAMERA_C_REQUEST__
#define __LIBCAMERA_C_REQUEST__

#include "controls.h"
#include "framebuffer.h"
#include "stream.h"

#include <stdint.h>

enum libcamera_request_status {
    LIBCAMERA_REQUEST_STATUS_PENDING,
    LIBCAMERA_REQUEST_STATUS_COMPLETE,
    LIBCAMERA_REQUEST_STATUS_CANCELLED,
};

#ifdef __cplusplus
#include <libcamera/camera.h>

typedef libcamera::Request::Status libcamera_request_status_t;
typedef libcamera::Request libcamera_request_t;

extern "C" {
#else
typedef enum libcamera_request_status libcamera_request_status_t;
typedef struct libcamera_request libcamera_request_t;
#endif

void libcamera_request_destroy(libcamera_request_t *request);
libcamera_control_list_t *libcamera_request_controls(libcamera_request_t *request);
libcamera_control_list_t *libcamera_request_metadata(libcamera_request_t *request);
int libcamera_request_add_buffer(libcamera_request_t *request, const libcamera_stream_t *stream, libcamera_framebuffer_t *buffer);
libcamera_request_status_t libcamera_request_status(const libcamera_request_t *request);

#ifdef __cplusplus
}
#endif

#endif
