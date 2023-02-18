#include "controls.h"

#include <libcamera/libcamera.h>
#include <string.h>

extern "C" {

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

libcamera_control_list_t *libcamera_control_list_create() {
    return new libcamera::ControlList();
}

void libcamera_control_list_destroy(libcamera_control_list_t *list) {
    delete list;
}

const libcamera_control_value_t *libcamera_control_list_get(libcamera_control_list_t *list, enum libcamera_property_id id) {
    if (list->contains(id)) {
        return &list->get(id);
    } else {
        return nullptr;
    }
}

void libcamera_control_list_set(libcamera_control_list_t *list, enum libcamera_property_id id, const libcamera_control_value_t *val) {
    // It would be nice to report status of this operation, however API does not provide any feedback
    // and internally used `_validator` is private.
    list->set(id, *val);
}

libcamera_control_list_iter_t *libcamera_control_list_iter(libcamera_control_list_t *list) {
    auto it = list->begin();
    return new libcamera_control_list_iter_t { list, it };
}

void libcamera_control_list_iter_destroy(libcamera_control_list_iter_t *iter) {
    delete iter;
}

bool libcamera_control_list_iter_end(const libcamera_control_list_iter_t *iter) {
    return iter->it == iter->list->end();
}

void libcamera_control_list_iter_next(libcamera_control_list_iter_t *iter) {
    if (iter->it != iter->list->end()) {
        ++(iter->it);
    }
}

unsigned int libcamera_control_list_iter_id(libcamera_control_list_iter_t *iter) {
    return iter->it->first;
}

const libcamera_control_value_t *libcamera_control_list_iter_value(libcamera_control_list_iter_t *iter) {
    return &iter->it->second;
}

libcamera_control_value_t *libcamera_control_value_create() {
    return new libcamera::ControlValue();
}

void libcamera_control_value_destroy(libcamera_control_value_t *val) {
    delete val;
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

void libcamera_control_value_set(libcamera_control_value_t *val, enum libcamera_control_type type, const void *data, bool is_array, size_t num_elements) {
    val->reserve((libcamera::ControlType)type, is_array, num_elements);
    libcamera::Span<uint8_t> storage = val->data();
    memcpy(storage.data(), data, storage.size());
}

}
