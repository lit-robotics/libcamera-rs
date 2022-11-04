#include "camera.h"

#ifdef __cplusplus
extern "C" {
#endif

void libcamera_camera_destroy(libcamera_camera_t *cam) {
    delete cam;
}

const char *libcamera_camera_id(libcamera_camera_t *cam) {
    return cam->get()->id().c_str();
}

int libcamera_camera_acquire(libcamera_camera_t *cam) {
    return cam->get()->acquire();
}

int libcamera_camera_release(libcamera_camera_t *cam) {
    return cam->get()->release();
}

#ifdef __cplusplus
}
#endif
