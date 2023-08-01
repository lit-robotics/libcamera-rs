#include "camera.h"

extern "C" {

void libcamera_camera_configuration_destroy(libcamera_camera_configuration_t* config) {
    delete config;
}

size_t libcamera_camera_configuration_size(const libcamera_camera_configuration_t* config) {
    return config->size();
}

libcamera_stream_configuration_t *libcamera_camera_configuration_at(libcamera_camera_configuration_t* config, size_t index) {
    if (config->size() > index) {
        return &config->at(index);
    } else {
        return nullptr;
    }
}

libcamera_camera_configuration_status_t libcamera_camera_configuration_validate(libcamera_camera_configuration_t* config) {
    return config->validate();
}

libcamera_camera_t* libcamera_camera_copy(libcamera_camera_t *cam) {
    const libcamera_camera_t& ptr = *cam;
    return new libcamera_camera_t(ptr);
}

void libcamera_camera_destroy(libcamera_camera_t *cam) {
    delete cam;
}

const char *libcamera_camera_id(const libcamera_camera_t *cam) {
    return cam->get()->id().c_str();
}

libcamera_callback_handle_t *libcamera_camera_request_completed_connect(libcamera_camera_t *cam, libcamera_request_completed_cb_t *callback, void *data) {
    libcamera_callback_handle_t *handle = new libcamera_callback_handle_t {};

    cam->get()->requestCompleted.connect(handle, [=](libcamera::Request *request) {
        callback(data, request);
    });

    return handle;
}

void libcamera_camera_request_completed_disconnect(libcamera_camera_t *cam, libcamera_callback_handle_t *handle) {
    cam->get()->requestCompleted.disconnect(handle);
    delete handle;
}

int libcamera_camera_acquire(libcamera_camera_t *cam) {
    return cam->get()->acquire();
}

int libcamera_camera_release(libcamera_camera_t *cam) {
    return cam->get()->release();
}

const libcamera_control_info_map_t *libcamera_camera_controls(const libcamera_camera_t *cam) {
    return &cam->get()->controls();
}

const libcamera_control_list_t *libcamera_camera_properties(const libcamera_camera_t *cam) {
    return &cam->get()->properties();
}

libcamera_camera_configuration_t *libcamera_camera_generate_configuration(libcamera_camera_t *cam, const enum libcamera_stream_role *roles, size_t role_count) {
    std::vector<libcamera::StreamRole> roles_vec((libcamera::StreamRole*)roles, (libcamera::StreamRole*)roles + role_count);
    return cam->get()->generateConfiguration(roles_vec).release();
}

int libcamera_camera_configure(libcamera_camera_t *cam, libcamera_camera_configuration_t *config) {
    return cam->get()->configure(config);
}

libcamera_request_t *libcamera_camera_create_request(libcamera_camera_t *cam, uint64_t cookie) {
    return cam->get()->createRequest(cookie).release();
}

int libcamera_camera_queue_request(libcamera_camera_t *cam, libcamera_request_t *request) {
    return cam->get()->queueRequest(request);
}

int libcamera_camera_start(libcamera_camera_t *cam, const libcamera_control_list_t *controls) {
    return cam->get()->start(controls);
}

int libcamera_camera_stop(libcamera_camera_t *cam) {
    return cam->get()->stop();
}

}
