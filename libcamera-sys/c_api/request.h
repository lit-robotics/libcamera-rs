#ifndef __LIBCAMERA_C_REQUEST__
#define __LIBCAMERA_C_REQUEST__

#include <stdint.h>

#ifdef __cplusplus
#include <libcamera/camera.h>

typedef libcamera::Request libcamera_request_t;

extern "C" {
#else
typedef struct libcamera_request libcamera_request_t;
#endif

void libcamera_request_destroy(libcamera_request_t *request);

#ifdef __cplusplus
}
#endif

#endif
