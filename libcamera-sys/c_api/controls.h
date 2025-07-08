#ifndef __LIBCAMERA_C_CONTROLS__
#define __LIBCAMERA_C_CONTROLS__

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
#include <libcamera/controls.h>

struct libcamera_control_list_iter {
	libcamera::ControlList *list;
	libcamera::ControlList::iterator it;
};

struct libcamera_control_info_map_iter {
    libcamera::ControlInfoMap::const_iterator current;
    libcamera::ControlInfoMap::const_iterator end;
};


typedef libcamera::ControlValue libcamera_control_value_t;
typedef libcamera::ControlList libcamera_control_list_t;
typedef struct libcamera_control_list_iter libcamera_control_list_iter_t;
typedef struct libcamera_control_info_map_iter libcamera_control_info_map_iter_t;
typedef libcamera::ControlInfoMap libcamera_control_info_map_t;
typedef libcamera::ControlIdMap libcamera_control_id_map_t;
typedef libcamera::ControlId libcamera_control_id_t;
typedef libcamera::ControlInfo libcamera_control_info_t;

extern "C" {
#else
typedef struct libcamera_control_value libcamera_control_value_t;
typedef struct libcamera_control_list libcamera_control_list_t;
typedef struct libcamera_control_list_iter libcamera_control_list_iter_t;
typedef struct libcamera_control_info_map_iter libcamera_control_info_map_iter_t;
typedef struct libcamera_control_info_map libcamera_control_info_map_t;
typedef struct libcamera_control_id libcamera_control_id_t;
typedef struct libcamera_control_info libcamera_control_info_t;
typedef struct libcamera_control_id_map libcamera_control_id_map_t;

#endif

enum libcamera_control_id_enum { libcamera_control_id_DUMMY };
enum libcamera_property_id { libcamera_property_id_DUMMY };

enum libcamera_control_type {
	LIBCAMERA_CONTROL_TYPE_NONE,
	LIBCAMERA_CONTROL_TYPE_BOOL,
	LIBCAMERA_CONTROL_TYPE_BYTE,
	LIBCAMERA_CONTROL_TYPE_UINT16,
	LIBCAMERA_CONTROL_TYPE_UINT32,
	LIBCAMERA_CONTROL_TYPE_INT32,
	LIBCAMERA_CONTROL_TYPE_INT64,
	LIBCAMERA_CONTROL_TYPE_FLOAT,
	LIBCAMERA_CONTROL_TYPE_STRING,
	LIBCAMERA_CONTROL_TYPE_RECTANGLE,
	LIBCAMERA_CONTROL_TYPE_SIZE,
	LIBCAMERA_CONTROL_TYPE_POINT,
};

// --- libcamera_control_id ---
const libcamera_control_id_t *libcamera_control_from_id(enum libcamera_control_id_enum id);
const char *libcamera_control_name_from_id(enum libcamera_control_id_enum id);
enum libcamera_control_type libcamera_control_type_from_id(enum libcamera_control_id_enum id);

enum libcamera_control_id_enum libcamera_control_id(libcamera_control_id_t *control);
const char *libcamera_control_name(libcamera_control_id_t *control);
enum libcamera_control_type libcamera_control_type(libcamera_control_id_t *control);
// --- libcamera_property_id ---
const char *libcamera_property_name_by_id(enum libcamera_property_id id);
enum libcamera_control_type libcamera_property_type_by_id(enum libcamera_property_id id);

// --- libcamera_control_list_t ---
libcamera_control_list_t *libcamera_control_list_create();
void libcamera_control_list_destroy(libcamera_control_list_t *list);
const libcamera_control_value_t *libcamera_control_list_get(libcamera_control_list_t *list, enum libcamera_property_id id);
void libcamera_control_list_set(libcamera_control_list_t *list, enum libcamera_property_id id, const libcamera_control_value_t *val);
libcamera_control_list_iter_t *libcamera_control_list_iter(libcamera_control_list_t *list);

// --- libcamera_control_list_iter_t ---
void libcamera_control_list_iter_destroy(libcamera_control_list_iter_t *iter);
bool libcamera_control_list_iter_end(const libcamera_control_list_iter_t *iter);
void libcamera_control_list_iter_next(libcamera_control_list_iter_t *iter);
unsigned int libcamera_control_list_iter_id(libcamera_control_list_iter_t *iter);
const libcamera_control_value_t *libcamera_control_list_iter_value(libcamera_control_list_iter_t *iter);

// --- libcamera_control_value_t ---
libcamera_control_value_t *libcamera_control_value_create();
void libcamera_control_value_destroy(libcamera_control_value_t *val);
enum libcamera_control_type libcamera_control_value_type(const libcamera_control_value_t *val);
bool libcamera_control_value_is_none(const libcamera_control_value_t *val);
bool libcamera_control_value_is_array(const libcamera_control_value_t *val);
size_t libcamera_control_value_num_elements(const libcamera_control_value_t *val);
const void *libcamera_control_value_get(const libcamera_control_value_t *val);
void libcamera_control_value_set(libcamera_control_value_t *val, enum libcamera_control_type type, const void *data, bool is_array, size_t num_elements);
size_t libcamera_control_value_size();
// --- libcamera_control_info_t ---
const libcamera_control_value_t *libcamera_control_info_max(libcamera_control_info_t *val);
const libcamera_control_value_t *libcamera_control_info_min(libcamera_control_info_t *val);
const libcamera_control_value_t *libcamera_control_info_def(libcamera_control_info_t *val);
const libcamera_control_value_t* libcamera_control_info_values(const libcamera_control_info_t* info, size_t* size);
// --- libcamera_control_id_map ---
bool libcamera_control_id_map_add(libcamera_control_id_map_t *idmap, unsigned int key, const libcamera_control_id_t *control_id);
const libcamera_control_id_t *libcamera_control_id_map_get(libcamera_control_id_map_t *idmap, unsigned int key);
 
// --- libcamera_control_info_map ---
const libcamera_control_info_t *libcamera_control_info_map_at(libcamera_control_info_map_t *map, unsigned int key);
size_t libcamera_control_info_map_size(const libcamera_control_info_map_t *map);
size_t libcamera_control_info_map_count(const libcamera_control_info_map_t *map, unsigned int key);
const libcamera_control_info_t * libcamera_control_info_map_find(const libcamera_control_info_map_t *map, unsigned int key);

// --- libcamera_control_info_map_iter_t ---
libcamera_control_info_map_iter_t* libcamera_control_info_map_iter_create(const libcamera_control_info_map_t* map);
bool libcamera_control_info_map_iter_has_next(const libcamera_control_info_map_iter_t* iter);
unsigned int libcamera_control_info_map_iter_key(const libcamera_control_info_map_iter_t* iter);
const libcamera_control_info_t* libcamera_control_info_map_iter_value(const libcamera_control_info_map_iter_t* iter);
void libcamera_control_info_map_iter_next(libcamera_control_info_map_iter_t* iter);
void libcamera_control_info_map_iter_destroy(libcamera_control_info_map_iter_t* iter);

#ifdef __cplusplus
}
#endif

#endif
