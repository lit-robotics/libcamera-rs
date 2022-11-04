#pragma once

#ifdef __cplusplus
#include <libcamera/camera.h>

typedef std::shared_ptr<libcamera::Camera> libcamera_camera_t;

extern "C" {
#else
typedef struct libcamera_camera_t libcamera_camera_t;
#endif

void libcamera_camera_destroy(libcamera_camera_t *cam);
const char *libcamera_camera_id(libcamera_camera_t *cam);
int libcamera_camera_acquire(libcamera_camera_t *cam);
int libcamera_camera_release(libcamera_camera_t *cam);

#ifdef __cplusplus
}
#endif
