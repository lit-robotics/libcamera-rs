#include "framebuffer.h"
#include "fence.h"

#include <vector>
#include <libcamera/base/shared_fd.h>
#include <unistd.h>

extern "C" {

// --- libcamera_frame_metadata_t ---
libcamera_frame_metadata_status_t libcamera_frame_metadata_status(const libcamera_frame_metadata_t *metadata) {
    return metadata->status;
}

unsigned int libcamera_frame_metadata_sequence(const libcamera_frame_metadata_t *metadata) {
    return metadata->sequence;
}

uint64_t libcamera_frame_metadata_timestamp(const libcamera_frame_metadata_t *metadata) {
    return metadata->timestamp;
}

libcamera_frame_metadata_planes_t *libcamera_frame_metadata_planes(libcamera_frame_metadata_t *metadata) {
    return new libcamera_frame_metadata_planes_t(metadata->planes());
}

// --- libcamera_frame_metadata_planes_t ---
void libcamera_frame_metadata_planes_destroy(libcamera_frame_metadata_planes_t *planes) {
    delete planes;
}

size_t libcamera_frame_metadata_planes_size(const libcamera_frame_metadata_planes_t *planes) {
    return planes->size();
}

libcamera_frame_metadata_plane_t *libcamera_frame_metadata_planes_at(libcamera_frame_metadata_planes_t *planes, size_t index) {
    return &planes->data()[index];
}

// --- libcamera_framebuffer_t ---
libcamera_framebuffer_t *libcamera_framebuffer_create(const struct libcamera_framebuffer_plane_info *planes, size_t num_planes, uint64_t cookie) {
    if (!planes || num_planes == 0)
        return nullptr;

    std::vector<libcamera::FrameBuffer::Plane> fb_planes;
    fb_planes.reserve(num_planes);

    for (size_t i = 0; i < num_planes; ++i) {
        const auto &p = planes[i];
        libcamera::SharedFD fd(p.fd);
        if (!fd.isValid())
            return nullptr;

        libcamera::FrameBuffer::Plane plane;
        plane.fd = std::move(fd);
        plane.offset = p.offset;
        plane.length = p.length;
        fb_planes.push_back(std::move(plane));
    }

    return new libcamera::FrameBuffer(fb_planes, cookie);
}

void libcamera_framebuffer_destroy(libcamera_framebuffer_t *framebuffer) {
    delete framebuffer;
}

struct libcamera_framebuffer_planes {
    std::vector<libcamera::FrameBuffer::Plane> planes;
};

libcamera_framebuffer_planes_t *libcamera_framebuffer_planes(const libcamera_framebuffer_t *framebuffer) {
    auto wrapper = new libcamera_framebuffer_planes_t();
    const auto planes = framebuffer->planes();
    wrapper->planes.assign(planes.begin(), planes.end());
    return wrapper;
}

const libcamera_frame_metadata_t *libcamera_framebuffer_metadata(const libcamera_framebuffer_t *framebuffer) {
    return &framebuffer->metadata();
}

uint64_t libcamera_framebuffer_cookie(const libcamera_framebuffer_t *framebuffer) {
    return framebuffer->cookie();
}

void libcamera_framebuffer_set_cookie(libcamera_framebuffer_t *framebuffer, uint64_t cookie) {
    framebuffer->setCookie(cookie);
}

libcamera_fence_t *libcamera_framebuffer_release_fence_handle(libcamera_framebuffer_t *framebuffer) {
    auto fence = framebuffer->releaseFence();
    return fence.release();
}

int libcamera_framebuffer_release_fence(libcamera_framebuffer_t *framebuffer) {
    auto fence = framebuffer->releaseFence();
    if (!fence)
        return -1;
    int fd = fence->fd().get();
    if (fd < 0)
        return -1;
    return ::dup(fd);
}

libcamera_request_t *libcamera_framebuffer_request(const libcamera_framebuffer_t *framebuffer) {
    return framebuffer->request();
}

// --- libcamera_framebuffer_plane_t ---
int libcamera_framebuffer_plane_fd(libcamera_framebuffer_plane_t *plane) {
    return plane->fd.get();
}

size_t libcamera_framebuffer_plane_offset(const libcamera_framebuffer_plane_t *plane) {
    return plane->offset;
}

bool libcamera_framebuffer_plane_offset_valid(const libcamera_framebuffer_plane_t *plane) {
    return plane->offset != plane->kInvalidOffset;
}

size_t libcamera_framebuffer_plane_length(const libcamera_framebuffer_plane_t *plane) {
    return plane->length;
}

// --- libcamera_framebuffer_planes_t ---
void libcamera_framebuffer_planes_destroy(libcamera_framebuffer_planes_t *planes) {
    delete planes;
}

size_t libcamera_framebuffer_planes_size(const libcamera_framebuffer_planes_t *planes) {
    return planes->planes.size();
}

libcamera_framebuffer_plane_t *libcamera_framebuffer_planes_at(libcamera_framebuffer_planes_t *planes, size_t index) {
    return &planes->planes.at(index);
}

}
