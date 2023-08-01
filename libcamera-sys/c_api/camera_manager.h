#ifndef __LIBCAMERA_C_CAMERA_MANAGER__
#define __LIBCAMERA_C_CAMERA_MANAGER__

#include "camera.h"

#include <stddef.h>
#include <sys/types.h>

#ifdef __cplusplus
#include <libcamera/camera_manager.h>

typedef libcamera::CameraManager libcamera_camera_manager_t;
typedef std::vector<std::shared_ptr<libcamera::Camera>> libcamera_camera_list_t;

extern "C" {
#else
typedef struct libcamera_camera_manager_t libcamera_camera_manager_t;
typedef struct libcamera_camera_list_t libcamera_camera_list_t;
#endif

libcamera_camera_manager_t *libcamera_camera_manager_create();
void libcamera_camera_manager_destroy(libcamera_camera_manager_t *mgr);
int libcamera_camera_manager_start(libcamera_camera_manager_t *mgr);
void libcamera_camera_manager_stop(libcamera_camera_manager_t *mgr);
libcamera_camera_list_t *libcamera_camera_manager_cameras(const libcamera_camera_manager_t *mgr);
libcamera_camera_t *libcamera_camera_manager_get_id(libcamera_camera_manager_t *mgr, const char *id);
const char *libcamera_camera_manager_version(libcamera_camera_manager_t *mgr);

void libcamera_camera_list_destroy(libcamera_camera_list_t *list);
size_t libcamera_camera_list_size(libcamera_camera_list_t *list);
libcamera_camera_t *libcamera_camera_list_get(libcamera_camera_list_t *list, size_t index);

#ifdef __cplusplus
}
#endif

#endif
