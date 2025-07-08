#include "controls.h"

#include <libcamera/libcamera.h>
#include <string.h>

extern "C" {

enum libcamera_control_id_enum libcamera_control_id(libcamera_control_id_t *control){
    return (enum libcamera_control_id_enum)control->id();
}

const char *libcamera_control_name(libcamera_control_id_t *control){
    return control->name().c_str();
}

enum libcamera_control_type libcamera_control_type(libcamera_control_id_t *control) {
    return (enum libcamera_control_type) control->type();
}

const libcamera_control_id_t *libcamera_control_from_id(enum libcamera_control_id_enum id){
     auto it = libcamera::controls::controls.find(id);
    if (it != libcamera::controls::controls.end())
        return it->second;
    else
        return nullptr;
}

const char *libcamera_control_name_from_id(enum libcamera_control_id_enum id) {
    auto it = libcamera::controls::controls.find(id);
    if (it != libcamera::controls::controls.end())
        return it->second->name().c_str();
    else
        return nullptr;
}

enum libcamera_control_type libcamera_control_type_from_id(enum libcamera_control_id_enum id) {
    auto it = libcamera::controls::controls.find(id);
    if (it != libcamera::controls::controls.end())
        return (enum libcamera_control_type)it->second->type();
    else
        return LIBCAMERA_CONTROL_TYPE_NONE;
}

const char *libcamera_property_name_by_id(enum libcamera_property_id id) {
    auto it = libcamera::properties::properties.find(id);
    if (it != libcamera::properties::properties.end())
        return it->second->name().c_str();
    else
        return nullptr;
}

enum libcamera_control_type libcamera_property_type_by_id(enum libcamera_property_id id) {
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

size_t libcamera_control_value_size() {
     return sizeof(libcamera::ControlValue);
}

const libcamera_control_value_t *libcamera_control_info_max(libcamera_control_info_t *val){
    return &val->max();
}
const libcamera_control_value_t *libcamera_control_info_min(libcamera_control_info_t *val){
    return &val->min();
}
const libcamera_control_value_t *libcamera_control_info_def(libcamera_control_info_t *val){
    return &val->def();
}

const libcamera_control_value_t* libcamera_control_info_values(const libcamera_control_info_t* info, size_t* size)
{
    if (!info || !size) return nullptr;
    const std::vector<libcamera::ControlValue>& values = info->values();
    *size = values.size();
    return reinterpret_cast<const libcamera_control_value_t*>(values.data());
}

bool libcamera_control_id_map_add(libcamera_control_id_map_t *idmap, unsigned int key, const libcamera_control_id_t *control_id)
{
	if (!idmap || !control_id)
		return false;

	(*idmap)[key] = control_id;
	return true;
}

const libcamera_control_id_t *libcamera_control_id_map_get(libcamera_control_id_map_t *idmap, unsigned int key)
{
	if (!idmap)
		return nullptr;

	auto it = idmap->find(key);
	if (it != idmap->end())
		return it->second;
	return nullptr;
}


const libcamera_control_info_t *libcamera_control_info_map_at(libcamera_control_info_map_t *map, unsigned int key)
{
	if (!map)
		return nullptr;

	try {
		return &map->at(key);
	} catch (const std::out_of_range &) {
		return nullptr;
	}
}

size_t libcamera_control_info_map_count(const libcamera_control_info_map_t *map, unsigned int key)
{
	if (!map)
		return 0;

	return map->count(key);
}

size_t libcamera_control_info_map_size(const libcamera_control_info_map_t *map)
{
	if (!map)
		return 0;

	return map->size();
}

const libcamera_control_info_t *libcamera_control_info_map_find(const libcamera_control_info_map_t *map, unsigned int key)
{
	if (!map)
		return nullptr;

	auto it = map->find(key);
	if (it != map->end()) {
		return &it->second;
	}
 
	return nullptr;
}


libcamera_control_info_map_iter_t* libcamera_control_info_map_iter_create(const libcamera_control_info_map_t* map) {
    if (!map) return nullptr;
    libcamera_control_info_map_iter_t* iter = new libcamera_control_info_map_iter_t();
    iter->current = map->begin();
    iter->end = map->end();
    return iter;
}

bool libcamera_control_info_map_iter_has_next(const libcamera_control_info_map_iter_t* iter) {
    if (!iter) return false;
    return iter->current != iter->end;
}

unsigned int libcamera_control_info_map_iter_key(const libcamera_control_info_map_iter_t* iter) {
    if (!iter || iter->current == iter->end) return 0;
    return iter->current->first->id();
}

const libcamera_control_info_t* libcamera_control_info_map_iter_value(const libcamera_control_info_map_iter_t* iter) {
    if (!iter || iter->current == iter->end) return nullptr;
    return &(iter->current->second);
}

void libcamera_control_info_map_iter_next(libcamera_control_info_map_iter_t* iter) {
    if (!iter || iter->current == iter->end) return;
    ++(iter->current);
}

void libcamera_control_info_map_iter_destroy(libcamera_control_info_map_iter_t* iter) {
    delete iter;
}


}
