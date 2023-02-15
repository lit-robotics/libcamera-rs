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

enum libcamera_request_reuse_flag {
    LIBCAMERA_REQUEST_REUSE_FLAG_DEFAULT = 0,
    LIBCAMERA_REQUEST_REUSE_FLAG_REUSE_BUFFERS = 1 << 0,

};

#ifdef __cplusplus
#include <libcamera/camera.h>

struct libcamera_request_buffer_map_iter {
	libcamera::Request::BufferMap *buffer_map;
	libcamera::Request::BufferMap::iterator it;
};

typedef libcamera::Request::Status libcamera_request_status_t;
typedef libcamera::Request::ReuseFlag libcamera_request_reuse_flag_t;
typedef libcamera::Request libcamera_request_t;
typedef libcamera::Request::BufferMap libcamera_request_buffer_map_t;
typedef struct libcamera_request_buffer_map_iter libcamera_request_buffer_map_iter_t;

extern "C" {
#else
typedef enum libcamera_request_status libcamera_request_status_t;
typedef enum libcamera_request_reuse_flag libcamera_request_reuse_flag_t;
typedef struct libcamera_request libcamera_request_t;
typedef struct libcamera_request_buffer_map libcamera_request_buffer_map_t;
typedef struct libcamera_request_buffer_map_iter libcamera_request_buffer_map_iter_t;
#endif

// --- libcamera_request_t ---
void libcamera_request_destroy(libcamera_request_t *request);
libcamera_control_list_t *libcamera_request_controls(libcamera_request_t *request);
libcamera_control_list_t *libcamera_request_metadata(libcamera_request_t *request);
const libcamera_request_buffer_map_t *libcamera_request_buffers(const libcamera_request_t *request);
int libcamera_request_add_buffer(libcamera_request_t *request, const libcamera_stream_t *stream, libcamera_framebuffer_t *buffer);
libcamera_framebuffer_t *libcamera_request_find_buffer(const libcamera_request_t *request, const libcamera_stream_t *stream);
uint32_t libcamera_request_sequence(const libcamera_request_t *request);
uint64_t libcamera_request_cookie(const libcamera_request_t *request);
libcamera_request_status_t libcamera_request_status(const libcamera_request_t *request);
void libcamera_request_reuse(libcamera_request_t *request, libcamera_request_reuse_flag_t flags);

// --- libcamera_request_buffer_map_t ---
libcamera_framebuffer_t *libcamera_request_buffer_map_get(libcamera_request_buffer_map_t* buffer_map, const libcamera_stream_t *stream);
libcamera_request_buffer_map_iter_t *libcamera_request_buffer_map_iter(libcamera_request_buffer_map_t* buffer_map);

// --- libcamera_request_buffer_map_iter_t ---
void libcamera_request_buffer_map_iter_destroy(libcamera_request_buffer_map_iter_t *iter);
bool libcamera_request_buffer_map_iter_end(const libcamera_request_buffer_map_iter_t *iter);
void libcamera_request_buffer_map_iter_next(libcamera_request_buffer_map_iter_t *iter);
const libcamera_stream_t *libcamera_request_buffer_map_iter_stream(libcamera_request_buffer_map_iter_t *iter);
libcamera_framebuffer_t *libcamera_request_buffer_map_iter_buffer(libcamera_request_buffer_map_iter_t *iter);

#ifdef __cplusplus
}
#endif

#endif
