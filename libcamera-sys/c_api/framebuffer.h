#ifndef __LIBCAMERA_C_FRAMEBUFFER__
#define __LIBCAMERA_C_FRAMEBUFFER__

#include <stddef.h>
#include <stdbool.h>
#include <stdint.h>
#include "fence.h"

enum libcamera_frame_metadata_status {
    LIBCAMERA_FRAME_METADATA_STATUS_SUCCESS,
    LIBCAMERA_FRAME_METADATA_STATUS_ERROR,
    LIBCAMERA_FRAME_METADATA_STATUS_CANCELLED,
    LIBCAMERA_FRAME_METADATA_STATUS_STARTUP,
};

struct libcamera_frame_metadata_plane {
    unsigned int bytes_used;
};

struct libcamera_framebuffer_plane_info {
    int fd;
    unsigned int offset;
    unsigned int length;
};

#ifdef __cplusplus
#include <libcamera/camera.h>
#include <libcamera/version.h>

typedef libcamera::FrameMetadata::Status libcamera_frame_metadata_status_t;
typedef libcamera::FrameMetadata::Plane libcamera_frame_metadata_plane_t;
#if LIBCAMERA_VERSION_MAJOR == 0 && LIBCAMERA_VERSION_MINOR < 8
typedef libcamera::Span<libcamera::FrameMetadata::Plane> libcamera_frame_metadata_planes_t;
#else
typedef std::span<libcamera::FrameMetadata::Plane> libcamera_frame_metadata_planes_t;
#endif
typedef libcamera::FrameMetadata libcamera_frame_metadata_t;
typedef libcamera::FrameBuffer::Plane libcamera_framebuffer_plane_t;
typedef struct libcamera_framebuffer_planes libcamera_framebuffer_planes_t;
typedef libcamera::FrameBuffer libcamera_framebuffer_t;
typedef libcamera::Request libcamera_request_t;

static_assert(sizeof(struct libcamera_frame_metadata_plane) == sizeof(libcamera_frame_metadata_plane_t));
static_assert(offsetof(struct libcamera_frame_metadata_plane, bytes_used) == offsetof(libcamera_frame_metadata_plane_t, bytesused));

extern "C" {
#else
typedef enum libcamera_frame_metadata_status libcamera_frame_metadata_status_t;
typedef struct libcamera_frame_metadata_plane libcamera_frame_metadata_plane_t;
typedef struct libcamera_frame_metadata_planes libcamera_frame_metadata_planes_t;
typedef struct libcamera_frame_metadata libcamera_frame_metadata_t;
typedef struct libcamera_framebuffer_plane libcamera_framebuffer_plane_t;
typedef struct libcamera_framebuffer_planes libcamera_framebuffer_planes_t;
typedef struct libcamera_framebuffer libcamera_framebuffer_t;
typedef struct libcamera_request libcamera_request_t;
#endif

// --- libcamera_frame_metadata_t ---
libcamera_frame_metadata_status_t libcamera_frame_metadata_status(const libcamera_frame_metadata_t *metadata);
unsigned int libcamera_frame_metadata_sequence(const libcamera_frame_metadata_t *metadata);
uint64_t libcamera_frame_metadata_timestamp(const libcamera_frame_metadata_t *metadata);
libcamera_frame_metadata_planes_t *libcamera_frame_metadata_planes(libcamera_frame_metadata_t *metadata);

// --- libcamera_frame_metadata_planes_t ---
void libcamera_frame_metadata_planes_destroy(libcamera_frame_metadata_planes_t *planes);
size_t libcamera_frame_metadata_planes_size(const libcamera_frame_metadata_planes_t *planes);
libcamera_frame_metadata_plane_t *libcamera_frame_metadata_planes_at(libcamera_frame_metadata_planes_t *planes, size_t index);

// --- libcamera_framebuffer_t ---
libcamera_framebuffer_t *libcamera_framebuffer_create(const struct libcamera_framebuffer_plane_info *planes, size_t num_planes, uint64_t cookie);
void libcamera_framebuffer_destroy(libcamera_framebuffer_t *framebuffer);
libcamera_framebuffer_planes_t *libcamera_framebuffer_planes(const libcamera_framebuffer_t *framebuffer);
const libcamera_frame_metadata_t *libcamera_framebuffer_metadata(const libcamera_framebuffer_t *framebuffer);
uint64_t libcamera_framebuffer_cookie(const libcamera_framebuffer_t *framebuffer);
void libcamera_framebuffer_set_cookie(libcamera_framebuffer_t *framebuffer, uint64_t cookie);
int libcamera_framebuffer_release_fence(libcamera_framebuffer_t *framebuffer);
libcamera_fence_t *libcamera_framebuffer_release_fence_handle(libcamera_framebuffer_t *framebuffer);
libcamera_request_t *libcamera_framebuffer_request(const libcamera_framebuffer_t *framebuffer);

// --- libcamera_framebuffer_plane_t ---
int libcamera_framebuffer_plane_fd(libcamera_framebuffer_plane_t *plane);
size_t libcamera_framebuffer_plane_offset(const libcamera_framebuffer_plane_t *plane);
bool libcamera_framebuffer_plane_offset_valid(const libcamera_framebuffer_plane_t *plane);
size_t libcamera_framebuffer_plane_length(const libcamera_framebuffer_plane_t *plane);

// --- libcamera_framebuffer_planes_t ---
void libcamera_framebuffer_planes_destroy(libcamera_framebuffer_planes_t *planes);
size_t libcamera_framebuffer_planes_size(const libcamera_framebuffer_planes_t *planes);
libcamera_framebuffer_plane_t *libcamera_framebuffer_planes_at(libcamera_framebuffer_planes_t *planes, size_t index);

#ifdef __cplusplus
}
#endif

#endif
