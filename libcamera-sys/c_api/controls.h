#ifndef __LIBCAMERA_C_CONTROLS__
#define __LIBCAMERA_C_CONTROLS__

#include "controls_generated.h"

#ifdef __cplusplus
#include <libcamera/controls.h>

typedef libcamera::ControlList libcamera_control_list_t;
typedef libcamera::ControlInfoMap libcamera_control_info_map_t;

extern "C" {
#else
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

const char *libcamera_control_name(enum libcamera_control_id id);
enum libcamera_control_type libcamera_control_type(enum libcamera_control_id id);

const char *libcamera_property_name(enum libcamera_property_id id);
enum libcamera_control_type libcamera_property_type(enum libcamera_property_id id);

// const libcamera_control_id_t *libcamera_control_id(unsigned int id);
// const libcamera_control_id_t *libcamera_property_id(unsigned int id);

// unsigned int libcamera_control_id_id(const libcamera_control_id_t *ctrl_id);
// const char *libcamera_control_id_name(const libcamera_control_id_t *ctrl_id);
// unsigned int libcamera_control_id_type(const libcamera_control_id_t *ctrl_id);
// libcamera_control_id_t *libcamera_control_id_copy(const libcamera_control_id_t *ctrl_id);

#ifdef __cplusplus
}
#endif

#endif
