#include "camera_manager.h"

#include <libcamera/camera_manager.h>

extern "C" {

libcamera_camera_manager_t *libcamera_camera_manager_create() {
    return new libcamera::CameraManager();
}

void libcamera_camera_manager_destroy(libcamera_camera_manager_t *mgr) {
    delete mgr;
}

int libcamera_camera_manager_start(libcamera_camera_manager_t *mgr) {
    return mgr->start();
}

void libcamera_camera_manager_stop(libcamera_camera_manager_t *mgr) {
    mgr->stop();
}

libcamera_camera_list_t *libcamera_camera_manager_cameras(const libcamera_camera_manager_t *mgr) {
    return new libcamera_camera_list_t(std::move(mgr->cameras()));
}

libcamera_camera_t *libcamera_camera_manager_get_id(libcamera_camera_manager_t *mgr, const char *id) {
    auto camera = mgr->get(std::string(id));

    if (camera == nullptr)
        return NULL;
    else
        return new libcamera_camera_t(camera);
}

const char *libcamera_camera_manager_version(libcamera_camera_manager_t *mgr) {
    return mgr->version().c_str();
}

size_t libcamera_camera_list_size(libcamera_camera_list_t *list) {
    return list->size();
}

libcamera_camera_t *libcamera_camera_list_get(libcamera_camera_list_t *list, size_t index) {
    if (list->size() <= index)
        return nullptr;
    else
        return new libcamera_camera_t(list->at(index));
}

void libcamera_camera_list_destroy(libcamera_camera_list_t *list) {
    delete list;
}

}
