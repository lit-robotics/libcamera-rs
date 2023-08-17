#ifndef __LIBCAMERA_C_FRAMEBUFFER__
#define __LIBCAMERA_C_FRAMEBUFFER__

#include <stdbool.h>
#include <stdint.h>
#include <stddef.h>

enum libcamera_frame_metadata_status {
    LIBCAMERA_FRAME_METADATA_STATUS_SUCCESS,
    LIBCAMERA_FRAME_METADATA_STATUS_ERROR,
    LIBCAMERA_FRAME_METADATA_STATUS_CANCELLED,
};

struct libcamera_frame_metadata_plane {
    unsigned int bytes_used;
};

#ifdef __cplusplus
#include <libcamera/camera.h>

typedef libcamera::FrameMetadata::Status libcamera_frame_metadata_status_t;
typedef libcamera::FrameMetadata::Plane libcamera_frame_metadata_plane_t;
typedef libcamera::Span<libcamera::FrameMetadata::Plane> libcamera_frame_metadata_planes_t;
typedef libcamera::FrameMetadata libcamera_frame_metadata_t;
typedef libcamera::FrameBuffer::Plane libcamera_framebuffer_plane_t;
typedef std::vector<libcamera::FrameBuffer::Plane> libcamera_framebuffer_planes_t;
typedef libcamera::FrameBuffer libcamera_framebuffer_t;

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
const libcamera_framebuffer_planes_t *libcamera_framebuffer_planes(const libcamera_framebuffer_t *framebuffer);
const libcamera_frame_metadata_t *libcamera_framebuffer_metadata(const libcamera_framebuffer_t *framebuffer);
uint64_t libcamera_framebuffer_cookie(const libcamera_framebuffer_t *framebuffer);

// --- libcamera_framebuffer_plane_t ---
int libcamera_framebuffer_plane_fd(libcamera_framebuffer_plane_t *plane);
size_t libcamera_framebuffer_plane_offset(const libcamera_framebuffer_plane_t *plane);
bool libcamera_framebuffer_plane_offset_valid(const libcamera_framebuffer_plane_t *plane);
size_t libcamera_framebuffer_plane_length(const libcamera_framebuffer_plane_t *plane);

// --- libcamera_framebuffer_planes_t ---
size_t libcamera_framebuffer_planes_size(const libcamera_framebuffer_planes_t *planes);
libcamera_framebuffer_plane_t *libcamera_framebuffer_planes_at(libcamera_framebuffer_planes_t *planes, size_t index);

#ifdef __cplusplus
}
#endif

#endif
