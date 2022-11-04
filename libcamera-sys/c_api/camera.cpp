#include "camera.h"

#ifdef __cplusplus
extern "C" {
#endif

libcamera_camera_t* libcamera_camera_copy(libcamera_camera_t *cam) {
    return new libcamera_camera_t(*cam);
}

void libcamera_camera_destroy(libcamera_camera_t *cam) {
    delete cam;
}

const char *libcamera_camera_id(const libcamera_camera_t *cam) {
    return cam->get()->id().c_str();
}

int libcamera_camera_acquire(libcamera_camera_t *cam) {
    return cam->get()->acquire();
}

int libcamera_camera_release(libcamera_camera_t *cam) {
    return cam->get()->release();
}

int libcamera_camera_start(libcamera_camera_t *cam, const libcamera_control_list_t *controls) {
    return cam->get()->start(controls);
}

int libcamera_camera_stop(libcamera_camera_t *cam) {
    return cam->get()->stop();
}

#ifdef __cplusplus
}
#endif
