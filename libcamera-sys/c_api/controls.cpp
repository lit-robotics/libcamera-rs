#include "controls.h"

#include <libcamera/libcamera.h>

#ifdef __cplusplus
extern "C" {
#endif

const char *libcamera_control_name(enum libcamera_control_id id) {
    auto it = libcamera::controls::controls.find(id);
    if (it != libcamera::controls::controls.end())
        return it->second->name().c_str();
    else
        return nullptr;
}

enum libcamera_control_type libcamera_control_type(enum libcamera_control_id id) {
    auto it = libcamera::controls::controls.find(id);
    if (it != libcamera::controls::controls.end())
        return (enum libcamera_control_type)it->second->type();
    else
        return LIBCAMERA_CONTROL_TYPE_NONE;
}

const char *libcamera_property_name(enum libcamera_property_id id) {
    auto it = libcamera::properties::properties.find(id);
    if (it != libcamera::properties::properties.end())
        return it->second->name().c_str();
    else
        return nullptr;
}

enum libcamera_control_type libcamera_property_type(enum libcamera_property_id id) {
    auto it = libcamera::properties::properties.find(id);
    if (it != libcamera::properties::properties.end())
        return (enum libcamera_control_type)it->second->type();
    else
        return LIBCAMERA_CONTROL_TYPE_NONE;
}

// const libcamera_control_id_t *libcamera_control_id(unsigned int id) {
//     auto it = libcamera::controls::controls.find(id);
//     if (it != libcamera::controls::controls.end())
//         return it->second;
//     else
//         return nullptr;
// }

// const libcamera_control_id_t *libcamera_property_id(unsigned int id) {
//     auto it = libcamera::properties::properties.find(id);
//     if (it != libcamera::properties::properties.end())
//         return it->second;
//     else
//         return nullptr;
// }

// unsigned int libcamera_control_id_id(const libcamera_control_id_t *ctrl_id) {
//     return ctrl_id->id();
// }

// const char *libcamera_control_id_name(const libcamera_control_id_t *ctrl_id) {
//     return ctrl_id->name().c_str();
// }

// unsigned int libcamera_control_id_type(const libcamera_control_id_t *ctrl_id) {
//     return ctrl_id->type();
// }

#ifdef __cplusplus
}
#endif
