#include "controls.h"

#include <libcamera/libcamera.h>
#include <string.h>

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

// const char *libcamera_control_list_get_str(const libcamera_control_list_t *list, enum libcamera_property_id id) {
//     const libcamera::ControlValue &val = list->get(id);
//     if (val.type() == LIBCAMERA_CONTROL_TYPE_STRING) {
//         return val.get<std::string>().c_str();
//     } else {
//         return nullptr;
//     }
// }

const libcamera_control_value_t *libcamera_control_list_get(const libcamera_control_list_t *list, enum libcamera_property_id id) {
    if (list->contains(id)) {
        return &list->get(id);
    } else {
        return nullptr;
    }
}

enum libcamera_control_type libcamera_control_value_type(const libcamera_control_value_t *val) {
    return (enum libcamera_control_type)val->type();
}

bool libcamera_control_value_is_none(const libcamera_control_value_t *val) {
    return val->isNone();
}

bool libcamera_control_value_is_array(const libcamera_control_value_t *val) {
    return val->isArray();
}

size_t libcamera_control_value_num_elements(const libcamera_control_value_t *val) {
    return val->numElements();
}

const void *libcamera_control_value_get(const libcamera_control_value_t *val) {
    return (const void*)val->data().data();
}

void libcamera_control_value_set(libcamera_control_value_t *val, enum libcamera_control_type type, const void *data, size_t num_elements) {
    val->reserve((libcamera::ControlType)type, num_elements != 1, num_elements);
    libcamera::Span<uint8_t> storage = val->data();
    memcpy(storage.data(), data, storage.size());
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
