#include "request.h"

extern "C" {

void libcamera_request_destroy(libcamera_request_t *request) {
    delete request;
}

}
