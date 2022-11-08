#ifndef __LIBCAMERA_C_CONTROLS__
#define __LIBCAMERA_C_CONTROLS__

#include "controls_generated.h"

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
#include <libcamera/controls.h>

typedef libcamera::ControlValue libcamera_control_value_t;
typedef libcamera::ControlList libcamera_control_list_t;
typedef libcamera::ControlInfoMap libcamera_control_info_map_t;

extern "C" {
#else
typedef struct libcamera_control_value_t libcamera_control_value_t;
typedef struct libcamera_control_list_t libcamera_control_list_t;
typedef struct libcamera_control_info_map_t libcamera_control_info_map_t;
#endif

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
const libcamera_control_value_t *libcamera_control_list_get(const libcamera_control_list_t *list, enum libcamera_property_id id);

// --- libcamera_control_value_t ---
enum libcamera_control_type libcamera_control_value_type(const libcamera_control_value_t *val);
bool libcamera_control_value_is_none(const libcamera_control_value_t *val);
bool libcamera_control_value_is_array(const libcamera_control_value_t *val);
size_t libcamera_control_value_num_elements(const libcamera_control_value_t *val);
const void *libcamera_control_value_get(const libcamera_control_value_t *val);
void libcamera_control_value_set(libcamera_control_value_t *val, enum libcamera_control_type type, const void *data, size_t num_elements);

#ifdef __cplusplus
}
#endif

#endif
