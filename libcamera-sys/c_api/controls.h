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

typedef libcamera::ControlValue libcamera_control_value_t;
typedef libcamera::ControlList libcamera_control_list_t;
typedef struct libcamera_control_list_iter libcamera_control_list_iter_t;
typedef libcamera::ControlInfoMap libcamera_control_info_map_t;

extern "C" {
#else
typedef struct libcamera_control_value libcamera_control_value_t;
typedef struct libcamera_control_list libcamera_control_list_t;
typedef struct libcamera_control_list_iter libcamera_control_list_iter_t;
typedef struct libcamera_control_info_map libcamera_control_info_map_t;
#endif

enum libcamera_control_id { libcamera_control_id_DUMMY };
enum libcamera_property_id { libcamera_property_id_DUMMY };

enum libcamera_control_type {
	LIBCAMERA_CONTROL_TYPE_NONE,
	LIBCAMERA_CONTROL_TYPE_BOOL,
	LIBCAMERA_CONTROL_TYPE_BYTE,
	LIBCAMERA_CONTROL_TYPE_INT32,
	LIBCAMERA_CONTROL_TYPE_INT64,
	LIBCAMERA_CONTROL_TYPE_FLOAT,
	LIBCAMERA_CONTROL_TYPE_STRING,
	LIBCAMERA_CONTROL_TYPE_RECTANGLE,
	LIBCAMERA_CONTROL_TYPE_SIZE,
};

// --- libcamera_control_id ---
const char *libcamera_control_name(enum libcamera_control_id id);
enum libcamera_control_type libcamera_control_type(enum libcamera_control_id id);

// --- libcamera_property_id ---
const char *libcamera_property_name(enum libcamera_property_id id);
enum libcamera_control_type libcamera_property_type(enum libcamera_property_id id);

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

#ifdef __cplusplus
}
#endif

#endif
