enum libcamera_control_id {
    /**
     * \brief Enable or disable the AE.
     * 
     * \sa ExposureTime AnalogueGain
     */
    LIBCAMERA_CONTROL_ID_AE_ENABLE = 1,
    /**
     * \brief Report the lock status of a running AE algorithm.
     * 
     * If the AE algorithm is locked the value shall be set to true, if it's
     * converging it shall be set to false. If the AE algorithm is not
     * running the control shall not be present in the metadata control list.
     * 
     * \sa AeEnable
     */
    LIBCAMERA_CONTROL_ID_AE_LOCKED = 2,
    /**
     * \brief Specify a metering mode for the AE algorithm to use. The metering
     * modes determine which parts of the image are used to determine the
     * scene brightness. Metering modes may be platform specific and not
     * all metering modes may be supported.
     */
    LIBCAMERA_CONTROL_ID_AE_METERING_MODE = 3,
    /**
     * \brief Specify a constraint mode for the AE algorithm to use. These determine
     * how the measured scene brightness is adjusted to reach the desired
     * target exposure. Constraint modes may be platform specific, and not
     * all constraint modes may be supported.
     */
    LIBCAMERA_CONTROL_ID_AE_CONSTRAINT_MODE = 4,
    /**
     * \brief Specify an exposure mode for the AE algorithm to use. These specify
     * how the desired total exposure is divided between the shutter time
     * and the sensor's analogue gain. The exposure modes are platform
     * specific, and not all exposure modes may be supported.
     */
    LIBCAMERA_CONTROL_ID_AE_EXPOSURE_MODE = 5,
    /**
     * \brief Specify an Exposure Value (EV) parameter. The EV parameter will only be
     * applied if the AE algorithm is currently enabled.
     * 
     * By convention EV adjusts the exposure as log2. For example
     * EV = [-2, -1, 0.5, 0, 0.5, 1, 2] results in an exposure adjustment
     * of [1/4x, 1/2x, 1/sqrt(2)x, 1x, sqrt(2)x, 2x, 4x].
     * 
     * \sa AeEnable
     */
    LIBCAMERA_CONTROL_ID_EXPOSURE_VALUE = 6,
    /**
     * \brief Exposure time (shutter speed) for the frame applied in the sensor
     * device. This value is specified in micro-seconds.
     * 
     * Setting this value means that it is now fixed and the AE algorithm may
     * not change it. Setting it back to zero returns it to the control of the
     * AE algorithm.
     * 
     * \sa AnalogueGain AeEnable
     * 
     * \todo Document the interactions between AeEnable and setting a fixed
     * value for this control. Consider interactions with other AE features,
     * such as aperture and aperture/shutter priority mode, and decide if
     * control of which features should be automatically adjusted shouldn't
     * better be handled through a separate AE mode control.
     */
    LIBCAMERA_CONTROL_ID_EXPOSURE_TIME = 7,
    /**
     * \brief Analogue gain value applied in the sensor device.
     * The value of the control specifies the gain multiplier applied to all
     * colour channels. This value cannot be lower than 1.0.
     * 
     * Setting this value means that it is now fixed and the AE algorithm may
     * not change it. Setting it back to zero returns it to the control of the
     * AE algorithm.
     * 
     * \sa ExposureTime AeEnable
     * 
     * \todo Document the interactions between AeEnable and setting a fixed
     * value for this control. Consider interactions with other AE features,
     * such as aperture and aperture/shutter priority mode, and decide if
     * control of which features should be automatically adjusted shouldn't
     * better be handled through a separate AE mode control.
     */
    LIBCAMERA_CONTROL_ID_ANALOGUE_GAIN = 8,
    /**
     * \brief Specify a fixed brightness parameter. Positive values (up to 1.0)
     * produce brighter images; negative values (up to -1.0) produce darker
     * images and 0.0 leaves pixels unchanged.
     */
    LIBCAMERA_CONTROL_ID_BRIGHTNESS = 9,
    /**
     * \brief Specify a fixed contrast parameter. Normal contrast is given by the
     * value 1.0; larger values produce images with more contrast.
     */
    LIBCAMERA_CONTROL_ID_CONTRAST = 10,
    /**
     * \brief Report an estimate of the current illuminance level in lux. The Lux
     * control can only be returned in metadata.
     */
    LIBCAMERA_CONTROL_ID_LUX = 11,
    /**
     * \brief Enable or disable the AWB.
     * 
     * \sa ColourGains
     */
    LIBCAMERA_CONTROL_ID_AWB_ENABLE = 12,
    /**
     * \brief Specify the range of illuminants to use for the AWB algorithm. The modes
     * supported are platform specific, and not all modes may be supported.
     */
    LIBCAMERA_CONTROL_ID_AWB_MODE = 13,
    /**
     * \brief Report the lock status of a running AWB algorithm.
     * 
     * If the AWB algorithm is locked the value shall be set to true, if it's
     * converging it shall be set to false. If the AWB algorithm is not
     * running the control shall not be present in the metadata control list.
     * 
     * \sa AwbEnable
     */
    LIBCAMERA_CONTROL_ID_AWB_LOCKED = 14,
    /**
     * \brief Pair of gain values for the Red and Blue colour channels, in that
     * order. ColourGains can only be applied in a Request when the AWB is
     * disabled.
     * 
     * \sa AwbEnable
     */
    LIBCAMERA_CONTROL_ID_COLOUR_GAINS = 15,
    /**
     * \brief Report the current estimate of the colour temperature, in kelvin, for this frame. The ColourTemperature control can only be returned in metadata.
     */
    LIBCAMERA_CONTROL_ID_COLOUR_TEMPERATURE = 16,
    /**
     * \brief Specify a fixed saturation parameter. Normal saturation is given by
     * the value 1.0; larger values produce more saturated colours; 0.0
     * produces a greyscale image.
     */
    LIBCAMERA_CONTROL_ID_SATURATION = 17,
    /**
     * \brief Reports the sensor black levels used for processing a frame, in the
     * order R, Gr, Gb, B. These values are returned as numbers out of a 16-bit
     * pixel range (as if pixels ranged from 0 to 65535). The SensorBlackLevels
     * control can only be returned in metadata.
     */
    LIBCAMERA_CONTROL_ID_SENSOR_BLACK_LEVELS = 18,
    /**
     * \brief A value of 0.0 means no sharpening. The minimum value means
     * minimal sharpening, and shall be 0.0 unless the camera can't
     * disable sharpening completely. The default value shall give a
     * "reasonable" level of sharpening, suitable for most use cases.
     * The maximum value may apply extremely high levels of sharpening,
     * higher than anyone could reasonably want. Negative values are
     * not allowed. Note also that sharpening is not applied to raw
     * streams.
     */
    LIBCAMERA_CONTROL_ID_SHARPNESS = 19,
    /**
     * \brief Reports a Figure of Merit (FoM) to indicate how in-focus the frame is.
     * A larger FocusFoM value indicates a more in-focus frame. This control
     * depends on the IPA to gather ISP statistics from the defined focus
     * region, and combine them in a suitable way to generate a FocusFoM value.
     * In this respect, it is not necessarily aimed at providing a way to
     * implement a focus algorithm by the application, rather an indication of
     * how in-focus a frame is.
     */
    LIBCAMERA_CONTROL_ID_FOCUS_FO_M = 20,
    /**
     * \brief The 3x3 matrix that converts camera RGB to sRGB within the
     * imaging pipeline. This should describe the matrix that is used
     * after pixels have been white-balanced, but before any gamma
     * transformation. The 3x3 matrix is stored in conventional reading
     * order in an array of 9 floating point values.
     */
    LIBCAMERA_CONTROL_ID_COLOUR_CORRECTION_MATRIX = 21,
    /**
     * \brief Sets the image portion that will be scaled to form the whole of
     * the final output image. The (x,y) location of this rectangle is
     * relative to the PixelArrayActiveAreas that is being used. The units
     * remain native sensor pixels, even if the sensor is being used in
     * a binning or skipping mode.
     * 
     * This control is only present when the pipeline supports scaling. Its
     * maximum valid value is given by the properties::ScalerCropMaximum
     * property, and the two can be used to implement digital zoom.
     */
    LIBCAMERA_CONTROL_ID_SCALER_CROP = 22,
    /**
     * \brief Digital gain value applied during the processing steps applied
     * to the image as captured from the sensor.
     * 
     * The global digital gain factor is applied to all the colour channels
     * of the RAW image. Different pipeline models are free to
     * specify how the global gain factor applies to each separate
     * channel.
     * 
     * If an imaging pipeline applies digital gain in distinct
     * processing steps, this value indicates their total sum.
     * Pipelines are free to decide how to adjust each processing
     * step to respect the received gain factor and shall report
     * their total value in the request metadata.
     */
    LIBCAMERA_CONTROL_ID_DIGITAL_GAIN = 23,
    /**
     * \brief The instantaneous frame duration from start of frame exposure to start
     * of next exposure, expressed in microseconds. This control is meant to
     * be returned in metadata.
     */
    LIBCAMERA_CONTROL_ID_FRAME_DURATION = 24,
    /**
     * \brief The minimum and maximum (in that order) frame duration,
     * expressed in microseconds.
     * 
     * When provided by applications, the control specifies the sensor frame
     * duration interval the pipeline has to use. This limits the largest
     * exposure time the sensor can use. For example, if a maximum frame
     * duration of 33ms is requested (corresponding to 30 frames per second),
     * the sensor will not be able to raise the exposure time above 33ms.
     * A fixed frame duration is achieved by setting the minimum and maximum
     * values to be the same. Setting both values to 0 reverts to using the
     * IPA provided defaults.
     * 
     * The maximum frame duration provides the absolute limit to the shutter
     * speed computed by the AE algorithm and it overrides any exposure mode
     * setting specified with controls::AeExposureMode. Similarly, when a
     * manual exposure time is set through controls::ExposureTime, it also
     * gets clipped to the limits set by this control. When reported in
     * metadata, the control expresses the minimum and maximum frame
     * durations used after being clipped to the sensor provided frame
     * duration limits.
     * 
     * \sa AeExposureMode
     * \sa ExposureTime
     * 
     * \todo Define how to calculate the capture frame rate by
     * defining controls to report additional delays introduced by
     * the capture pipeline or post-processing stages (ie JPEG
     * conversion, frame scaling).
     * 
     * \todo Provide an explicit definition of default control values, for
     * this and all other controls.
     */
    LIBCAMERA_CONTROL_ID_FRAME_DURATION_LIMITS = 25,
    /**
     * \brief Temperature measure from the camera sensor in Celsius. This is typically
     * obtained by a thermal sensor present on-die or in the camera module. The
     * range of reported temperatures is device dependent.
     * 
     * The SensorTemperature control will only be returned in metadata if a
     * themal sensor is present.
     */
    LIBCAMERA_CONTROL_ID_SENSOR_TEMPERATURE = 26,
    /**
     * \brief The time when the first row of the image sensor active array is exposed.
     * 
     * The timestamp, expressed in nanoseconds, represents a monotonically
     * increasing counter since the system boot time, as defined by the
     * Linux-specific CLOCK_BOOTTIME clock id.
     * 
     * The SensorTimestamp control can only be returned in metadata.
     * 
     * \todo Define how the sensor timestamp has to be used in the reprocessing
     * use case.
     */
    LIBCAMERA_CONTROL_ID_SENSOR_TIMESTAMP = 27,
    /**
     * \brief Control to set the mode of the AF (autofocus) algorithm.
     * 
     * An implementation may choose not to implement all the modes.
     */
    LIBCAMERA_CONTROL_ID_AF_MODE = 28,
    /**
     * \brief Control to set the range of focus distances that is scanned. An
     * implementation may choose not to implement all the options here.
     */
    LIBCAMERA_CONTROL_ID_AF_RANGE = 29,
    /**
     * \brief Control that determines whether the AF algorithm is to move the lens
     * as quickly as possible or more steadily. For example, during video
     * recording it may be desirable not to move the lens too abruptly, but
     * when in a preview mode (waiting for a still capture) it may be
     * helpful to move the lens as quickly as is reasonably possible.
     */
    LIBCAMERA_CONTROL_ID_AF_SPEED = 30,
    /**
     * \brief Instruct the AF algorithm how it should decide which parts of the image
     * should be used to measure focus.
     */
    LIBCAMERA_CONTROL_ID_AF_METERING = 31,
    /**
     * \brief Sets the focus windows used by the AF algorithm when AfMetering is set
     * to AfMeteringWindows. The units used are pixels within the rectangle
     * returned by the ScalerCropMaximum property.
     * 
     * In order to be activated, a rectangle must be programmed with non-zero
     * width and height. Internally, these rectangles are intersected with the
     * ScalerCropMaximum rectangle. If the window becomes empty after this
     * operation, then the window is ignored. If all the windows end up being
     * ignored, then the behaviour is platform dependent.
     * 
     * On platforms that support the ScalerCrop control (for implementing
     * digital zoom, for example), no automatic recalculation or adjustment of
     * AF windows is performed internally if the ScalerCrop is changed. If any
     * window lies outside the output image after the scaler crop has been
     * applied, it is up to the application to recalculate them.
     * 
     * The details of how the windows are used are platform dependent. We note
     * that when there is more than one AF window, a typical implementation
     * might find the optimal focus position for each one and finally select
     * the window where the focal distance for the objects shown in that part
     * of the image are closest to the camera.
     */
    LIBCAMERA_CONTROL_ID_AF_WINDOWS = 32,
    /**
     * \brief This control starts an autofocus scan when AfMode is set to AfModeAuto,
     * and can also be used to terminate a scan early.
     * 
     * It is ignored if AfMode is set to AfModeManual or AfModeContinuous.
     */
    LIBCAMERA_CONTROL_ID_AF_TRIGGER = 33,
    /**
     * \brief This control has no effect except when in continuous autofocus mode
     * (AfModeContinuous). It can be used to pause any lens movements while
     * (for example) images are captured. The algorithm remains inactive
     * until it is instructed to resume.
     */
    LIBCAMERA_CONTROL_ID_AF_PAUSE = 34,
    /**
     * \brief Acts as a control to instruct the lens to move to a particular position
     * and also reports back the position of the lens for each frame.
     * 
     * The LensPosition control is ignored unless the AfMode is set to
     * AfModeManual, though the value is reported back unconditionally in all
     * modes.
     * 
     * The units are a reciprocal distance scale like dioptres but normalised
     * for the hyperfocal distance. That is, for a lens with hyperfocal
     * distance H, and setting it to a focal distance D, the lens position LP,
     * which is generally a non-integer, is given by
     * 
     * \f$LP = \frac{H}{D}\f$
     * 
     * For example:
     * 
     * 0 moves the lens to infinity.
     * 0.5 moves the lens to twice the hyperfocal distance.
     * 1 moves the lens to the hyperfocal position.
     * And larger values will focus the lens ever closer.
     * 
     * \todo Define a property to report the Hyperforcal distance of calibrated
     * lenses.
     * 
     * \todo Define a property to report the maximum and minimum positions of
     * this lens. The minimum value will often be zero (meaning infinity).
     */
    LIBCAMERA_CONTROL_ID_LENS_POSITION = 35,
    /**
     * \brief Reports the current state of the AF algorithm in conjunction with the
     * reported AfMode value and (in continuous AF mode) the AfPauseState
     * value. The possible state changes are described below, though we note
     * the following state transitions that occur when the AfMode is changed.
     * 
     * If the AfMode is set to AfModeManual, then the AfState will always
     * report AfStateIdle (even if the lens is subsequently moved). Changing to
     * the AfModeManual state does not initiate any lens movement.
     * 
     * If the AfMode is set to AfModeAuto then the AfState will report
     * AfStateIdle. However, if AfModeAuto and AfTriggerStart are sent together
     * then AfState will omit AfStateIdle and move straight to AfStateScanning
     * (and start a scan).
     * 
     * If the AfMode is set to AfModeContinuous then the AfState will initially
     * report AfStateScanning.
     */
    LIBCAMERA_CONTROL_ID_AF_STATE = 36,
    /**
     * \brief Only applicable in continuous (AfModeContinuous) mode, this reports
     * whether the algorithm is currently running, paused or pausing (that is,
     * will pause as soon as any in-progress scan completes).
     * 
     * Any change to AfMode will cause AfPauseStateRunning to be reported.
     */
    LIBCAMERA_CONTROL_ID_AF_PAUSE_STATE = 37,
    /**
     * \brief Control for AE metering trigger. Currently identical to
     * ANDROID_CONTROL_AE_PRECAPTURE_TRIGGER.
     * 
     * Whether the camera device will trigger a precapture metering sequence
     * when it processes this request.
     */
    LIBCAMERA_CONTROL_ID_AE_PRECAPTURE_TRIGGER = 38,
    /**
     * \brief Control to select the noise reduction algorithm mode. Currently
     * identical to ANDROID_NOISE_REDUCTION_MODE.
     * 
     *  Mode of operation for the noise reduction algorithm.
     */
    LIBCAMERA_CONTROL_ID_NOISE_REDUCTION_MODE = 39,
    /**
     * \brief Control to select the color correction aberration mode. Currently
     * identical to ANDROID_COLOR_CORRECTION_ABERRATION_MODE.
     * 
     *  Mode of operation for the chromatic aberration correction algorithm.
     */
    LIBCAMERA_CONTROL_ID_COLOR_CORRECTION_ABERRATION_MODE = 40,
    /**
     * \brief Control to report the current AE algorithm state. Currently identical to
     * ANDROID_CONTROL_AE_STATE.
     * 
     *  Current state of the AE algorithm.
     */
    LIBCAMERA_CONTROL_ID_AE_STATE = 41,
    /**
     * \brief Control to report the current AWB algorithm state. Currently identical
     * to ANDROID_CONTROL_AWB_STATE.
     * 
     *  Current state of the AWB algorithm.
     */
    LIBCAMERA_CONTROL_ID_AWB_STATE = 42,
    /**
     * \brief Control to report the time between the start of exposure of the first
     * row and the start of exposure of the last row. Currently identical to
     * ANDROID_SENSOR_ROLLING_SHUTTER_SKEW
     */
    LIBCAMERA_CONTROL_ID_SENSOR_ROLLING_SHUTTER_SKEW = 43,
    /**
     * \brief Control to report if the lens shading map is available. Currently
     * identical to ANDROID_STATISTICS_LENS_SHADING_MAP_MODE.
     */
    LIBCAMERA_CONTROL_ID_LENS_SHADING_MAP_MODE = 44,
    /**
     * \brief Control to report the detected scene light frequency. Currently
     * identical to ANDROID_STATISTICS_SCENE_FLICKER.
     */
    LIBCAMERA_CONTROL_ID_SCENE_FLICKER = 45,
    /**
     * \brief Specifies the number of pipeline stages the frame went through from when
     * it was exposed to when the final completed result was available to the
     * framework. Always less than or equal to PipelineMaxDepth. Currently
     * identical to ANDROID_REQUEST_PIPELINE_DEPTH.
     * 
     * The typical value for this control is 3 as a frame is first exposed,
     * captured and then processed in a single pass through the ISP. Any
     * additional processing step performed after the ISP pass (in example face
     * detection, additional format conversions etc) count as an additional
     * pipeline stage.
     */
    LIBCAMERA_CONTROL_ID_PIPELINE_DEPTH = 46,
    /**
     * \brief The maximum number of frames that can occur after a request (different
     * than the previous) has been submitted, and before the result's state
     * becomes synchronized. A value of -1 indicates unknown latency, and 0
     * indicates per-frame control. Currently identical to
     * ANDROID_SYNC_MAX_LATENCY.
     */
    LIBCAMERA_CONTROL_ID_MAX_LATENCY = 47,
    /**
     * \brief Control to select the test pattern mode. Currently identical to
     * ANDROID_SENSOR_TEST_PATTERN_MODE.
     */
    LIBCAMERA_CONTROL_ID_TEST_PATTERN_MODE = 48,
};

/**
 * \brief Supported values for LIBCAMERA_CONTROL_ID_AE_METERING_MODE control
 */
enum libcamera_ae_metering_mode {
    /**
     * \brief Centre-weighted metering mode.
     */
    LIBCAMERA_METERING_CENTRE_WEIGHTED = 0,
    /**
     * \brief Spot metering mode.
     */
    LIBCAMERA_METERING_SPOT = 1,
    /**
     * \brief Matrix metering mode.
     */
    LIBCAMERA_METERING_MATRIX = 2,
    /**
     * \brief Custom metering mode.
     */
    LIBCAMERA_METERING_CUSTOM = 3,
};

/**
 * \brief Supported values for LIBCAMERA_CONTROL_ID_AE_CONSTRAINT_MODE control
 */
enum libcamera_ae_constraint_mode {
    /**
     * \brief Default constraint mode. This mode aims to balance the exposure of different parts of the image so as to reach a reasonable average level. However, highlights in the image may appear over-exposed and lowlights may appear under-exposed.
     */
    LIBCAMERA_CONSTRAINT_NORMAL = 0,
    /**
     * \brief Highlight constraint mode. This mode adjusts the exposure levels in order to try and avoid over-exposing the brightest parts (highlights) of an image. Other non-highlight parts of the image may appear under-exposed.
     */
    LIBCAMERA_CONSTRAINT_HIGHLIGHT = 1,
    /**
     * \brief Shadows constraint mode. This mode adjusts the exposure levels in order to try and avoid under-exposing the dark parts (shadows) of an image. Other normally exposed parts of the image may appear over-exposed.
     */
    LIBCAMERA_CONSTRAINT_SHADOWS = 2,
    /**
     * \brief Custom constraint mode.
     */
    LIBCAMERA_CONSTRAINT_CUSTOM = 3,
};

/**
 * \brief Supported values for LIBCAMERA_CONTROL_ID_AE_EXPOSURE_MODE control
 */
enum libcamera_ae_exposure_mode {
    /**
     * \brief Default exposure mode.
     */
    LIBCAMERA_EXPOSURE_NORMAL = 0,
    /**
     * \brief Exposure mode allowing only short exposure times.
     */
    LIBCAMERA_EXPOSURE_SHORT = 1,
    /**
     * \brief Exposure mode allowing long exposure times.
     */
    LIBCAMERA_EXPOSURE_LONG = 2,
    /**
     * \brief Custom exposure mode.
     */
    LIBCAMERA_EXPOSURE_CUSTOM = 3,
};

/**
 * \brief Supported values for LIBCAMERA_CONTROL_ID_AWB_MODE control
 */
enum libcamera_awb_mode {
    /**
     * \brief Search over the whole colour temperature range.
     */
    LIBCAMERA_AWB_AUTO = 0,
    /**
     * \brief Incandescent AWB lamp mode.
     */
    LIBCAMERA_AWB_INCANDESCENT = 1,
    /**
     * \brief Tungsten AWB lamp mode.
     */
    LIBCAMERA_AWB_TUNGSTEN = 2,
    /**
     * \brief Fluorescent AWB lamp mode.
     */
    LIBCAMERA_AWB_FLUORESCENT = 3,
    /**
     * \brief Indoor AWB lighting mode.
     */
    LIBCAMERA_AWB_INDOOR = 4,
    /**
     * \brief Daylight AWB lighting mode.
     */
    LIBCAMERA_AWB_DAYLIGHT = 5,
    /**
     * \brief Cloudy AWB lighting mode.
     */
    LIBCAMERA_AWB_CLOUDY = 6,
    /**
     * \brief Custom AWB mode.
     */
    LIBCAMERA_AWB_CUSTOM = 7,
};

/**
 * \brief Supported values for LIBCAMERA_CONTROL_ID_AF_MODE control
 */
enum libcamera_af_mode {
    /**
     * \brief The AF algorithm is in manual mode. In this mode it will never
     * perform any action nor move the lens of its own accord, but an
     * application can specify the desired lens position using the
     * LensPosition control.
     * 
     * In this mode the AfState will always report AfStateIdle.
     */
    LIBCAMERA_AF_MODE_MANUAL = 0,
    /**
     * \brief The AF algorithm is in auto mode. This means that the algorithm
     * will never move the lens or change state unless the AfTrigger
     * control is used. The AfTrigger control can be used to initiate a
     * focus scan, the results of which will be reported by AfState.
     * 
     * If the autofocus algorithm is moved from AfModeAuto to another
     * mode while a scan is in progress, the scan is cancelled
     * immediately, without waiting for the scan to finish.
     * 
     * When first entering this mode the AfState will report
     * AfStateIdle. When a trigger control is sent, AfState will
     * report AfStateScanning for a period before spontaneously
     * changing to AfStateFocused or AfStateFailed, depending on
     * the outcome of the scan. It will remain in this state until
     * another scan is initiated by the AfTrigger control. If a scan is
     * cancelled (without changing to another mode), AfState will return
     * to AfStateIdle.
     */
    LIBCAMERA_AF_MODE_AUTO = 1,
    /**
     * \brief The AF algorithm is in continuous mode. This means that the lens can
     * re-start a scan spontaneously at any moment, without any user
     * intervention. The AfState still reports whether the algorithm is
     * currently scanning or not, though the application has no ability to
     * initiate or cancel scans, nor to move the lens for itself.
     * 
     * However, applications can pause the AF algorithm from continuously
     * scanning by using the AfPause control. This allows video or still
     * images to be captured whilst guaranteeing that the focus is fixed.
     * 
     * When set to AfModeContinuous, the system will immediately initiate a
     * scan so AfState will report AfStateScanning, and will settle on one
     * of AfStateFocused or AfStateFailed, depending on the scan result.
     */
    LIBCAMERA_AF_MODE_CONTINUOUS = 2,
};

/**
 * \brief Supported values for LIBCAMERA_CONTROL_ID_AF_RANGE control
 */
enum libcamera_af_range {
    /**
     * \brief A wide range of focus distances is scanned, all the way from
     * infinity down to close distances, though depending on the
     * implementation, possibly not including the very closest macro
     * positions.
     */
    LIBCAMERA_AF_RANGE_NORMAL = 0,
    /**
     * \brief Only close distances are scanned.
     */
    LIBCAMERA_AF_RANGE_MACRO = 1,
    /**
     * \brief The full range of focus distances is scanned just as with
     * AfRangeNormal but this time including the very closest macro
     * positions.
     */
    LIBCAMERA_AF_RANGE_FULL = 2,
};

/**
 * \brief Supported values for LIBCAMERA_CONTROL_ID_AF_SPEED control
 */
enum libcamera_af_speed {
    /**
     * \brief Move the lens at its usual speed.
     */
    LIBCAMERA_AF_SPEED_NORMAL = 0,
    /**
     * \brief Move the lens more quickly.
     */
    LIBCAMERA_AF_SPEED_FAST = 1,
};

/**
 * \brief Supported values for LIBCAMERA_CONTROL_ID_AF_METERING control
 */
enum libcamera_af_metering {
    /**
     * \brief The AF algorithm should decide for itself where it will measure focus.
     */
    LIBCAMERA_AF_METERING_AUTO = 0,
    /**
     * \brief The AF algorithm should use the rectangles defined by the AfWindows control to measure focus. If no windows are specified the behaviour is platform dependent.
     */
    LIBCAMERA_AF_METERING_WINDOWS = 1,
};

/**
 * \brief Supported values for LIBCAMERA_CONTROL_ID_AF_TRIGGER control
 */
enum libcamera_af_trigger {
    /**
     * \brief Start an AF scan. Ignored if a scan is in progress.
     */
    LIBCAMERA_AF_TRIGGER_START = 0,
    /**
     * \brief Cancel an AF scan. This does not cause the lens to move anywhere else. Ignored if no scan is in progress.
     */
    LIBCAMERA_AF_TRIGGER_CANCEL = 1,
};

/**
 * \brief Supported values for LIBCAMERA_CONTROL_ID_AF_PAUSE control
 */
enum libcamera_af_pause {
    /**
     * \brief Pause the continuous autofocus algorithm immediately, whether or not
     * any kind of scan is underway. AfPauseState will subsequently report
     * AfPauseStatePaused. AfState may report any of AfStateScanning,
     * AfStateFocused or AfStateFailed, depending on the algorithm's state
     * when it received this control.
     */
    LIBCAMERA_AF_PAUSE_IMMEDIATE = 0,
    /**
     * \brief This is similar to AfPauseImmediate, and if the AfState is currently
     * reporting AfStateFocused or AfStateFailed it will remain in that
     * state and AfPauseState will report AfPauseStatePaused.
     * 
     * However, if the algorithm is scanning (AfStateScanning),
     * AfPauseState will report AfPauseStatePausing until the scan is
     * finished, at which point AfState will report one of AfStateFocused
     * or AfStateFailed, and AfPauseState will change to
     * AfPauseStatePaused.
     */
    LIBCAMERA_AF_PAUSE_DEFERRED = 1,
    /**
     * \brief Resume continuous autofocus operation. The algorithm starts again
     * from exactly where it left off, and AfPauseState will report
     * AfPauseStateRunning.
     */
    LIBCAMERA_AF_PAUSE_RESUME = 2,
};

/**
 * \brief Supported values for LIBCAMERA_CONTROL_ID_AF_STATE control
 */
enum libcamera_af_state {
    /**
     * \brief The AF algorithm is in manual mode (AfModeManual) or in auto mode
     * (AfModeAuto) and a scan has not yet been triggered, or an
     * in-progress scan was cancelled.
     */
    LIBCAMERA_AF_STATE_IDLE = 0,
    /**
     * \brief The AF algorithm is in auto mode (AfModeAuto), and a scan has been
     * started using the AfTrigger control. The scan can be cancelled by
     * sending AfTriggerCancel at which point the algorithm will either
     * move back to AfStateIdle or, if the scan actually completes before
     * the cancel request is processed, to one of AfStateFocused or
     * AfStateFailed.
     * 
     * Alternatively the AF algorithm could be in continuous mode
     * (AfModeContinuous) at which point it may enter this state
     * spontaneously whenever it determines that a rescan is needed.
     */
    LIBCAMERA_AF_STATE_SCANNING = 1,
    /**
     * \brief The AF algorithm is in auto (AfModeAuto) or continuous
     * (AfModeContinuous) mode and a scan has completed with the result
     * that the algorithm believes the image is now in focus.
     */
    LIBCAMERA_AF_STATE_FOCUSED = 2,
    /**
     * \brief The AF algorithm is in auto (AfModeAuto) or continuous
     * (AfModeContinuous) mode and a scan has completed with the result
     * that the algorithm did not find a good focus position.
     */
    LIBCAMERA_AF_STATE_FAILED = 3,
};

/**
 * \brief Supported values for LIBCAMERA_CONTROL_ID_AF_PAUSE_STATE control
 */
enum libcamera_af_pause_state {
    /**
     * \brief Continuous AF is running and the algorithm may restart a scan
     * spontaneously.
     */
    LIBCAMERA_AF_PAUSE_STATE_RUNNING = 0,
    /**
     * \brief Continuous AF has been sent an AfPauseDeferred control, and will
     * pause as soon as any in-progress scan completes (and then report
     * AfPauseStatePaused). No new scans will be start spontaneously until
     * the AfPauseResume control is sent.
     */
    LIBCAMERA_AF_PAUSE_STATE_PAUSING = 1,
    /**
     * \brief Continuous AF is paused. No further state changes or lens movements
     * will occur until the AfPauseResume control is sent.
     */
    LIBCAMERA_AF_PAUSE_STATE_PAUSED = 2,
};

/**
 * \brief Supported values for LIBCAMERA_CONTROL_ID_AE_PRECAPTURE_TRIGGER control
 */
enum libcamera_ae_precapture_trigger {
    /**
     * \brief The trigger is idle.
     */
    LIBCAMERA_AE_PRECAPTURE_TRIGGER_IDLE = 0,
    /**
     * \brief The pre-capture AE metering is started by the camera.
     */
    LIBCAMERA_AE_PRECAPTURE_TRIGGER_START = 1,
    /**
     * \brief The camera will cancel any active or completed metering sequence.
     * The AE algorithm is reset to its initial state.
     */
    LIBCAMERA_AE_PRECAPTURE_TRIGGER_CANCEL = 2,
};

/**
 * \brief Supported values for LIBCAMERA_CONTROL_ID_NOISE_REDUCTION_MODE control
 */
enum libcamera_noise_reduction_mode {
    /**
     * \brief No noise reduction is applied
     */
    LIBCAMERA_NOISE_REDUCTION_MODE_OFF = 0,
    /**
     * \brief Noise reduction is applied without reducing the frame rate.
     */
    LIBCAMERA_NOISE_REDUCTION_MODE_FAST = 1,
    /**
     * \brief High quality noise reduction at the expense of frame rate.
     */
    LIBCAMERA_NOISE_REDUCTION_MODE_HIGH_QUALITY = 2,
    /**
     * \brief Minimal noise reduction is applied without reducing the frame rate.
     */
    LIBCAMERA_NOISE_REDUCTION_MODE_MINIMAL = 3,
    /**
     * \brief Noise reduction is applied at different levels to different streams.
     */
    LIBCAMERA_NOISE_REDUCTION_MODE_ZSL = 4,
};

/**
 * \brief Supported values for LIBCAMERA_CONTROL_ID_COLOR_CORRECTION_ABERRATION_MODE control
 */
enum libcamera_color_correction_aberration_mode {
    /**
     * \brief No aberration correction is applied.
     */
    LIBCAMERA_COLOR_CORRECTION_ABERRATION_OFF = 0,
    /**
     * \brief Aberration correction will not slow down the frame rate.
     */
    LIBCAMERA_COLOR_CORRECTION_ABERRATION_FAST = 1,
    /**
     * \brief High quality aberration correction which might reduce the frame
     * rate.
     */
    LIBCAMERA_COLOR_CORRECTION_ABERRATION_HIGH_QUALITY = 2,
};

/**
 * \brief Supported values for LIBCAMERA_CONTROL_ID_AE_STATE control
 */
enum libcamera_ae_state {
    /**
     * \brief The AE algorithm is inactive.
     */
    LIBCAMERA_AE_STATE_INACTIVE = 0,
    /**
     * \brief The AE algorithm has not converged yet.
     */
    LIBCAMERA_AE_STATE_SEARCHING = 1,
    /**
     * \brief The AE algorithm has converged.
     */
    LIBCAMERA_AE_STATE_CONVERGED = 2,
    /**
     * \brief The AE algorithm is locked.
     */
    LIBCAMERA_AE_STATE_LOCKED = 3,
    /**
     * \brief The AE algorithm would need a flash for good results
     */
    LIBCAMERA_AE_STATE_FLASH_REQUIRED = 4,
    /**
     * \brief The AE algorithm has started a pre-capture metering session.
     * \sa AePrecaptureTrigger
     */
    LIBCAMERA_AE_STATE_PRECAPTURE = 5,
};

/**
 * \brief Supported values for LIBCAMERA_CONTROL_ID_AWB_STATE control
 */
enum libcamera_awb_state {
    /**
     * \brief The AWB algorithm is inactive.
     */
    LIBCAMERA_AWB_STATE_INACTIVE = 0,
    /**
     * \brief The AWB algorithm has not converged yet.
     */
    LIBCAMERA_AWB_STATE_SEARCHING = 1,
    /**
     * \brief The AWB algorithm has converged.
     */
    LIBCAMERA_AWB_CONVERGED = 2,
    /**
     * \brief The AWB algorithm is locked.
     */
    LIBCAMERA_AWB_LOCKED = 3,
};

/**
 * \brief Supported values for LIBCAMERA_CONTROL_ID_LENS_SHADING_MAP_MODE control
 */
enum libcamera_lens_shading_map_mode {
    /**
     * \brief No lens shading map mode is available.
     */
    LIBCAMERA_LENS_SHADING_MAP_MODE_OFF = 0,
    /**
     * \brief The lens shading map mode is available.
     */
    LIBCAMERA_LENS_SHADING_MAP_MODE_ON = 1,
};

/**
 * \brief Supported values for LIBCAMERA_CONTROL_ID_SCENE_FLICKER control
 */
enum libcamera_scene_flicker {
    /**
     * \brief No flickering detected.
     */
    LIBCAMERA_SCENE_FICKER_OFF = 0,
    /**
     * \brief 50Hz flickering detected.
     */
    LIBCAMERA_SCENE_FICKER_50HZ = 1,
    /**
     * \brief 60Hz flickering detected.
     */
    LIBCAMERA_SCENE_FICKER_60HZ = 2,
};

/**
 * \brief Supported values for LIBCAMERA_CONTROL_ID_TEST_PATTERN_MODE control
 */
enum libcamera_test_pattern_mode {
    /**
     * \brief No test pattern mode is used. The camera device returns frames from
     * the image sensor.
     */
    LIBCAMERA_TEST_PATTERN_MODE_OFF = 0,
    /**
     * \brief Each pixel in [R, G_even, G_odd, B] is replaced by its respective
     * color channel provided in test pattern data.
     * \todo Add control for test pattern data.
     */
    LIBCAMERA_TEST_PATTERN_MODE_SOLID_COLOR = 1,
    /**
     * \brief All pixel data is replaced with an 8-bar color pattern. The vertical
     * bars (left-to-right) are as follows; white, yellow, cyan, green,
     * magenta, red, blue and black. Each bar should take up 1/8 of the
     * sensor pixel array width. When this is not possible, the bar size
     * should be rounded down to the nearest integer and the pattern can
     * repeat on the right side. Each bar's height must always take up the
     * full sensor pixel array height.
     */
    LIBCAMERA_TEST_PATTERN_MODE_COLOR_BARS = 2,
    /**
     * \brief The test pattern is similar to TestPatternModeColorBars,
     * except that each bar should start at its specified color at the top
     * and fade to gray at the bottom. Furthermore each bar is further
     * subdevided into a left and right half. The left half should have a
     * smooth gradient, and the right half should have a quantized
     * gradient. In particular, the right half's should consist of blocks
     * of the same color for 1/16th active sensor pixel array width. The
     * least significant bits in the quantized gradient should be copied
     * from the most significant bits of the smooth gradient. The height of
     * each bar should always be a multiple of 128. When this is not the
     * case, the pattern should repeat at the bottom of the image.
     */
    LIBCAMERA_TEST_PATTERN_MODE_COLOR_BARS_FADE_TO_GRAY = 3,
    /**
     * \brief All pixel data is replaced by a pseudo-random sequence generated
     * from a PN9 512-bit sequence (typically implemented in hardware with
     * a linear feedback shift register). The generator should be reset at
     * the beginning of each frame, and thus each subsequent raw frame with
     * this test pattern should be exactly the same as the last.
     */
    LIBCAMERA_TEST_PATTERN_MODE_PN_9 = 4,
    /**
     * \brief The first custom test pattern. All custom patterns that are
     * available only on this camera device are at least this numeric
     * value. All of the custom test patterns will be static (that is the
     * raw image must not vary from frame to frame).
     */
    LIBCAMERA_TEST_PATTERN_MODE_CUSTOM_1 = 256,
};

enum libcamera_control_id {
    /**
     * \brief Camera mounting location
     */
    LIBCAMERA_CONTROL_ID_LOCATION = 1,
    /**
     * \brief The camera rotation is expressed as the angular difference in degrees
     * between two reference systems, one relative to the camera module, and
     * one defined on the external world scene to be captured when projected
     * on the image sensor pixel array.
     * 
     * A camera sensor has a 2-dimensional reference system 'Rc' defined by
     * its pixel array read-out order. The origin is set to the first pixel
     * being read out, the X-axis points along the column read-out direction
     * towards the last columns, and the Y-axis along the row read-out
     * direction towards the last row.
     * 
     * A typical example for a sensor with a 2592x1944 pixel array matrix
     * observed from the front is
     * 
     *             2591       X-axis          0
     *               <------------------------+ 0
     *               .......... ... ..........!
     *               .......... ... ..........! Y-axis
     *                          ...           !
     *               .......... ... ..........!
     *               .......... ... ..........! 1943
     *                                        V
     * 
     * 
     * The external world scene reference system 'Rs' is a 2-dimensional
     * reference system on the focal plane of the camera module. The origin is
     * placed on the top-left corner of the visible scene, the X-axis points
     * towards the right, and the Y-axis points towards the bottom of the
     * scene. The top, bottom, left and right directions are intentionally not
     * defined and depend on the environment in which the camera is used.
     * 
     * A typical example of a (very common) picture of a shark swimming from
     * left to right, as seen from the camera, is
     * 
     *              0               X-axis
     *            0 +------------------------------------->
     *              !
     *              !
     *              !
     *              !           |\____)\___
     *              !           ) _____  __`<
     *              !           |/     )/
     *              !
     *              !
     *              !
     *              V
     *            Y-axis
     * 
     * With the reference system 'Rs' placed on the camera focal plane.
     * 
     *                                 ¸.·˙!
     *                             ¸.·˙    !
     *                 _       ¸.·˙        !
     *              +-/ \-+¸.·˙            !
     *              | (o) |                ! Camera focal plane
     *              +-----+˙·.¸            !
     *                         ˙·.¸        !
     *                             ˙·.¸    !
     *                                 ˙·.¸!
     * 
     * When projected on the sensor's pixel array, the image and the associated
     * reference system 'Rs' are typically (but not always) inverted, due to
     * the camera module's lens optical inversion effect.
     * 
     * Assuming the above represented scene of the swimming shark, the lens
     * inversion projects the scene and its reference system onto the sensor
     * pixel array, seen from the front of the camera sensor, as follow
     * 
     *           Y-axis
     *              ^
     *              !
     *              !
     *              !
     *              !            |\_____)\__
     *              !            ) ____  ___.<
     *              !            |/    )/
     *              !
     *              !
     *              !
     *            0 +------------------------------------->
     *              0               X-axis
     * 
     * Note the shark being upside-down.
     * 
     * The resulting projected reference system is named 'Rp'.
     * 
     * The camera rotation property is then defined as the angular difference
     * in the counter-clockwise direction between the camera reference system
     * 'Rc' and the projected scene reference system 'Rp'. It is expressed in
     * degrees as a number in the range [0, 360[.
     * 
     * Examples
     * 
     * 0 degrees camera rotation
     * 
     * 
     *                   Y-Rp
     *                    ^
     *             Y-Rc   !
     *              ^     !
     *              !     !
     *              !     !
     *              !     !
     *              !     !
     *              !     !
     *              !     !
     *              !     !
     *              !   0 +------------------------------------->
     *              !     0               X-Rp
     *            0 +------------------------------------->
     *              0               X-Rc
     * 
     * 
     *                               X-Rc                0
     *              <------------------------------------+ 0
     *                          X-Rp                 0   !
     *          <------------------------------------+ 0 !
     *                                               !   !
     *                                               !   !
     *                                               !   !
     *                                               !   !
     *                                               !   !
     *                                               !   !
     *                                               !   !
     *                                               !   V
     *                                               !  Y-Rc
     *                                               V
     *                                              Y-Rp
     * 
     * 90 degrees camera rotation
     * 
     *              0        Y-Rc
     *            0 +-------------------->
     *              !   Y-Rp
     *              !    ^
     *              !    !
     *              !    !
     *              !    !
     *              !    !
     *              !    !
     *              !    !
     *              !    !
     *              !    !
     *              !    !
     *              !  0 +------------------------------------->
     *              !    0              X-Rp
     *              !
     *              !
     *              !
     *              !
     *              V
     *             X-Rc
     * 
     * 180 degrees camera rotation
     * 
     *                                           0
     *      <------------------------------------+ 0
     *                       X-Rc                !
     *             Y-Rp                          !
     *              ^                            !
     *              !                            !
     *              !                            !
     *              !                            !
     *              !                            !
     *              !                            !
     *              !                            !
     *              !                            V
     *              !                           Y-Rc
     *            0 +------------------------------------->
     *              0              X-Rp
     * 
     * 270 degrees camera rotation
     * 
     *              0        Y-Rc
     *            0 +-------------------->
     *              !                                        0
     *              !    <-----------------------------------+ 0
     *              !                    X-Rp                !
     *              !                                        !
     *              !                                        !
     *              !                                        !
     *              !                                        !
     *              !                                        !
     *              !                                        !
     *              !                                        !
     *              !                                        !
     *              !                                        V
     *              !                                       Y-Rp
     *              !
     *              !
     *              !
     *              !
     *              V
     *             X-Rc
     * 
     * 
     * Example one - Webcam
     * 
     * A camera module installed on the user facing part of a laptop screen
     * casing used for video calls. The captured images are meant to be
     * displayed in landscape mode (width > height) on the laptop screen.
     * 
     * The camera is typically mounted upside-down to compensate the lens
     * optical inversion effect.
     * 
     *                   Y-Rp
     *             Y-Rc   ^
     *              ^     !
     *              !     !
     *              !     !       |\_____)\__
     *              !     !       ) ____  ___.<
     *              !     !       |/    )/
     *              !     !
     *              !     !
     *              !     !
     *              !   0 +------------------------------------->
     *              !     0           X-Rp
     *            0 +------------------------------------->
     *              0            X-Rc
     * 
     * The two reference systems are aligned, the resulting camera rotation is
     * 0 degrees, no rotation correction needs to be applied to the resulting
     * image once captured to memory buffers to correctly display it to users.
     * 
     *              +--------------------------------------+
     *              !                                      !
     *              !                                      !
     *              !                                      !
     *              !             |\____)\___              !
     *              !             ) _____  __`<            !
     *              !             |/     )/                !
     *              !                                      !
     *              !                                      !
     *              !                                      !
     *              +--------------------------------------+
     * 
     * If the camera sensor is not mounted upside-down to compensate for the
     * lens optical inversion, the two reference systems will not be aligned,
     * with 'Rp' being rotated 180 degrees relatively to 'Rc'.
     * 
     * 
     *                       X-Rc                0
     *      <------------------------------------+ 0
     *                                           !
     *             Y-Rp                          !
     *              ^                            !
     *              !                            !
     *              !       |\_____)\__          !
     *              !       ) ____  ___.<        !
     *              !       |/    )/             !
     *              !                            !
     *              !                            !
     *              !                            V
     *              !                           Y-Rc
     *            0 +------------------------------------->
     *              0            X-Rp
     * 
     * The image once captured to memory will then be rotated by 180 degrees
     * 
     *              +--------------------------------------+
     *              !                                      !
     *              !                                      !
     *              !                                      !
     *              !              __/(_____/|             !
     *              !            >.___  ____ (             !
     *              !                 \(    \|             !
     *              !                                      !
     *              !                                      !
     *              !                                      !
     *              +--------------------------------------+
     * 
     * A software rotation correction of 180 degrees should be applied to
     * correctly display the image.
     * 
     *              +--------------------------------------+
     *              !                                      !
     *              !                                      !
     *              !                                      !
     *              !             |\____)\___              !
     *              !             ) _____  __`<            !
     *              !             |/     )/                !
     *              !                                      !
     *              !                                      !
     *              !                                      !
     *              +--------------------------------------+
     * 
     * Example two - Phone camera
     * 
     * A camera installed on the back side of a mobile device facing away from
     * the user. The captured images are meant to be displayed in portrait mode
     * (height > width) to match the device screen orientation and the device
     * usage orientation used when taking the picture.
     * 
     * The camera sensor is typically mounted with its pixel array longer side
     * aligned to the device longer side, upside-down mounted to compensate for
     * the lens optical inversion effect.
     * 
     *              0        Y-Rc
     *            0 +-------------------->
     *              !   Y-Rp
     *              !    ^
     *              !    !
     *              !    !
     *              !    !
     *              !    !            |\_____)\__
     *              !    !            ) ____  ___.<
     *              !    !            |/    )/
     *              !    !
     *              !    !
     *              !    !
     *              !  0 +------------------------------------->
     *              !    0                X-Rp
     *              !
     *              !
     *              !
     *              !
     *              V
     *             X-Rc
     * 
     * The two reference systems are not aligned and the 'Rp' reference
     * system is rotated by 90 degrees in the counter-clockwise direction
     * relatively to the 'Rc' reference system.
     * 
     * The image once captured to memory will be rotated.
     * 
     *              +-------------------------------------+
     *              |                 _ _                 |
     *              |                \   /                |
     *              |                 | |                 |
     *              |                 | |                 |
     *              |                 |  >                |
     *              |                <  |                 |
     *              |                 | |                 |
     *              |                   .                 |
     *              |                  V                  |
     *              +-------------------------------------+
     * 
     * A correction of 90 degrees in counter-clockwise direction has to be
     * applied to correctly display the image in portrait mode on the device
     * screen.
     * 
     *                       +--------------------+
     *                       |                    |
     *                       |                    |
     *                       |                    |
     *                       |                    |
     *                       |                    |
     *                       |                    |
     *                       |   |\____)\___      |
     *                       |   ) _____  __`<    |
     *                       |   |/     )/        |
     *                       |                    |
     *                       |                    |
     *                       |                    |
     *                       |                    |
     *                       |                    |
     *                       +--------------------+
     */
    LIBCAMERA_CONTROL_ID_ROTATION = 2,
    /**
     * \brief The model name shall to the extent possible describe the sensor. For
     * most devices this is the model name of the sensor. While for some
     * devices the sensor model is unavailable as the sensor or the entire
     * camera is part of a larger unit and exposed as a black-box to the
     * system. In such cases the model name of the smallest device that
     * contains the camera sensor shall be used.
     * 
     * The model name is not meant to be a camera name displayed to the
     * end-user, but may be combined with other camera information to create a
     * camera name.
     * 
     * The model name is not guaranteed to be unique in the system nor is
     * it guaranteed to be stable or have any other properties required to make
     * it a good candidate to be used as a permanent identifier of a camera.
     * 
     * The model name shall describe the camera in a human readable format and
     * shall be encoded in ASCII.
     * 
     * Example model names are 'ov5670', 'imx219' or 'Logitech Webcam C930e'.
     */
    LIBCAMERA_CONTROL_ID_MODEL = 3,
    /**
     * \brief The pixel unit cell physical size, in nanometers.
     * 
     * The UnitCellSize properties defines the horizontal and vertical sizes of
     * a single pixel unit, including its active and non-active parts. In
     * other words, it expresses the horizontal and vertical distance between
     * the top-left corners of adjacent pixels.
     * 
     * The property can be used to calculate the physical size of the sensor's
     * pixel array area and for calibration purposes.
     */
    LIBCAMERA_CONTROL_ID_UNIT_CELL_SIZE = 4,
    /**
     * \brief The camera sensor pixel array readable area vertical and horizontal
     * sizes, in pixels.
     * 
     * The PixelArraySize property defines the size in pixel units of the
     * readable part of full pixel array matrix, including optical black
     * pixels used for calibration, pixels which are not considered valid for
     * capture and active pixels containing valid image data.
     * 
     * The property describes the maximum size of the raw data captured by the
     * camera, which might not correspond to the physical size of the sensor
     * pixel array matrix, as some portions of the physical pixel array matrix
     * are not accessible and cannot be transmitted out.
     * 
     * For example, let's consider a pixel array matrix assembled as follows
     * 
     *      +--------------------------------------------------+
     *      |xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx|
     *      |xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx|
     *      |xxDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDxx|
     *      |xxDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDxx|
     *      |xxDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDxx|
     *      |xxDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDxx|
     *      |xxDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDxx|
     *      |xxDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDxx|
     *      ...          ...           ...      ...          ...
     * 
     *      ...          ...           ...      ...          ...
     *      |xxDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDxx|
     *      |xxDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDxx|
     *      |xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx|
     *      |xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx|
     *      +--------------------------------------------------+
     * 
     * starting with two lines of non-readable pixels (x), followed by N lines
     * of readable data (D) surrounded by two columns of non-readable pixels on
     * each side, and ending with two more lines of non-readable pixels. Only
     * the readable portion is transmitted to the receiving side, defining the
     * sizes of the largest possible buffer of raw data that can be presented
     * to applications.
     * 
     *                      PixelArraySize.width
     *        /----------------------------------------------/
     *        +----------------------------------------------+ /
     *        |DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD| |
     *        |DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD| |
     *        |DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD| |
     *        |DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD| |
     *        |DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD| |
     *        |DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD| | PixelArraySize.height
     *        ...        ...           ...      ...        ...
     *        ...        ...           ...      ...        ...
     *        |DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD| |
     *        |DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD| |
     *        +----------------------------------------------+ /
     * 
     * This defines a rectangle whose top-left corner is placed in position (0,
     * 0) and whose vertical and horizontal sizes are defined by this property.
     * All other rectangles that describe portions of the pixel array, such as
     * the optical black pixels rectangles and active pixel areas, are defined
     * relatively to this rectangle.
     * 
     * All the coordinates are expressed relative to the default sensor readout
     * direction, without any transformation (such as horizontal and vertical
     * flipping) applied. When mapping them to the raw pixel buffer,
     * applications shall take any configured transformation into account.
     * 
     * \todo Rename this property to Size once we will have property
     *       categories (i.e. Properties::PixelArray::Size)
     */
    LIBCAMERA_CONTROL_ID_PIXEL_ARRAY_SIZE = 5,
    /**
     * \brief The pixel array region(s) which contain optical black pixels
     * considered valid for calibration purposes.
     * 
     * This property describes the position and size of optical black pixel
     * regions in the raw data buffer as stored in memory, which might differ
     * from their actual physical location in the pixel array matrix.
     * 
     * It is important to note, in fact, that camera sensors might
     * automatically reorder or skip portions of their pixels array matrix when
     * transmitting data to the receiver. For instance, a sensor may merge the
     * top and bottom optical black rectangles into a single rectangle,
     * transmitted at the beginning of the frame.
     * 
     * The pixel array contains several areas with different purposes,
     * interleaved by lines and columns which are said not to be valid for
     * capturing purposes. Invalid lines and columns are defined as invalid as
     * they could be positioned too close to the chip margins or to the optical
     * black shielding placed on top of optical black pixels.
     * 
     *                      PixelArraySize.width
     *        /----------------------------------------------/
     *           x1                                       x2
     *        +--o---------------------------------------o---+ /
     *        |IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII| |
     *        |IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII| |
     *     y1 oIIOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOII| |
     *        |IIOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOII| |
     *        |IIOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOII| |
     *     y2 oIIOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOII| |
     *        |IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII| |
     *        |IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII| |
     *     y3 |IIOOPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPOOII| |
     *        |IIOOPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPOOII| | PixelArraySize.height
     *        |IIOOPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPOOII| |
     *        ...          ...           ...     ...       ...
     *        ...          ...           ...     ...       ...
     *     y4 |IIOOPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPOOII| |
     *        |IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII| |
     *        |IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII| |
     *        +----------------------------------------------+ /
     * 
     * The readable pixel array matrix is composed by
     * 2 invalid lines (I)
     * 4 lines of valid optical black pixels (O)
     * 2 invalid lines (I)
     * n lines of valid pixel data (P)
     * 2 invalid lines (I)
     * 
     * And the position of the optical black pixel rectangles is defined by
     * 
     *     PixelArrayOpticalBlackRectangles = {
     *        { x1, y1, x2 - x1 + 1, y2 - y1 + 1 },
     *        { x1, y3, 2, y4 - y3 + 1 },
     *        { x2, y3, 2, y4 - y3 + 1 },
     *     };
     * 
     * If the camera, when capturing the full pixel array matrix, automatically
     * skips the invalid lines and columns, producing the following data
     * buffer, when captured to memory
     * 
     *                      PixelArraySize.width
     *        /----------------------------------------------/
     *                                                    x1
     *        +--------------------------------------------o-+ /
     *        |OOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO| |
     *        |OOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO| |
     *        |OOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO| |
     *        |OOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO| |
     *     y1 oOOPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPOO| |
     *        |OOPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPOO| |
     *        |OOPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPOO| | PixelArraySize.height
     *        ...       ...          ...       ...         ... |
     *        ...       ...          ...       ...         ... |
     *        |OOPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPOO| |
     *        |OOPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPOO| |
     *        +----------------------------------------------+ /
     * 
     * then the invalid lines and columns should not be reported as part of the
     * PixelArraySize property in first place.
     * 
     * In this case, the position of the black pixel rectangles will be
     * 
     *     PixelArrayOpticalBlackRectangles = {
     *        { 0, 0, y1 + 1, PixelArraySize[0] },
     *        { 0, y1, 2, PixelArraySize[1] - y1 + 1 },
     *        { x1, y1, 2, PixelArraySize[1] - y1 + 1 },
     *     };
     * 
     * \todo Rename this property to Size once we will have property
     *       categories (i.e. Properties::PixelArray::OpticalBlackRectangles)
     */
    LIBCAMERA_CONTROL_ID_PIXEL_ARRAY_OPTICAL_BLACK_RECTANGLES = 6,
    /**
     * \brief The PixelArrayActiveAreas property defines the (possibly multiple and
     * overlapping) portions of the camera sensor readable pixel matrix
     * which are considered valid for image acquisition purposes.
     * 
     * This property describes an arbitrary number of overlapping rectangles,
     * with each rectangle representing the maximum image size that the camera
     * sensor can produce for a particular aspect ratio. They are defined
     * relatively to the PixelArraySize rectangle.
     * 
     * When multiple rectangles are reported, they shall be ordered from the
     * tallest to the shortest.
     * 
     * Example 1
     * A camera sensor which only produces images in the 4:3 image resolution
     * will report a single PixelArrayActiveAreas rectangle, from which all
     * other image formats are obtained by either cropping the field-of-view
     * and/or applying pixel sub-sampling techniques such as pixel skipping or
     * binning.
     * 
     *            PixelArraySize.width
     *             /----------------/
     *               x1          x2
     *     (0,0)-> +-o------------o-+  /
     *          y1 o +------------+ |  |
     *             | |////////////| |  |
     *             | |////////////| |  | PixelArraySize.height
     *             | |////////////| |  |
     *          y2 o +------------+ |  |
     *             +----------------+  /
     * 
     * The property reports a single rectangle
     * 
     *          PixelArrayActiveAreas = (x1, y1, x2 - x1 + 1, y2 - y1 + 1)
     * 
     * Example 2
     * A camera sensor which can produce images in different native
     * resolutions will report several overlapping rectangles, one for each
     * natively supported resolution.
     * 
     *              PixelArraySize.width
     *             /------------------/
     *               x1  x2    x3  x4
     *     (0,0)-> +o---o------o---o+  /
     *          y1 o    +------+    |  |
     *             |    |//////|    |  |
     *          y2 o+---+------+---+|  |
     *             ||///|//////|///||  | PixelArraySize.height
     *          y3 o+---+------+---+|  |
     *             |    |//////|    |  |
     *          y4 o    +------+    |  |
     *             +----+------+----+  /
     * 
     * The property reports two rectangles
     * 
     *         PixelArrayActiveAreas = ((x2, y1, x3 - x2 + 1, y4 - y1 + 1),
     *                                  (x1, y2, x4 - x1 + 1, y3 - y2 + 1))
     * 
     * The first rectangle describes the maximum field-of-view of all image
     * formats in the 4:3 resolutions, while the second one describes the
     * maximum field of view for all image formats in the 16:9 resolutions.
     * 
     * Multiple rectangles shall only be reported when the sensor can't capture
     * the pixels in the corner regions. If all the pixels in the (x1,y1) -
     * (x4,y4) area can be captured, the PixelArrayActiveAreas property shall
     * contains the single rectangle (x1,y1) - (x4,y4).
     * 
     * \todo Rename this property to ActiveAreas once we will have property
     *       categories (i.e. Properties::PixelArray::ActiveAreas)
     */
    LIBCAMERA_CONTROL_ID_PIXEL_ARRAY_ACTIVE_AREAS = 7,
    /**
     * \brief The maximum valid rectangle for the controls::ScalerCrop control. This
     * reflects the minimum mandatory cropping applied in the camera sensor and
     * the rest of the pipeline. Just as the ScalerCrop control, it defines a
     * rectangle taken from the sensor's active pixel array.
     * 
     * This property is valid only after the camera has been successfully
     * configured and its value may change whenever a new configuration is
     * applied.
     * 
     * \todo Turn this property into a "maximum control value" for the
     * ScalerCrop control once "dynamic" controls have been implemented.
     */
    LIBCAMERA_CONTROL_ID_SCALER_CROP_MAXIMUM = 8,
    /**
     * \brief The relative sensitivity of the chosen sensor mode.
     * 
     * Some sensors have readout modes with different sensitivities. For example,
     * a binned camera mode might, with the same exposure and gains, produce
     * twice the signal level of the full resolution readout. This would be
     * signalled by the binned mode, when it is chosen, indicating a value here
     * that is twice that of the full resolution mode. This value will be valid
     * after the configure method has returned successfully.
     */
    LIBCAMERA_CONTROL_ID_SENSOR_SENSITIVITY = 9,
    /**
     * \brief The arrangement of color filters on sensor; represents the colors in the
     * top-left 2x2 section of the sensor, in reading order. Currently
     * identical to ANDROID_SENSOR_INFO_COLOR_FILTER_ARRANGEMENT.
     */
    LIBCAMERA_CONTROL_ID_COLOR_FILTER_ARRANGEMENT = 10,
};

/**
 * \brief Supported values for LIBCAMERA_CONTROL_ID_LOCATION control
 */
enum libcamera_location {
    /**
     * \brief The camera is mounted on the front side of the device, facing the
     * user
     */
    LIBCAMERA_CAMERA_LOCATION_FRONT = 0,
    /**
     * \brief The camera is mounted on the back side of the device, facing away
     * from the user
     */
    LIBCAMERA_CAMERA_LOCATION_BACK = 1,
    /**
     * \brief The camera is attached to the device in a way that allows it to
     * be moved freely
     */
    LIBCAMERA_CAMERA_LOCATION_EXTERNAL = 2,
};

/**
 * \brief Supported values for LIBCAMERA_CONTROL_ID_COLOR_FILTER_ARRANGEMENT control
 */
enum libcamera_color_filter_arrangement {
    /**
     * \brief RGGB Bayer pattern
     */
    LIBCAMERA_RGGB = 0,
    /**
     * \brief GRBG Bayer pattern
     */
    LIBCAMERA_GRBG = 1,
    /**
     * \brief GBRG Bayer pattern
     */
    LIBCAMERA_GBRG = 2,
    /**
     * \brief BGGR Bayer pattern
     */
    LIBCAMERA_BGGR = 3,
    /**
     * \brief Sensor is not Bayer; output has 3 16-bit values for each pixel,
     * instead of just 1 16-bit value per pixel.
     */
    LIBCAMERA_RGB = 4,
    /**
     * \brief Sensor is not Bayer; output consists of a single colour channel.
     */
    LIBCAMERA_MONO = 5,
};

