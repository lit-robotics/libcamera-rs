#ifndef __LIBCAMERA_C_FRAMEBUFFER__
#define __LIBCAMERA_C_FRAMEBUFFER__

#include <stdint.h>

#ifdef __cplusplus
#include <libcamera/camera.h>

typedef libcamera::FrameBuffer libcamera_framebuffer_t;

extern "C" {
#else
typedef struct libcamera_framebuffer libcamera_framebuffer_t;
#endif



#ifdef __cplusplus
}
#endif

#endif
