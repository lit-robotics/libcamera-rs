#include <iostream>
#include "logging.h"

int libcamera_log_set_file(const char *path, bool color) {
    return libcamera::logSetFile(path, color);
}

int libcamera_log_set_stream(libcamera_logging_stream_t stream, bool color) {
    std::ostream *ostream = NULL;
    switch (stream) {
        case LIBCAMERA_LOGGING_STREAM_STDOUT:
            ostream = &std::cout;
        break;

        case LIBCAMERA_LOGGING_STREAM_STDERR:
            ostream = &std::cerr;
        break;
    }
    return libcamera::logSetStream(ostream, color);
}

int libcamera_log_set_target(libcamera_logging_target_t target) {
    return libcamera::logSetTarget(target);
}

void libcamera_log_set_level(const char *category, const char *level) {
    libcamera::logSetLevel(category, level);
}
