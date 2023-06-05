#ifndef __LIBCAMERA_C_LOGGING__
#define __LIBCAMERA_C_LOGGING__

#include <stdbool.h>

enum libcamera_logging_target {
    LIBCAMERA_LOGGING_TARGET_NONE,
    LIBCAMERA_LOGGING_TARGET_SYSLOG,
};

enum libcamera_logging_stream {
    LIBCAMERA_LOGGING_STREAM_STDOUT,
    LIBCAMERA_LOGGING_STREAM_STDERR,
};

typedef enum libcamera_logging_stream libcamera_logging_stream_t;

#ifdef __cplusplus

#include <ostream>
#include <libcamera/logging.h>

typedef libcamera::LoggingTarget libcamera_logging_target_t;

extern "C" {
#else
typedef enum libcamera_logging_target libcamera_logging_target_t;
#endif

int libcamera_log_set_file(const char *path, bool color);
int libcamera_log_set_stream(libcamera_logging_stream_t stream, bool color);
int libcamera_log_set_target(libcamera_logging_target_t target);
void libcamera_log_set_level(const char *category, const char *level);

#ifdef __cplusplus
}
#endif

#endif
