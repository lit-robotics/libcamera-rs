#ifndef __LIBCAMERA_C_GEOMETRY__
#define __LIBCAMERA_C_GEOMETRY__

#include <stddef.h>
#include <stdint.h>

struct libcamera_point {
    int x;
    int y;
};

struct libcamera_size {
    unsigned int width;
    unsigned int height;
};

struct libcamera_size_range {
    struct libcamera_size min;
    struct libcamera_size max;
    unsigned int hStep;
    unsigned int vStep;
};

struct libcamera_rectangle {
    int x;
    int y;
    unsigned int width;
    unsigned int height;
};

#ifdef __cplusplus
#include <libcamera/camera.h>

typedef libcamera::Point libcamera_point_t;
static_assert(sizeof(struct libcamera_point) == sizeof(libcamera_point_t));
static_assert(offsetof(struct libcamera_point, x) == offsetof(libcamera_point_t, x));
static_assert(offsetof(struct libcamera_point, y) == offsetof(libcamera_point_t, y));

typedef libcamera::Size libcamera_size_t;
static_assert(sizeof(struct libcamera_size) == sizeof(libcamera_size_t));
static_assert(offsetof(struct libcamera_size, width) == offsetof(libcamera_size_t, width));
static_assert(offsetof(struct libcamera_size, height) == offsetof(libcamera_size_t, height));

typedef libcamera::SizeRange libcamera_size_range_t;
static_assert(sizeof(struct libcamera_size_range) == sizeof(libcamera_size_range_t));
static_assert(offsetof(struct libcamera_size_range, min) == offsetof(libcamera_size_range_t, min));
static_assert(offsetof(struct libcamera_size_range, max) == offsetof(libcamera_size_range_t, max));
static_assert(offsetof(struct libcamera_size_range, hStep) == offsetof(libcamera_size_range_t, hStep));
static_assert(offsetof(struct libcamera_size_range, vStep) == offsetof(libcamera_size_range_t, vStep));

typedef libcamera::Rectangle libcamera_rectangle_t;
static_assert(sizeof(struct libcamera_rectangle) == sizeof(libcamera_rectangle_t));
static_assert(offsetof(struct libcamera_rectangle, x) == offsetof(libcamera_rectangle_t, x));
static_assert(offsetof(struct libcamera_rectangle, y) == offsetof(libcamera_rectangle_t, y));
static_assert(offsetof(struct libcamera_rectangle, width) == offsetof(libcamera_rectangle_t, width));
static_assert(offsetof(struct libcamera_rectangle, height) == offsetof(libcamera_rectangle_t, height));

typedef std::vector<libcamera::Size> libcamera_sizes_t;

extern "C" {
#else
typedef struct libcamera_point libcamera_point_t;
typedef struct libcamera_size libcamera_size_t;
typedef struct libcamera_size_range libcamera_size_range_t;
typedef struct libcamera_rectangle libcamera_rectangle_t;

typedef struct libcamera_sizes libcamera_sizes_t;
#endif

void libcamera_sizes_destroy(libcamera_sizes_t *sizes);
size_t libcamera_sizes_size(const libcamera_sizes_t *sizes);
const libcamera_size_t *libcamera_sizes_at(const libcamera_sizes_t *sizes, size_t index);

#ifdef __cplusplus
}
#endif

#endif
