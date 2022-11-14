#ifndef __LIBCAMERA_C_CAMERA__
#define __LIBCAMERA_C_CAMERA__

#include "controls.h"
#include "request.h"
#include "signal.h"
#include "stream.h"

#include <stddef.h>

enum libcamera_camera_configuration_status {
    LIBCAMERA_CAMERA_CONFIGURATION_STATUS_VALID,
    LIBCAMERA_CAMERA_CONFIGURATION_STATUS_ADJUSTED,
    LIBCAMERA_CAMERA_CONFIGURATION_STATUS_INVALID,
};

typedef void libcamera_request_completed_cb_t(void*, libcamera_request_t*);

#ifdef __cplusplus
#include <libcamera/camera.h>

typedef libcamera::CameraConfiguration libcamera_camera_configuration_t;
typedef libcamera::CameraConfiguration::Status libcamera_camera_configuration_status_t;
typedef std::shared_ptr<libcamera::Camera> libcamera_camera_t;

extern "C" {
#else
typedef enum libcamera_camera_configuration_status libcamera_camera_configuration_status_t;
typedef struct libcamera_camera_configuration_t libcamera_camera_configuration_t;
typedef struct libcamera_camera_t libcamera_camera_t;
#endif

void libcamera_camera_configuration_destroy(libcamera_camera_configuration_t* config);
size_t libcamera_camera_configuration_size(const libcamera_camera_configuration_t* config);
libcamera_stream_configuration_t *libcamera_camera_configuration_at(libcamera_camera_configuration_t* config, size_t index);
libcamera_camera_configuration_status_t libcamera_camera_configuration_validate(libcamera_camera_configuration_t* config);

libcamera_camera_t *libcamera_camera_copy(libcamera_camera_t *cam);
void libcamera_camera_destroy(libcamera_camera_t *cam);
const char *libcamera_camera_id(const libcamera_camera_t *cam);
libcamera_callback_handle_t *libcamera_camera_request_completed_connect(libcamera_camera_t *cam, libcamera_request_completed_cb_t *callback, void *data);
void libcamera_camera_request_completed_disconnect(libcamera_camera_t *cam, libcamera_callback_handle_t *handle);
int libcamera_camera_acquire(libcamera_camera_t *cam);
int libcamera_camera_release(libcamera_camera_t *cam);
const libcamera_control_info_map_t *libcamera_camera_controls(const libcamera_camera_t *cam);
const libcamera_control_list_t *libcamera_camera_properties(const libcamera_camera_t *cam);
libcamera_camera_configuration_t *libcamera_camera_generate_configuration(libcamera_camera_t *cam, const enum libcamera_stream_role *roles, size_t role_count);
int libcamera_camera_configure(libcamera_camera_t *cam, libcamera_camera_configuration_t *config);
libcamera_request_t *libcamera_camera_create_request(libcamera_camera_t *cam, uint64_t cookie);
int libcamera_camera_queue_request(libcamera_camera_t *cam, libcamera_request_t *request);
int libcamera_camera_start(libcamera_camera_t *cam, const libcamera_control_list_t *controls);
int libcamera_camera_stop(libcamera_camera_t *cam);

#ifdef __cplusplus
}
#endif

#endif
