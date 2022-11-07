#ifndef __LIBCAMERA_C_CAMERA__
#define __LIBCAMERA_C_CAMERA__

#include "controls.h"

#include <stddef.h>

#ifdef __cplusplus
#include <libcamera/camera.h>

typedef std::shared_ptr<libcamera::Camera> libcamera_camera_t;

extern "C" {
#else
typedef struct libcamera_camera_t libcamera_camera_t;
#endif

libcamera_camera_t *libcamera_camera_copy(libcamera_camera_t *cam);
void libcamera_camera_destroy(libcamera_camera_t *cam);
const char *libcamera_camera_id(const libcamera_camera_t *cam);
int libcamera_camera_acquire(libcamera_camera_t *cam);
int libcamera_camera_release(libcamera_camera_t *cam);
const libcamera_control_info_map_t *libcamera_camera_controls(const libcamera_camera_t *cam);
const libcamera_control_list_t *libcamera_camera_properties(const libcamera_camera_t *cam);
int libcamera_camera_start(libcamera_camera_t *cam, const libcamera_control_list_t *controls);
int libcamera_camera_stop(libcamera_camera_t *cam);

#ifdef __cplusplus
}
#endif

#endif
