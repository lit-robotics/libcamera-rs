#include <libcamera/version.h>
#include "transform.h"
#include <cstring>

extern "C" {

static inline libcamera::Transform to_cpp(libcamera_transform_t t) {
    libcamera::Transform result;
    std::memcpy(&result, &t, sizeof(result));
    return result;
}

static inline libcamera_transform_t from_cpp(const libcamera::Transform &t) {
    libcamera_transform_t result;
    std::memcpy(&result, &t, sizeof(result));
    return result;
}

libcamera_transform_t libcamera_transform_identity() {
    return from_cpp(libcamera::Transform::Identity);
}

libcamera_transform_t libcamera_transform_from_rotation(int angle, bool hflip, bool *success) {
    libcamera::Transform t;
#if LIBCAMERA_VERSION_MAJOR == 0 && LIBCAMERA_VERSION_MINOR < 8
    bool ok = false;
    t = libcamera::transformFromRotation(angle, &ok);
#else
    auto result = libcamera::transformFromRotation(angle);
    bool ok = result.has_value();
    t = result.value_or(libcamera::Transform::Identity);
#endif
    if (!ok) {
        if (success)
            *success = false;
        return from_cpp(libcamera::Transform::Identity);
    }
    if (hflip)
        t = libcamera::Transform::HFlip * t;
    if (success)
        *success = ok;
    return from_cpp(t);
}

libcamera_transform_t libcamera_transform_inverse(libcamera_transform_t transform) {
    return from_cpp(-to_cpp(transform));
}

libcamera_transform_t libcamera_transform_combine(libcamera_transform_t a, libcamera_transform_t b) {
    return from_cpp(to_cpp(a) * to_cpp(b));
}

char *libcamera_transform_to_string(libcamera_transform_t transform) {
    return ::strdup(libcamera::transformToString(to_cpp(transform)));
}

libcamera_transform_t libcamera_transform_between_orientations(libcamera_orientation_t from, libcamera_orientation_t to) {
    return from_cpp(from / to);
}

libcamera_orientation_t libcamera_transform_apply_orientation(libcamera_orientation_t orientation, libcamera_transform_t transform) {
    return orientation * to_cpp(transform);
}

libcamera_orientation_t libcamera_orientation_from_rotation(int angle, bool *success) {
    libcamera::Orientation ori;
#if LIBCAMERA_VERSION_MAJOR == 0 && LIBCAMERA_VERSION_MINOR < 8
    bool ok = false;
    ori = libcamera::orientationFromRotation(angle, &ok);
#else
    auto result = libcamera::orientationFromRotation(angle);
    bool ok = result.has_value();
    ori = result.value_or(libcamera::Orientation::Rotate0);
#endif
    if (success)
        *success = ok;
    return ori;
}

libcamera_transform_t libcamera_transform_hflip() {
    return from_cpp(libcamera::Transform::HFlip);
}

libcamera_transform_t libcamera_transform_vflip() {
    return from_cpp(libcamera::Transform::VFlip);
}

libcamera_transform_t libcamera_transform_transpose() {
    return from_cpp(libcamera::Transform::Transpose);
}

libcamera_transform_t libcamera_transform_or(libcamera_transform_t a, libcamera_transform_t b) {
    return from_cpp(to_cpp(a) | to_cpp(b));
}

libcamera_transform_t libcamera_transform_and(libcamera_transform_t a, libcamera_transform_t b) {
    return from_cpp(to_cpp(a) & to_cpp(b));
}

libcamera_transform_t libcamera_transform_xor(libcamera_transform_t a, libcamera_transform_t b) {
    return from_cpp(to_cpp(a) ^ to_cpp(b));
}

libcamera_transform_t libcamera_transform_not(libcamera_transform_t t) {
    return from_cpp(~to_cpp(t));
}

}
