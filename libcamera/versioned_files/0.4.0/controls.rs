use std::ops::{Deref, DerefMut};
use num_enum::{IntoPrimitive, TryFromPrimitive};
#[allow(unused_imports)]
use crate::control::{Control, Property, ControlEntry, DynControlEntry};
use crate::control_value::{ControlValue, ControlValueError};
#[allow(unused_imports)]
use crate::geometry::{Rectangle, Size};
#[allow(unused_imports)]
use libcamera_sys::*;
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum ControlId {
    /// Enable or disable the AE.
    ///
    /// \sa ExposureTime AnalogueGain
    AeEnable = AE_ENABLE,
    /// Report the lock status of a running AE algorithm.
    ///
    /// If the AE algorithm is locked the value shall be set to true, if it's
    /// converging it shall be set to false. If the AE algorithm is not
    /// running the control shall not be present in the metadata control list.
    ///
    /// \sa AeEnable
    AeLocked = AE_LOCKED,
    /// Specify a metering mode for the AE algorithm to use.
    ///
    /// The metering modes determine which parts of the image are used to
    /// determine the scene brightness. Metering modes may be platform specific
    /// and not all metering modes may be supported.
    AeMeteringMode = AE_METERING_MODE,
    /// Specify a constraint mode for the AE algorithm to use.
    ///
    /// The constraint modes determine how the measured scene brightness is
    /// adjusted to reach the desired target exposure. Constraint modes may be
    /// platform specific, and not all constraint modes may be supported.
    AeConstraintMode = AE_CONSTRAINT_MODE,
    /// Specify an exposure mode for the AE algorithm to use.
    ///
    /// The exposure modes specify how the desired total exposure is divided
    /// between the exposure time and the sensor's analogue gain. They are
    /// platform specific, and not all exposure modes may be supported.
    AeExposureMode = AE_EXPOSURE_MODE,
    /// Specify an Exposure Value (EV) parameter.
    ///
    /// The EV parameter will only be applied if the AE algorithm is currently
    /// enabled.
    ///
    /// By convention EV adjusts the exposure as log2. For example
    /// EV = [-2, -1, -0.5, 0, 0.5, 1, 2] results in an exposure adjustment
    /// of [1/4x, 1/2x, 1/sqrt(2)x, 1x, sqrt(2)x, 2x, 4x].
    ///
    /// \sa AeEnable
    ExposureValue = EXPOSURE_VALUE,
    /// Exposure time for the frame applied in the sensor device.
    ///
    /// This value is specified in micro-seconds.
    ///
    /// Setting this value means that it is now fixed and the AE algorithm may
    /// not change it. Setting it back to zero returns it to the control of the
    /// AE algorithm.
    ///
    /// \sa AnalogueGain AeEnable
    ///
    /// \todo Document the interactions between AeEnable and setting a fixed
    /// value for this control. Consider interactions with other AE features,
    /// such as aperture and aperture/shutter priority mode, and decide if
    /// control of which features should be automatically adjusted shouldn't
    /// better be handled through a separate AE mode control.
    ExposureTime = EXPOSURE_TIME,
    /// Analogue gain value applied in the sensor device.
    ///
    /// The value of the control specifies the gain multiplier applied to all
    /// colour channels. This value cannot be lower than 1.0.
    ///
    /// Setting this value means that it is now fixed and the AE algorithm may
    /// not change it. Setting it back to zero returns it to the control of the
    /// AE algorithm.
    ///
    /// \sa ExposureTime AeEnable
    ///
    /// \todo Document the interactions between AeEnable and setting a fixed
    /// value for this control. Consider interactions with other AE features,
    /// such as aperture and aperture/shutter priority mode, and decide if
    /// control of which features should be automatically adjusted shouldn't
    /// better be handled through a separate AE mode control.
    AnalogueGain = ANALOGUE_GAIN,
    /// Set the flicker avoidance mode for AGC/AEC.
    ///
    /// The flicker mode determines whether, and how, the AGC/AEC algorithm
    /// attempts to hide flicker effects caused by the duty cycle of artificial
    /// lighting.
    ///
    /// Although implementation dependent, many algorithms for "flicker
    /// avoidance" work by restricting this exposure time to integer multiples
    /// of the cycle period, wherever possible.
    ///
    /// Implementations may not support all of the flicker modes listed below.
    ///
    /// By default the system will start in FlickerAuto mode if this is
    /// supported, otherwise the flicker mode will be set to FlickerOff.
    AeFlickerMode = AE_FLICKER_MODE,
    /// Manual flicker period in microseconds.
    ///
    /// This value sets the current flicker period to avoid. It is used when
    /// AeFlickerMode is set to FlickerManual.
    ///
    /// To cancel 50Hz mains flicker, this should be set to 10000 (corresponding
    /// to 100Hz), or 8333 (120Hz) for 60Hz mains.
    ///
    /// Setting the mode to FlickerManual when no AeFlickerPeriod has ever been
    /// set means that no flicker cancellation occurs (until the value of this
    /// control is updated).
    ///
    /// Switching to modes other than FlickerManual has no effect on the
    /// value of the AeFlickerPeriod control.
    ///
    /// \sa AeFlickerMode
    AeFlickerPeriod = AE_FLICKER_PERIOD,
    /// Flicker period detected in microseconds.
    ///
    /// The value reported here indicates the currently detected flicker
    /// period, or zero if no flicker at all is detected.
    ///
    /// When AeFlickerMode is set to FlickerAuto, there may be a period during
    /// which the value reported here remains zero. Once a non-zero value is
    /// reported, then this is the flicker period that has been detected and is
    /// now being cancelled.
    ///
    /// In the case of 50Hz mains flicker, the value would be 10000
    /// (corresponding to 100Hz), or 8333 (120Hz) for 60Hz mains flicker.
    ///
    /// It is implementation dependent whether the system can continue to detect
    /// flicker of different periods when another frequency is already being
    /// cancelled.
    ///
    /// \sa AeFlickerMode
    AeFlickerDetected = AE_FLICKER_DETECTED,
    /// Specify a fixed brightness parameter.
    ///
    /// Positive values (up to 1.0) produce brighter images; negative values
    /// (up to -1.0) produce darker images and 0.0 leaves pixels unchanged.
    Brightness = BRIGHTNESS,
    /// Specify a fixed contrast parameter.
    ///
    /// Normal contrast is given by the value 1.0; larger values produce images
    /// with more contrast.
    Contrast = CONTRAST,
    /// Report an estimate of the current illuminance level in lux.
    ///
    /// The Lux control can only be returned in metadata.
    Lux = LUX,
    /// Enable or disable the AWB.
    ///
    /// When AWB is enabled, the algorithm estimates the colour temperature of
    /// the scene and computes colour gains and the colour correction matrix
    /// automatically. The computed colour temperature, gains and correction
    /// matrix are reported in metadata. The corresponding controls are ignored
    /// if set in a request.
    ///
    /// When AWB is disabled, the colour temperature, gains and correction
    /// matrix are not updated automatically and can be set manually in
    /// requests.
    ///
    /// \sa ColourCorrectionMatrix
    /// \sa ColourGains
    /// \sa ColourTemperature
    AwbEnable = AWB_ENABLE,
    /// Specify the range of illuminants to use for the AWB algorithm.
    ///
    /// The modes supported are platform specific, and not all modes may be
    /// supported.
    AwbMode = AWB_MODE,
    /// Report the lock status of a running AWB algorithm.
    ///
    /// If the AWB algorithm is locked the value shall be set to true, if it's
    /// converging it shall be set to false. If the AWB algorithm is not
    /// running the control shall not be present in the metadata control list.
    ///
    /// \sa AwbEnable
    AwbLocked = AWB_LOCKED,
    /// Pair of gain values for the Red and Blue colour channels, in that
    /// order.
    ///
    /// ColourGains can only be applied in a Request when the AWB is disabled.
    /// If ColourGains is set in a request but ColourTemperature is not, the
    /// implementation shall calculate and set the ColourTemperature based on
    /// the ColourGains.
    ///
    /// \sa AwbEnable
    /// \sa ColourTemperature
    ColourGains = COLOUR_GAINS,
    /// ColourTemperature of the frame, in kelvin.
    ///
    /// ColourTemperature can only be applied in a Request when the AWB is
    /// disabled.
    ///
    /// If ColourTemperature is set in a request but ColourGains is not, the
    /// implementation shall calculate and set the ColourGains based on the
    /// given ColourTemperature. If ColourTemperature is set (either directly,
    /// or indirectly by setting ColourGains) but ColourCorrectionMatrix is not,
    /// the ColourCorrectionMatrix is updated based on the ColourTemperature.
    ///
    /// The ColourTemperature used to process the frame is reported in metadata.
    ///
    /// \sa AwbEnable
    /// \sa ColourCorrectionMatrix
    /// \sa ColourGains
    ColourTemperature = COLOUR_TEMPERATURE,
    /// Specify a fixed saturation parameter.
    ///
    /// Normal saturation is given by the value 1.0; larger values produce more
    /// saturated colours; 0.0 produces a greyscale image.
    Saturation = SATURATION,
    /// Reports the sensor black levels used for processing a frame.
    ///
    /// The values are in the order R, Gr, Gb, B. They are returned as numbers
    /// out of a 16-bit pixel range (as if pixels ranged from 0 to 65535). The
    /// SensorBlackLevels control can only be returned in metadata.
    SensorBlackLevels = SENSOR_BLACK_LEVELS,
    /// Intensity of the sharpening applied to the image.
    ///
    /// A value of 0.0 means no sharpening. The minimum value means
    /// minimal sharpening, and shall be 0.0 unless the camera can't
    /// disable sharpening completely. The default value shall give a
    /// "reasonable" level of sharpening, suitable for most use cases.
    /// The maximum value may apply extremely high levels of sharpening,
    /// higher than anyone could reasonably want. Negative values are
    /// not allowed. Note also that sharpening is not applied to raw
    /// streams.
    Sharpness = SHARPNESS,
    /// Reports a Figure of Merit (FoM) to indicate how in-focus the frame is.
    ///
    /// A larger FocusFoM value indicates a more in-focus frame. This singular
    /// value may be based on a combination of statistics gathered from
    /// multiple focus regions within an image. The number of focus regions and
    /// method of combination is platform dependent. In this respect, it is not
    /// necessarily aimed at providing a way to implement a focus algorithm by
    /// the application, rather an indication of how in-focus a frame is.
    FocusFoM = FOCUS_FO_M,
    /// The 3x3 matrix that converts camera RGB to sRGB within the imaging
    /// pipeline.
    ///
    /// This should describe the matrix that is used after pixels have been
    /// white-balanced, but before any gamma transformation. The 3x3 matrix is
    /// stored in conventional reading order in an array of 9 floating point
    /// values.
    ///
    /// ColourCorrectionMatrix can only be applied in a Request when the AWB is
    /// disabled.
    ///
    /// \sa AwbEnable
    /// \sa ColourTemperature
    ColourCorrectionMatrix = COLOUR_CORRECTION_MATRIX,
    /// Sets the image portion that will be scaled to form the whole of
    /// the final output image.
    ///
    /// The (x,y) location of this rectangle is relative to the
    /// PixelArrayActiveAreas that is being used. The units remain native
    /// sensor pixels, even if the sensor is being used in a binning or
    /// skipping mode.
    ///
    /// This control is only present when the pipeline supports scaling. Its
    /// maximum valid value is given by the properties::ScalerCropMaximum
    /// property, and the two can be used to implement digital zoom.
    ScalerCrop = SCALER_CROP,
    /// Digital gain value applied during the processing steps applied
    /// to the image as captured from the sensor.
    ///
    /// The global digital gain factor is applied to all the colour channels
    /// of the RAW image. Different pipeline models are free to
    /// specify how the global gain factor applies to each separate
    /// channel.
    ///
    /// If an imaging pipeline applies digital gain in distinct
    /// processing steps, this value indicates their total sum.
    /// Pipelines are free to decide how to adjust each processing
    /// step to respect the received gain factor and shall report
    /// their total value in the request metadata.
    DigitalGain = DIGITAL_GAIN,
    /// The instantaneous frame duration from start of frame exposure to start
    /// of next exposure, expressed in microseconds.
    ///
    /// This control is meant to be returned in metadata.
    FrameDuration = FRAME_DURATION,
    /// The minimum and maximum (in that order) frame duration, expressed in
    /// microseconds.
    ///
    /// When provided by applications, the control specifies the sensor frame
    /// duration interval the pipeline has to use. This limits the largest
    /// exposure time the sensor can use. For example, if a maximum frame
    /// duration of 33ms is requested (corresponding to 30 frames per second),
    /// the sensor will not be able to raise the exposure time above 33ms.
    /// A fixed frame duration is achieved by setting the minimum and maximum
    /// values to be the same. Setting both values to 0 reverts to using the
    /// camera defaults.
    ///
    /// The maximum frame duration provides the absolute limit to the exposure
    /// time computed by the AE algorithm and it overrides any exposure mode
    /// setting specified with controls::AeExposureMode. Similarly, when a
    /// manual exposure time is set through controls::ExposureTime, it also
    /// gets clipped to the limits set by this control. When reported in
    /// metadata, the control expresses the minimum and maximum frame durations
    /// used after being clipped to the sensor provided frame duration limits.
    ///
    /// \sa AeExposureMode
    /// \sa ExposureTime
    ///
    /// \todo Define how to calculate the capture frame rate by
    /// defining controls to report additional delays introduced by
    /// the capture pipeline or post-processing stages (ie JPEG
    /// conversion, frame scaling).
    ///
    /// \todo Provide an explicit definition of default control values, for
    /// this and all other controls.
    FrameDurationLimits = FRAME_DURATION_LIMITS,
    /// Temperature measure from the camera sensor in Celsius.
    ///
    /// This value is typically obtained by a thermal sensor present on-die or
    /// in the camera module. The range of reported temperatures is device
    /// dependent.
    ///
    /// The SensorTemperature control will only be returned in metadata if a
    /// thermal sensor is present.
    SensorTemperature = SENSOR_TEMPERATURE,
    /// The time when the first row of the image sensor active array is exposed.
    ///
    /// The timestamp, expressed in nanoseconds, represents a monotonically
    /// increasing counter since the system boot time, as defined by the
    /// Linux-specific CLOCK_BOOTTIME clock id.
    ///
    /// The SensorTimestamp control can only be returned in metadata.
    ///
    /// \todo Define how the sensor timestamp has to be used in the reprocessing
    /// use case.
    SensorTimestamp = SENSOR_TIMESTAMP,
    /// The mode of the AF (autofocus) algorithm.
    ///
    /// An implementation may choose not to implement all the modes.
    AfMode = AF_MODE,
    /// The range of focus distances that is scanned.
    ///
    /// An implementation may choose not to implement all the options here.
    AfRange = AF_RANGE,
    /// Determine whether the AF is to move the lens as quickly as possible or
    /// more steadily.
    ///
    /// For example, during video recording it may be desirable not to move the
    /// lens too abruptly, but when in a preview mode (waiting for a still
    /// capture) it may be helpful to move the lens as quickly as is reasonably
    /// possible.
    AfSpeed = AF_SPEED,
    /// The parts of the image used by the AF algorithm to measure focus.
    AfMetering = AF_METERING,
    /// The focus windows used by the AF algorithm when AfMetering is set to
    /// AfMeteringWindows.
    ///
    /// The units used are pixels within the rectangle returned by the
    /// ScalerCropMaximum property.
    ///
    /// In order to be activated, a rectangle must be programmed with non-zero
    /// width and height. Internally, these rectangles are intersected with the
    /// ScalerCropMaximum rectangle. If the window becomes empty after this
    /// operation, then the window is ignored. If all the windows end up being
    /// ignored, then the behaviour is platform dependent.
    ///
    /// On platforms that support the ScalerCrop control (for implementing
    /// digital zoom, for example), no automatic recalculation or adjustment of
    /// AF windows is performed internally if the ScalerCrop is changed. If any
    /// window lies outside the output image after the scaler crop has been
    /// applied, it is up to the application to recalculate them.
    ///
    /// The details of how the windows are used are platform dependent. We note
    /// that when there is more than one AF window, a typical implementation
    /// might find the optimal focus position for each one and finally select
    /// the window where the focal distance for the objects shown in that part
    /// of the image are closest to the camera.
    AfWindows = AF_WINDOWS,
    /// Start an autofocus scan.
    ///
    /// This control starts an autofocus scan when AfMode is set to AfModeAuto,
    /// and is ignored if AfMode is set to AfModeManual or AfModeContinuous. It
    /// can also be used to terminate a scan early.
    AfTrigger = AF_TRIGGER,
    /// Pause lens movements when in continuous autofocus mode.
    ///
    /// This control has no effect except when in continuous autofocus mode
    /// (AfModeContinuous). It can be used to pause any lens movements while
    /// (for example) images are captured. The algorithm remains inactive
    /// until it is instructed to resume.
    AfPause = AF_PAUSE,
    /// Set and report the focus lens position.
    ///
    /// This control instructs the lens to move to a particular position and
    /// also reports back the position of the lens for each frame.
    ///
    /// The LensPosition control is ignored unless the AfMode is set to
    /// AfModeManual, though the value is reported back unconditionally in all
    /// modes.
    ///
    /// This value, which is generally a non-integer, is the reciprocal of the
    /// focal distance in metres, also known as dioptres. That is, to set a
    /// focal distance D, the lens position LP is given by
    ///
    /// \f$LP = \frac{1\mathrm{m}}{D}\f$
    ///
    /// For example:
    ///
    /// - 0 moves the lens to infinity.
    /// - 0.5 moves the lens to focus on objects 2m away.
    /// - 2 moves the lens to focus on objects 50cm away.
    /// - And larger values will focus the lens closer.
    ///
    /// The default value of the control should indicate a good general
    /// position for the lens, often corresponding to the hyperfocal distance
    /// (the closest position for which objects at infinity are still
    /// acceptably sharp). The minimum will often be zero (meaning infinity),
    /// and the maximum value defines the closest focus position.
    ///
    /// \todo Define a property to report the Hyperfocal distance of calibrated
    /// lenses.
    LensPosition = LENS_POSITION,
    /// The current state of the AF algorithm.
    ///
    /// This control reports the current state of the AF algorithm in
    /// conjunction with the reported AfMode value and (in continuous AF mode)
    /// the AfPauseState value. The possible state changes are described below,
    /// though we note the following state transitions that occur when the
    /// AfMode is changed.
    ///
    /// If the AfMode is set to AfModeManual, then the AfState will always
    /// report AfStateIdle (even if the lens is subsequently moved). Changing
    /// to the AfModeManual state does not initiate any lens movement.
    ///
    /// If the AfMode is set to AfModeAuto then the AfState will report
    /// AfStateIdle. However, if AfModeAuto and AfTriggerStart are sent
    /// together then AfState will omit AfStateIdle and move straight to
    /// AfStateScanning (and start a scan).
    ///
    /// If the AfMode is set to AfModeContinuous then the AfState will
    /// initially report AfStateScanning.
    AfState = AF_STATE,
    /// Report whether the autofocus is currently running, paused or pausing.
    ///
    /// This control is only applicable in continuous (AfModeContinuous) mode,
    /// and reports whether the algorithm is currently running, paused or
    /// pausing (that is, will pause as soon as any in-progress scan
    /// completes).
    ///
    /// Any change to AfMode will cause AfPauseStateRunning to be reported.
    AfPauseState = AF_PAUSE_STATE,
    /// Set the mode to be used for High Dynamic Range (HDR) imaging.
    ///
    /// HDR techniques typically include multiple exposure, image fusion and
    /// tone mapping techniques to improve the dynamic range of the resulting
    /// images.
    ///
    /// When using an HDR mode, images are captured with different sets of AGC
    /// settings called HDR channels. Channels indicate in particular the type
    /// of exposure (short, medium or long) used to capture the raw image,
    /// before fusion. Each HDR image is tagged with the corresponding channel
    /// using the HdrChannel control.
    ///
    /// \sa HdrChannel
    HdrMode = HDR_MODE,
    /// The HDR channel used to capture the frame.
    ///
    /// This value is reported back to the application so that it can discover
    /// whether this capture corresponds to the short or long exposure image
    /// (or any other image used by the HDR procedure). An application can
    /// monitor the HDR channel to discover when the differently exposed images
    /// have arrived.
    ///
    /// This metadata is only available when an HDR mode has been enabled.
    ///
    /// \sa HdrMode
    HdrChannel = HDR_CHANNEL,
    /// Specify a fixed gamma value.
    ///
    /// The default gamma value must be 2.2 which closely mimics sRGB gamma.
    /// Note that this is camera gamma, so it is applied as 1.0/gamma.
    Gamma = GAMMA,
    /// Enable or disable the debug metadata.
    DebugMetadataEnable = DEBUG_METADATA_ENABLE,
    /// Control for AE metering trigger. Currently identical to
    /// ANDROID_CONTROL_AE_PRECAPTURE_TRIGGER.
    ///
    /// Whether the camera device will trigger a precapture metering sequence
    /// when it processes this request.
    #[cfg(feature = "vendor_draft")]
    AePrecaptureTrigger = AE_PRECAPTURE_TRIGGER,
    /// Control to select the noise reduction algorithm mode. Currently
    /// identical to ANDROID_NOISE_REDUCTION_MODE.
    ///
    ///  Mode of operation for the noise reduction algorithm.
    #[cfg(feature = "vendor_draft")]
    NoiseReductionMode = NOISE_REDUCTION_MODE,
    /// Control to select the color correction aberration mode. Currently
    /// identical to ANDROID_COLOR_CORRECTION_ABERRATION_MODE.
    ///
    ///  Mode of operation for the chromatic aberration correction algorithm.
    #[cfg(feature = "vendor_draft")]
    ColorCorrectionAberrationMode = COLOR_CORRECTION_ABERRATION_MODE,
    /// Control to report the current AE algorithm state. Currently identical to
    /// ANDROID_CONTROL_AE_STATE.
    ///
    ///  Current state of the AE algorithm.
    #[cfg(feature = "vendor_draft")]
    AeState = AE_STATE,
    /// Control to report the current AWB algorithm state. Currently identical
    /// to ANDROID_CONTROL_AWB_STATE.
    ///
    ///  Current state of the AWB algorithm.
    #[cfg(feature = "vendor_draft")]
    AwbState = AWB_STATE,
    /// Control to report the time between the start of exposure of the first
    /// row and the start of exposure of the last row. Currently identical to
    /// ANDROID_SENSOR_ROLLING_SHUTTER_SKEW
    #[cfg(feature = "vendor_draft")]
    SensorRollingShutterSkew = SENSOR_ROLLING_SHUTTER_SKEW,
    /// Control to report if the lens shading map is available. Currently
    /// identical to ANDROID_STATISTICS_LENS_SHADING_MAP_MODE.
    #[cfg(feature = "vendor_draft")]
    LensShadingMapMode = LENS_SHADING_MAP_MODE,
    /// Specifies the number of pipeline stages the frame went through from when
    /// it was exposed to when the final completed result was available to the
    /// framework. Always less than or equal to PipelineMaxDepth. Currently
    /// identical to ANDROID_REQUEST_PIPELINE_DEPTH.
    ///
    /// The typical value for this control is 3 as a frame is first exposed,
    /// captured and then processed in a single pass through the ISP. Any
    /// additional processing step performed after the ISP pass (in example face
    /// detection, additional format conversions etc) count as an additional
    /// pipeline stage.
    #[cfg(feature = "vendor_draft")]
    PipelineDepth = PIPELINE_DEPTH,
    /// The maximum number of frames that can occur after a request (different
    /// than the previous) has been submitted, and before the result's state
    /// becomes synchronized. A value of -1 indicates unknown latency, and 0
    /// indicates per-frame control. Currently identical to
    /// ANDROID_SYNC_MAX_LATENCY.
    #[cfg(feature = "vendor_draft")]
    MaxLatency = MAX_LATENCY,
    /// Control to select the test pattern mode. Currently identical to
    /// ANDROID_SENSOR_TEST_PATTERN_MODE.
    #[cfg(feature = "vendor_draft")]
    TestPatternMode = TEST_PATTERN_MODE,
    /// Control to select the face detection mode used by the pipeline.
    ///
    /// Currently identical to ANDROID_STATISTICS_FACE_DETECT_MODE.
    ///
    /// \sa FaceDetectFaceRectangles
    /// \sa FaceDetectFaceScores
    /// \sa FaceDetectFaceLandmarks
    /// \sa FaceDetectFaceIds
    #[cfg(feature = "vendor_draft")]
    FaceDetectMode = FACE_DETECT_MODE,
    /// Boundary rectangles of the detected faces. The number of values is
    /// the number of detected faces.
    ///
    /// The FaceDetectFaceRectangles control can only be returned in metadata.
    ///
    /// Currently identical to ANDROID_STATISTICS_FACE_RECTANGLES.
    #[cfg(feature = "vendor_draft")]
    FaceDetectFaceRectangles = FACE_DETECT_FACE_RECTANGLES,
    /// Confidence score of each of the detected faces. The range of score is
    /// [0, 100]. The number of values should be the number of faces reported
    /// in FaceDetectFaceRectangles.
    ///
    /// The FaceDetectFaceScores control can only be returned in metadata.
    ///
    /// Currently identical to ANDROID_STATISTICS_FACE_SCORES.
    #[cfg(feature = "vendor_draft")]
    FaceDetectFaceScores = FACE_DETECT_FACE_SCORES,
    /// Array of human face landmark coordinates in format [..., left_eye_i,
    /// right_eye_i, mouth_i, left_eye_i+1, ...], with i = index of face. The
    /// number of values should be 3 * the number of faces reported in
    /// FaceDetectFaceRectangles.
    ///
    /// The FaceDetectFaceLandmarks control can only be returned in metadata.
    ///
    /// Currently identical to ANDROID_STATISTICS_FACE_LANDMARKS.
    #[cfg(feature = "vendor_draft")]
    FaceDetectFaceLandmarks = FACE_DETECT_FACE_LANDMARKS,
    /// Each detected face is given a unique ID that is valid for as long as the
    /// face is visible to the camera device. A face that leaves the field of
    /// view and later returns may be assigned a new ID. The number of values
    /// should be the number of faces reported in FaceDetectFaceRectangles.
    ///
    /// The FaceDetectFaceIds control can only be returned in metadata.
    ///
    /// Currently identical to ANDROID_STATISTICS_FACE_IDS.
    #[cfg(feature = "vendor_draft")]
    FaceDetectFaceIds = FACE_DETECT_FACE_IDS,
    /// Toggles the Raspberry Pi IPA to output the hardware generated statistics.
    ///
    /// When this control is set to true, the IPA outputs a binary dump of the
    /// hardware generated statistics through the Request metadata in the
    /// Bcm2835StatsOutput control.
    ///
    /// \sa Bcm2835StatsOutput
    #[cfg(feature = "vendor_rpi")]
    StatsOutputEnable = STATS_OUTPUT_ENABLE,
    /// Span of the BCM2835 ISP generated statistics for the current frame.
    ///
    /// This is sent in the Request metadata if the StatsOutputEnable is set to
    /// true.  The statistics struct definition can be found in
    /// include/linux/bcm2835-isp.h.
    ///
    /// \sa StatsOutputEnable
    #[cfg(feature = "vendor_rpi")]
    Bcm2835StatsOutput = BCM2835_STATS_OUTPUT,
    /// An array of rectangles, where each singular value has identical
    /// functionality to the ScalerCrop control. This control allows the
    /// Raspberry Pi pipeline handler to control individual scaler crops per
    /// output stream.
    ///
    /// The order of rectangles passed into the control must match the order of
    /// streams configured by the application. The pipeline handler will only
    /// configure crop retangles up-to the number of output streams configured.
    /// All subsequent rectangles passed into this control are ignored by the
    /// pipeline handler.
    ///
    /// If both rpi::ScalerCrops and ScalerCrop controls are present in a
    /// ControlList, the latter is discarded, and crops are obtained from this
    /// control.
    ///
    /// Note that using different crop rectangles for each output stream with
    /// this control is only applicable on the Pi5/PiSP platform. This control
    /// should also be considered temporary/draft and will be replaced with
    /// official libcamera API support for per-stream controls in the future.
    ///
    /// \sa ScalerCrop
    #[cfg(feature = "vendor_rpi")]
    ScalerCrops = SCALER_CROPS,
}
/// Enable or disable the AE.
///
/// \sa ExposureTime AnalogueGain
#[derive(Debug, Clone)]
pub struct AeEnable(pub bool);
impl Deref for AeEnable {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for AeEnable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for AeEnable {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<bool>::try_from(value)?))
    }
}
impl From<AeEnable> for ControlValue {
    fn from(val: AeEnable) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for AeEnable {
    const ID: u32 = ControlId::AeEnable as _;
}
impl Control for AeEnable {}
/// Report the lock status of a running AE algorithm.
///
/// If the AE algorithm is locked the value shall be set to true, if it's
/// converging it shall be set to false. If the AE algorithm is not
/// running the control shall not be present in the metadata control list.
///
/// \sa AeEnable
#[derive(Debug, Clone)]
pub struct AeLocked(pub bool);
impl Deref for AeLocked {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for AeLocked {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for AeLocked {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<bool>::try_from(value)?))
    }
}
impl From<AeLocked> for ControlValue {
    fn from(val: AeLocked) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for AeLocked {
    const ID: u32 = ControlId::AeLocked as _;
}
impl Control for AeLocked {}
/// Specify a metering mode for the AE algorithm to use.
///
/// The metering modes determine which parts of the image are used to
/// determine the scene brightness. Metering modes may be platform specific
/// and not all metering modes may be supported.
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum AeMeteringMode {
    /// Centre-weighted metering mode.
    MeteringCentreWeighted = 0,
    /// Spot metering mode.
    MeteringSpot = 1,
    /// Matrix metering mode.
    MeteringMatrix = 2,
    /// Custom metering mode.
    MeteringCustom = 3,
}
impl TryFrom<ControlValue> for AeMeteringMode {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Self::try_from(i32::try_from(value.clone())?)
            .map_err(|_| ControlValueError::UnknownVariant(value))
    }
}
impl From<AeMeteringMode> for ControlValue {
    fn from(val: AeMeteringMode) -> Self {
        ControlValue::from(<i32>::from(val))
    }
}
impl ControlEntry for AeMeteringMode {
    const ID: u32 = ControlId::AeMeteringMode as _;
}
impl Control for AeMeteringMode {}
/// Specify a constraint mode for the AE algorithm to use.
///
/// The constraint modes determine how the measured scene brightness is
/// adjusted to reach the desired target exposure. Constraint modes may be
/// platform specific, and not all constraint modes may be supported.
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum AeConstraintMode {
    /// Default constraint mode.
    ///
    /// This mode aims to balance the exposure of different parts of the
    /// image so as to reach a reasonable average level. However, highlights
    /// in the image may appear over-exposed and lowlights may appear
    /// under-exposed.
    ConstraintNormal = 0,
    /// Highlight constraint mode.
    ///
    /// This mode adjusts the exposure levels in order to try and avoid
    /// over-exposing the brightest parts (highlights) of an image.
    /// Other non-highlight parts of the image may appear under-exposed.
    ConstraintHighlight = 1,
    /// Shadows constraint mode.
    ///
    /// This mode adjusts the exposure levels in order to try and avoid
    /// under-exposing the dark parts (shadows) of an image. Other normally
    /// exposed parts of the image may appear over-exposed.
    ConstraintShadows = 2,
    /// Custom constraint mode.
    ConstraintCustom = 3,
}
impl TryFrom<ControlValue> for AeConstraintMode {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Self::try_from(i32::try_from(value.clone())?)
            .map_err(|_| ControlValueError::UnknownVariant(value))
    }
}
impl From<AeConstraintMode> for ControlValue {
    fn from(val: AeConstraintMode) -> Self {
        ControlValue::from(<i32>::from(val))
    }
}
impl ControlEntry for AeConstraintMode {
    const ID: u32 = ControlId::AeConstraintMode as _;
}
impl Control for AeConstraintMode {}
/// Specify an exposure mode for the AE algorithm to use.
///
/// The exposure modes specify how the desired total exposure is divided
/// between the exposure time and the sensor's analogue gain. They are
/// platform specific, and not all exposure modes may be supported.
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum AeExposureMode {
    /// Default exposure mode.
    ExposureNormal = 0,
    /// Exposure mode allowing only short exposure times.
    ExposureShort = 1,
    /// Exposure mode allowing long exposure times.
    ExposureLong = 2,
    /// Custom exposure mode.
    ExposureCustom = 3,
}
impl TryFrom<ControlValue> for AeExposureMode {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Self::try_from(i32::try_from(value.clone())?)
            .map_err(|_| ControlValueError::UnknownVariant(value))
    }
}
impl From<AeExposureMode> for ControlValue {
    fn from(val: AeExposureMode) -> Self {
        ControlValue::from(<i32>::from(val))
    }
}
impl ControlEntry for AeExposureMode {
    const ID: u32 = ControlId::AeExposureMode as _;
}
impl Control for AeExposureMode {}
/// Specify an Exposure Value (EV) parameter.
///
/// The EV parameter will only be applied if the AE algorithm is currently
/// enabled.
///
/// By convention EV adjusts the exposure as log2. For example
/// EV = [-2, -1, -0.5, 0, 0.5, 1, 2] results in an exposure adjustment
/// of [1/4x, 1/2x, 1/sqrt(2)x, 1x, sqrt(2)x, 2x, 4x].
///
/// \sa AeEnable
#[derive(Debug, Clone)]
pub struct ExposureValue(pub f32);
impl Deref for ExposureValue {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for ExposureValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for ExposureValue {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<f32>::try_from(value)?))
    }
}
impl From<ExposureValue> for ControlValue {
    fn from(val: ExposureValue) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for ExposureValue {
    const ID: u32 = ControlId::ExposureValue as _;
}
impl Control for ExposureValue {}
/// Exposure time for the frame applied in the sensor device.
///
/// This value is specified in micro-seconds.
///
/// Setting this value means that it is now fixed and the AE algorithm may
/// not change it. Setting it back to zero returns it to the control of the
/// AE algorithm.
///
/// \sa AnalogueGain AeEnable
///
/// \todo Document the interactions between AeEnable and setting a fixed
/// value for this control. Consider interactions with other AE features,
/// such as aperture and aperture/shutter priority mode, and decide if
/// control of which features should be automatically adjusted shouldn't
/// better be handled through a separate AE mode control.
#[derive(Debug, Clone)]
pub struct ExposureTime(pub i32);
impl Deref for ExposureTime {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for ExposureTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for ExposureTime {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<i32>::try_from(value)?))
    }
}
impl From<ExposureTime> for ControlValue {
    fn from(val: ExposureTime) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for ExposureTime {
    const ID: u32 = ControlId::ExposureTime as _;
}
impl Control for ExposureTime {}
/// Analogue gain value applied in the sensor device.
///
/// The value of the control specifies the gain multiplier applied to all
/// colour channels. This value cannot be lower than 1.0.
///
/// Setting this value means that it is now fixed and the AE algorithm may
/// not change it. Setting it back to zero returns it to the control of the
/// AE algorithm.
///
/// \sa ExposureTime AeEnable
///
/// \todo Document the interactions between AeEnable and setting a fixed
/// value for this control. Consider interactions with other AE features,
/// such as aperture and aperture/shutter priority mode, and decide if
/// control of which features should be automatically adjusted shouldn't
/// better be handled through a separate AE mode control.
#[derive(Debug, Clone)]
pub struct AnalogueGain(pub f32);
impl Deref for AnalogueGain {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for AnalogueGain {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for AnalogueGain {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<f32>::try_from(value)?))
    }
}
impl From<AnalogueGain> for ControlValue {
    fn from(val: AnalogueGain) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for AnalogueGain {
    const ID: u32 = ControlId::AnalogueGain as _;
}
impl Control for AnalogueGain {}
/// Set the flicker avoidance mode for AGC/AEC.
///
/// The flicker mode determines whether, and how, the AGC/AEC algorithm
/// attempts to hide flicker effects caused by the duty cycle of artificial
/// lighting.
///
/// Although implementation dependent, many algorithms for "flicker
/// avoidance" work by restricting this exposure time to integer multiples
/// of the cycle period, wherever possible.
///
/// Implementations may not support all of the flicker modes listed below.
///
/// By default the system will start in FlickerAuto mode if this is
/// supported, otherwise the flicker mode will be set to FlickerOff.
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum AeFlickerMode {
    /// No flicker avoidance is performed.
    FlickerOff = 0,
    /// Manual flicker avoidance.
    ///
    /// Suppress flicker effects caused by lighting running with a period
    /// specified by the AeFlickerPeriod control.
    /// \sa AeFlickerPeriod
    FlickerManual = 1,
    /// Automatic flicker period detection and avoidance.
    ///
    /// The system will automatically determine the most likely value of
    /// flicker period, and avoid flicker of this frequency. Once flicker
    /// is being corrected, it is implementation dependent whether the
    /// system is still able to detect a change in the flicker period.
    /// \sa AeFlickerDetected
    FlickerAuto = 2,
}
impl TryFrom<ControlValue> for AeFlickerMode {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Self::try_from(i32::try_from(value.clone())?)
            .map_err(|_| ControlValueError::UnknownVariant(value))
    }
}
impl From<AeFlickerMode> for ControlValue {
    fn from(val: AeFlickerMode) -> Self {
        ControlValue::from(<i32>::from(val))
    }
}
impl ControlEntry for AeFlickerMode {
    const ID: u32 = ControlId::AeFlickerMode as _;
}
impl Control for AeFlickerMode {}
/// Manual flicker period in microseconds.
///
/// This value sets the current flicker period to avoid. It is used when
/// AeFlickerMode is set to FlickerManual.
///
/// To cancel 50Hz mains flicker, this should be set to 10000 (corresponding
/// to 100Hz), or 8333 (120Hz) for 60Hz mains.
///
/// Setting the mode to FlickerManual when no AeFlickerPeriod has ever been
/// set means that no flicker cancellation occurs (until the value of this
/// control is updated).
///
/// Switching to modes other than FlickerManual has no effect on the
/// value of the AeFlickerPeriod control.
///
/// \sa AeFlickerMode
#[derive(Debug, Clone)]
pub struct AeFlickerPeriod(pub i32);
impl Deref for AeFlickerPeriod {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for AeFlickerPeriod {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for AeFlickerPeriod {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<i32>::try_from(value)?))
    }
}
impl From<AeFlickerPeriod> for ControlValue {
    fn from(val: AeFlickerPeriod) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for AeFlickerPeriod {
    const ID: u32 = ControlId::AeFlickerPeriod as _;
}
impl Control for AeFlickerPeriod {}
/// Flicker period detected in microseconds.
///
/// The value reported here indicates the currently detected flicker
/// period, or zero if no flicker at all is detected.
///
/// When AeFlickerMode is set to FlickerAuto, there may be a period during
/// which the value reported here remains zero. Once a non-zero value is
/// reported, then this is the flicker period that has been detected and is
/// now being cancelled.
///
/// In the case of 50Hz mains flicker, the value would be 10000
/// (corresponding to 100Hz), or 8333 (120Hz) for 60Hz mains flicker.
///
/// It is implementation dependent whether the system can continue to detect
/// flicker of different periods when another frequency is already being
/// cancelled.
///
/// \sa AeFlickerMode
#[derive(Debug, Clone)]
pub struct AeFlickerDetected(pub i32);
impl Deref for AeFlickerDetected {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for AeFlickerDetected {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for AeFlickerDetected {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<i32>::try_from(value)?))
    }
}
impl From<AeFlickerDetected> for ControlValue {
    fn from(val: AeFlickerDetected) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for AeFlickerDetected {
    const ID: u32 = ControlId::AeFlickerDetected as _;
}
impl Control for AeFlickerDetected {}
/// Specify a fixed brightness parameter.
///
/// Positive values (up to 1.0) produce brighter images; negative values
/// (up to -1.0) produce darker images and 0.0 leaves pixels unchanged.
#[derive(Debug, Clone)]
pub struct Brightness(pub f32);
impl Deref for Brightness {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Brightness {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for Brightness {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<f32>::try_from(value)?))
    }
}
impl From<Brightness> for ControlValue {
    fn from(val: Brightness) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for Brightness {
    const ID: u32 = ControlId::Brightness as _;
}
impl Control for Brightness {}
/// Specify a fixed contrast parameter.
///
/// Normal contrast is given by the value 1.0; larger values produce images
/// with more contrast.
#[derive(Debug, Clone)]
pub struct Contrast(pub f32);
impl Deref for Contrast {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Contrast {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for Contrast {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<f32>::try_from(value)?))
    }
}
impl From<Contrast> for ControlValue {
    fn from(val: Contrast) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for Contrast {
    const ID: u32 = ControlId::Contrast as _;
}
impl Control for Contrast {}
/// Report an estimate of the current illuminance level in lux.
///
/// The Lux control can only be returned in metadata.
#[derive(Debug, Clone)]
pub struct Lux(pub f32);
impl Deref for Lux {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Lux {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for Lux {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<f32>::try_from(value)?))
    }
}
impl From<Lux> for ControlValue {
    fn from(val: Lux) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for Lux {
    const ID: u32 = ControlId::Lux as _;
}
impl Control for Lux {}
/// Enable or disable the AWB.
///
/// When AWB is enabled, the algorithm estimates the colour temperature of
/// the scene and computes colour gains and the colour correction matrix
/// automatically. The computed colour temperature, gains and correction
/// matrix are reported in metadata. The corresponding controls are ignored
/// if set in a request.
///
/// When AWB is disabled, the colour temperature, gains and correction
/// matrix are not updated automatically and can be set manually in
/// requests.
///
/// \sa ColourCorrectionMatrix
/// \sa ColourGains
/// \sa ColourTemperature
#[derive(Debug, Clone)]
pub struct AwbEnable(pub bool);
impl Deref for AwbEnable {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for AwbEnable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for AwbEnable {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<bool>::try_from(value)?))
    }
}
impl From<AwbEnable> for ControlValue {
    fn from(val: AwbEnable) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for AwbEnable {
    const ID: u32 = ControlId::AwbEnable as _;
}
impl Control for AwbEnable {}
/// Specify the range of illuminants to use for the AWB algorithm.
///
/// The modes supported are platform specific, and not all modes may be
/// supported.
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum AwbMode {
    /// Search over the whole colour temperature range.
    AwbAuto = 0,
    /// Incandescent AWB lamp mode.
    AwbIncandescent = 1,
    /// Tungsten AWB lamp mode.
    AwbTungsten = 2,
    /// Fluorescent AWB lamp mode.
    AwbFluorescent = 3,
    /// Indoor AWB lighting mode.
    AwbIndoor = 4,
    /// Daylight AWB lighting mode.
    AwbDaylight = 5,
    /// Cloudy AWB lighting mode.
    AwbCloudy = 6,
    /// Custom AWB mode.
    AwbCustom = 7,
}
impl TryFrom<ControlValue> for AwbMode {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Self::try_from(i32::try_from(value.clone())?)
            .map_err(|_| ControlValueError::UnknownVariant(value))
    }
}
impl From<AwbMode> for ControlValue {
    fn from(val: AwbMode) -> Self {
        ControlValue::from(<i32>::from(val))
    }
}
impl ControlEntry for AwbMode {
    const ID: u32 = ControlId::AwbMode as _;
}
impl Control for AwbMode {}
/// Report the lock status of a running AWB algorithm.
///
/// If the AWB algorithm is locked the value shall be set to true, if it's
/// converging it shall be set to false. If the AWB algorithm is not
/// running the control shall not be present in the metadata control list.
///
/// \sa AwbEnable
#[derive(Debug, Clone)]
pub struct AwbLocked(pub bool);
impl Deref for AwbLocked {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for AwbLocked {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for AwbLocked {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<bool>::try_from(value)?))
    }
}
impl From<AwbLocked> for ControlValue {
    fn from(val: AwbLocked) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for AwbLocked {
    const ID: u32 = ControlId::AwbLocked as _;
}
impl Control for AwbLocked {}
/// Pair of gain values for the Red and Blue colour channels, in that
/// order.
///
/// ColourGains can only be applied in a Request when the AWB is disabled.
/// If ColourGains is set in a request but ColourTemperature is not, the
/// implementation shall calculate and set the ColourTemperature based on
/// the ColourGains.
///
/// \sa AwbEnable
/// \sa ColourTemperature
#[derive(Debug, Clone)]
pub struct ColourGains(pub [f32; 2]);
impl Deref for ColourGains {
    type Target = [f32; 2];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for ColourGains {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for ColourGains {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<[f32; 2]>::try_from(value)?))
    }
}
impl From<ColourGains> for ControlValue {
    fn from(val: ColourGains) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for ColourGains {
    const ID: u32 = ControlId::ColourGains as _;
}
impl Control for ColourGains {}
/// ColourTemperature of the frame, in kelvin.
///
/// ColourTemperature can only be applied in a Request when the AWB is
/// disabled.
///
/// If ColourTemperature is set in a request but ColourGains is not, the
/// implementation shall calculate and set the ColourGains based on the
/// given ColourTemperature. If ColourTemperature is set (either directly,
/// or indirectly by setting ColourGains) but ColourCorrectionMatrix is not,
/// the ColourCorrectionMatrix is updated based on the ColourTemperature.
///
/// The ColourTemperature used to process the frame is reported in metadata.
///
/// \sa AwbEnable
/// \sa ColourCorrectionMatrix
/// \sa ColourGains
#[derive(Debug, Clone)]
pub struct ColourTemperature(pub i32);
impl Deref for ColourTemperature {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for ColourTemperature {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for ColourTemperature {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<i32>::try_from(value)?))
    }
}
impl From<ColourTemperature> for ControlValue {
    fn from(val: ColourTemperature) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for ColourTemperature {
    const ID: u32 = ControlId::ColourTemperature as _;
}
impl Control for ColourTemperature {}
/// Specify a fixed saturation parameter.
///
/// Normal saturation is given by the value 1.0; larger values produce more
/// saturated colours; 0.0 produces a greyscale image.
#[derive(Debug, Clone)]
pub struct Saturation(pub f32);
impl Deref for Saturation {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Saturation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for Saturation {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<f32>::try_from(value)?))
    }
}
impl From<Saturation> for ControlValue {
    fn from(val: Saturation) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for Saturation {
    const ID: u32 = ControlId::Saturation as _;
}
impl Control for Saturation {}
/// Reports the sensor black levels used for processing a frame.
///
/// The values are in the order R, Gr, Gb, B. They are returned as numbers
/// out of a 16-bit pixel range (as if pixels ranged from 0 to 65535). The
/// SensorBlackLevels control can only be returned in metadata.
#[derive(Debug, Clone)]
pub struct SensorBlackLevels(pub [i32; 4]);
impl Deref for SensorBlackLevels {
    type Target = [i32; 4];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for SensorBlackLevels {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for SensorBlackLevels {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<[i32; 4]>::try_from(value)?))
    }
}
impl From<SensorBlackLevels> for ControlValue {
    fn from(val: SensorBlackLevels) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for SensorBlackLevels {
    const ID: u32 = ControlId::SensorBlackLevels as _;
}
impl Control for SensorBlackLevels {}
/// Intensity of the sharpening applied to the image.
///
/// A value of 0.0 means no sharpening. The minimum value means
/// minimal sharpening, and shall be 0.0 unless the camera can't
/// disable sharpening completely. The default value shall give a
/// "reasonable" level of sharpening, suitable for most use cases.
/// The maximum value may apply extremely high levels of sharpening,
/// higher than anyone could reasonably want. Negative values are
/// not allowed. Note also that sharpening is not applied to raw
/// streams.
#[derive(Debug, Clone)]
pub struct Sharpness(pub f32);
impl Deref for Sharpness {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Sharpness {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for Sharpness {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<f32>::try_from(value)?))
    }
}
impl From<Sharpness> for ControlValue {
    fn from(val: Sharpness) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for Sharpness {
    const ID: u32 = ControlId::Sharpness as _;
}
impl Control for Sharpness {}
/// Reports a Figure of Merit (FoM) to indicate how in-focus the frame is.
///
/// A larger FocusFoM value indicates a more in-focus frame. This singular
/// value may be based on a combination of statistics gathered from
/// multiple focus regions within an image. The number of focus regions and
/// method of combination is platform dependent. In this respect, it is not
/// necessarily aimed at providing a way to implement a focus algorithm by
/// the application, rather an indication of how in-focus a frame is.
#[derive(Debug, Clone)]
pub struct FocusFoM(pub i32);
impl Deref for FocusFoM {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for FocusFoM {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for FocusFoM {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<i32>::try_from(value)?))
    }
}
impl From<FocusFoM> for ControlValue {
    fn from(val: FocusFoM) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for FocusFoM {
    const ID: u32 = ControlId::FocusFoM as _;
}
impl Control for FocusFoM {}
/// The 3x3 matrix that converts camera RGB to sRGB within the imaging
/// pipeline.
///
/// This should describe the matrix that is used after pixels have been
/// white-balanced, but before any gamma transformation. The 3x3 matrix is
/// stored in conventional reading order in an array of 9 floating point
/// values.
///
/// ColourCorrectionMatrix can only be applied in a Request when the AWB is
/// disabled.
///
/// \sa AwbEnable
/// \sa ColourTemperature
#[derive(Debug, Clone)]
pub struct ColourCorrectionMatrix(pub [[f32; 3]; 3]);
impl Deref for ColourCorrectionMatrix {
    type Target = [[f32; 3]; 3];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for ColourCorrectionMatrix {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for ColourCorrectionMatrix {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<[[f32; 3]; 3]>::try_from(value)?))
    }
}
impl From<ColourCorrectionMatrix> for ControlValue {
    fn from(val: ColourCorrectionMatrix) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for ColourCorrectionMatrix {
    const ID: u32 = ControlId::ColourCorrectionMatrix as _;
}
impl Control for ColourCorrectionMatrix {}
/// Sets the image portion that will be scaled to form the whole of
/// the final output image.
///
/// The (x,y) location of this rectangle is relative to the
/// PixelArrayActiveAreas that is being used. The units remain native
/// sensor pixels, even if the sensor is being used in a binning or
/// skipping mode.
///
/// This control is only present when the pipeline supports scaling. Its
/// maximum valid value is given by the properties::ScalerCropMaximum
/// property, and the two can be used to implement digital zoom.
#[derive(Debug, Clone)]
pub struct ScalerCrop(pub Rectangle);
impl Deref for ScalerCrop {
    type Target = Rectangle;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for ScalerCrop {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for ScalerCrop {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<Rectangle>::try_from(value)?))
    }
}
impl From<ScalerCrop> for ControlValue {
    fn from(val: ScalerCrop) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for ScalerCrop {
    const ID: u32 = ControlId::ScalerCrop as _;
}
impl Control for ScalerCrop {}
/// Digital gain value applied during the processing steps applied
/// to the image as captured from the sensor.
///
/// The global digital gain factor is applied to all the colour channels
/// of the RAW image. Different pipeline models are free to
/// specify how the global gain factor applies to each separate
/// channel.
///
/// If an imaging pipeline applies digital gain in distinct
/// processing steps, this value indicates their total sum.
/// Pipelines are free to decide how to adjust each processing
/// step to respect the received gain factor and shall report
/// their total value in the request metadata.
#[derive(Debug, Clone)]
pub struct DigitalGain(pub f32);
impl Deref for DigitalGain {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for DigitalGain {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for DigitalGain {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<f32>::try_from(value)?))
    }
}
impl From<DigitalGain> for ControlValue {
    fn from(val: DigitalGain) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for DigitalGain {
    const ID: u32 = ControlId::DigitalGain as _;
}
impl Control for DigitalGain {}
/// The instantaneous frame duration from start of frame exposure to start
/// of next exposure, expressed in microseconds.
///
/// This control is meant to be returned in metadata.
#[derive(Debug, Clone)]
pub struct FrameDuration(pub i64);
impl Deref for FrameDuration {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for FrameDuration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for FrameDuration {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<i64>::try_from(value)?))
    }
}
impl From<FrameDuration> for ControlValue {
    fn from(val: FrameDuration) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for FrameDuration {
    const ID: u32 = ControlId::FrameDuration as _;
}
impl Control for FrameDuration {}
/// The minimum and maximum (in that order) frame duration, expressed in
/// microseconds.
///
/// When provided by applications, the control specifies the sensor frame
/// duration interval the pipeline has to use. This limits the largest
/// exposure time the sensor can use. For example, if a maximum frame
/// duration of 33ms is requested (corresponding to 30 frames per second),
/// the sensor will not be able to raise the exposure time above 33ms.
/// A fixed frame duration is achieved by setting the minimum and maximum
/// values to be the same. Setting both values to 0 reverts to using the
/// camera defaults.
///
/// The maximum frame duration provides the absolute limit to the exposure
/// time computed by the AE algorithm and it overrides any exposure mode
/// setting specified with controls::AeExposureMode. Similarly, when a
/// manual exposure time is set through controls::ExposureTime, it also
/// gets clipped to the limits set by this control. When reported in
/// metadata, the control expresses the minimum and maximum frame durations
/// used after being clipped to the sensor provided frame duration limits.
///
/// \sa AeExposureMode
/// \sa ExposureTime
///
/// \todo Define how to calculate the capture frame rate by
/// defining controls to report additional delays introduced by
/// the capture pipeline or post-processing stages (ie JPEG
/// conversion, frame scaling).
///
/// \todo Provide an explicit definition of default control values, for
/// this and all other controls.
#[derive(Debug, Clone)]
pub struct FrameDurationLimits(pub [i64; 2]);
impl Deref for FrameDurationLimits {
    type Target = [i64; 2];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for FrameDurationLimits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for FrameDurationLimits {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<[i64; 2]>::try_from(value)?))
    }
}
impl From<FrameDurationLimits> for ControlValue {
    fn from(val: FrameDurationLimits) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for FrameDurationLimits {
    const ID: u32 = ControlId::FrameDurationLimits as _;
}
impl Control for FrameDurationLimits {}
/// Temperature measure from the camera sensor in Celsius.
///
/// This value is typically obtained by a thermal sensor present on-die or
/// in the camera module. The range of reported temperatures is device
/// dependent.
///
/// The SensorTemperature control will only be returned in metadata if a
/// thermal sensor is present.
#[derive(Debug, Clone)]
pub struct SensorTemperature(pub f32);
impl Deref for SensorTemperature {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for SensorTemperature {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for SensorTemperature {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<f32>::try_from(value)?))
    }
}
impl From<SensorTemperature> for ControlValue {
    fn from(val: SensorTemperature) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for SensorTemperature {
    const ID: u32 = ControlId::SensorTemperature as _;
}
impl Control for SensorTemperature {}
/// The time when the first row of the image sensor active array is exposed.
///
/// The timestamp, expressed in nanoseconds, represents a monotonically
/// increasing counter since the system boot time, as defined by the
/// Linux-specific CLOCK_BOOTTIME clock id.
///
/// The SensorTimestamp control can only be returned in metadata.
///
/// \todo Define how the sensor timestamp has to be used in the reprocessing
/// use case.
#[derive(Debug, Clone)]
pub struct SensorTimestamp(pub i64);
impl Deref for SensorTimestamp {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for SensorTimestamp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for SensorTimestamp {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<i64>::try_from(value)?))
    }
}
impl From<SensorTimestamp> for ControlValue {
    fn from(val: SensorTimestamp) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for SensorTimestamp {
    const ID: u32 = ControlId::SensorTimestamp as _;
}
impl Control for SensorTimestamp {}
/// The mode of the AF (autofocus) algorithm.
///
/// An implementation may choose not to implement all the modes.
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum AfMode {
    /// The AF algorithm is in manual mode.
    ///
    /// In this mode it will never perform any action nor move the lens of
    /// its own accord, but an application can specify the desired lens
    /// position using the LensPosition control. The AfState will always
    /// report AfStateIdle.
    ///
    /// If the camera is started in AfModeManual, it will move the focus
    /// lens to the position specified by the LensPosition control.
    ///
    /// This mode is the recommended default value for the AfMode control.
    /// External cameras (as reported by the Location property set to
    /// CameraLocationExternal) may use a different default value.
    Manual = 0,
    /// The AF algorithm is in auto mode.
    ///
    /// In this mode the algorithm will never move the lens or change state
    /// unless the AfTrigger control is used. The AfTrigger control can be
    /// used to initiate a focus scan, the results of which will be
    /// reported by AfState.
    ///
    /// If the autofocus algorithm is moved from AfModeAuto to another mode
    /// while a scan is in progress, the scan is cancelled immediately,
    /// without waiting for the scan to finish.
    ///
    /// When first entering this mode the AfState will report AfStateIdle.
    /// When a trigger control is sent, AfState will report AfStateScanning
    /// for a period before spontaneously changing to AfStateFocused or
    /// AfStateFailed, depending on the outcome of the scan. It will remain
    /// in this state until another scan is initiated by the AfTrigger
    /// control. If a scan is cancelled (without changing to another mode),
    /// AfState will return to AfStateIdle.
    Auto = 1,
    /// The AF algorithm is in continuous mode.
    ///
    /// In this mode the lens can re-start a scan spontaneously at any
    /// moment, without any user intervention. The AfState still reports
    /// whether the algorithm is currently scanning or not, though the
    /// application has no ability to initiate or cancel scans, nor to move
    /// the lens for itself.
    ///
    /// However, applications can pause the AF algorithm from continuously
    /// scanning by using the AfPause control. This allows video or still
    /// images to be captured whilst guaranteeing that the focus is fixed.
    ///
    /// When set to AfModeContinuous, the system will immediately initiate a
    /// scan so AfState will report AfStateScanning, and will settle on one
    /// of AfStateFocused or AfStateFailed, depending on the scan result.
    Continuous = 2,
}
impl TryFrom<ControlValue> for AfMode {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Self::try_from(i32::try_from(value.clone())?)
            .map_err(|_| ControlValueError::UnknownVariant(value))
    }
}
impl From<AfMode> for ControlValue {
    fn from(val: AfMode) -> Self {
        ControlValue::from(<i32>::from(val))
    }
}
impl ControlEntry for AfMode {
    const ID: u32 = ControlId::AfMode as _;
}
impl Control for AfMode {}
/// The range of focus distances that is scanned.
///
/// An implementation may choose not to implement all the options here.
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum AfRange {
    /// A wide range of focus distances is scanned.
    ///
    /// Scanned distances cover all the way from infinity down to close
    /// distances, though depending on the implementation, possibly not
    /// including the very closest macro positions.
    Normal = 0,
    /// Only close distances are scanned.
    Macro = 1,
    /// The full range of focus distances is scanned.
    ///
    /// This range is similar to AfRangeNormal but includes the very
    /// closest macro positions.
    Full = 2,
}
impl TryFrom<ControlValue> for AfRange {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Self::try_from(i32::try_from(value.clone())?)
            .map_err(|_| ControlValueError::UnknownVariant(value))
    }
}
impl From<AfRange> for ControlValue {
    fn from(val: AfRange) -> Self {
        ControlValue::from(<i32>::from(val))
    }
}
impl ControlEntry for AfRange {
    const ID: u32 = ControlId::AfRange as _;
}
impl Control for AfRange {}
/// Determine whether the AF is to move the lens as quickly as possible or
/// more steadily.
///
/// For example, during video recording it may be desirable not to move the
/// lens too abruptly, but when in a preview mode (waiting for a still
/// capture) it may be helpful to move the lens as quickly as is reasonably
/// possible.
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum AfSpeed {
    /// Move the lens at its usual speed.
    Normal = 0,
    /// Move the lens more quickly.
    Fast = 1,
}
impl TryFrom<ControlValue> for AfSpeed {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Self::try_from(i32::try_from(value.clone())?)
            .map_err(|_| ControlValueError::UnknownVariant(value))
    }
}
impl From<AfSpeed> for ControlValue {
    fn from(val: AfSpeed) -> Self {
        ControlValue::from(<i32>::from(val))
    }
}
impl ControlEntry for AfSpeed {
    const ID: u32 = ControlId::AfSpeed as _;
}
impl Control for AfSpeed {}
/// The parts of the image used by the AF algorithm to measure focus.
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum AfMetering {
    /// Let the AF algorithm decide for itself where it will measure focus.
    Auto = 0,
    /// Use the rectangles defined by the AfWindows control to measure focus.
    ///
    /// If no windows are specified the behaviour is platform dependent.
    Windows = 1,
}
impl TryFrom<ControlValue> for AfMetering {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Self::try_from(i32::try_from(value.clone())?)
            .map_err(|_| ControlValueError::UnknownVariant(value))
    }
}
impl From<AfMetering> for ControlValue {
    fn from(val: AfMetering) -> Self {
        ControlValue::from(<i32>::from(val))
    }
}
impl ControlEntry for AfMetering {
    const ID: u32 = ControlId::AfMetering as _;
}
impl Control for AfMetering {}
/// The focus windows used by the AF algorithm when AfMetering is set to
/// AfMeteringWindows.
///
/// The units used are pixels within the rectangle returned by the
/// ScalerCropMaximum property.
///
/// In order to be activated, a rectangle must be programmed with non-zero
/// width and height. Internally, these rectangles are intersected with the
/// ScalerCropMaximum rectangle. If the window becomes empty after this
/// operation, then the window is ignored. If all the windows end up being
/// ignored, then the behaviour is platform dependent.
///
/// On platforms that support the ScalerCrop control (for implementing
/// digital zoom, for example), no automatic recalculation or adjustment of
/// AF windows is performed internally if the ScalerCrop is changed. If any
/// window lies outside the output image after the scaler crop has been
/// applied, it is up to the application to recalculate them.
///
/// The details of how the windows are used are platform dependent. We note
/// that when there is more than one AF window, a typical implementation
/// might find the optimal focus position for each one and finally select
/// the window where the focal distance for the objects shown in that part
/// of the image are closest to the camera.
#[derive(Debug, Clone)]
pub struct AfWindows(pub Vec<Rectangle>);
impl Deref for AfWindows {
    type Target = Vec<Rectangle>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for AfWindows {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for AfWindows {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<Vec<Rectangle>>::try_from(value)?))
    }
}
impl From<AfWindows> for ControlValue {
    fn from(val: AfWindows) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for AfWindows {
    const ID: u32 = ControlId::AfWindows as _;
}
impl Control for AfWindows {}
/// Start an autofocus scan.
///
/// This control starts an autofocus scan when AfMode is set to AfModeAuto,
/// and is ignored if AfMode is set to AfModeManual or AfModeContinuous. It
/// can also be used to terminate a scan early.
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum AfTrigger {
    /// Start an AF scan.
    ///
    /// Setting the control to AfTriggerStart is ignored if a scan is in
    /// progress.
    Start = 0,
    /// Cancel an AF scan.
    ///
    /// This does not cause the lens to move anywhere else. Ignored if no
    /// scan is in progress.
    Cancel = 1,
}
impl TryFrom<ControlValue> for AfTrigger {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Self::try_from(i32::try_from(value.clone())?)
            .map_err(|_| ControlValueError::UnknownVariant(value))
    }
}
impl From<AfTrigger> for ControlValue {
    fn from(val: AfTrigger) -> Self {
        ControlValue::from(<i32>::from(val))
    }
}
impl ControlEntry for AfTrigger {
    const ID: u32 = ControlId::AfTrigger as _;
}
impl Control for AfTrigger {}
/// Pause lens movements when in continuous autofocus mode.
///
/// This control has no effect except when in continuous autofocus mode
/// (AfModeContinuous). It can be used to pause any lens movements while
/// (for example) images are captured. The algorithm remains inactive
/// until it is instructed to resume.
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum AfPause {
    /// Pause the continuous autofocus algorithm immediately.
    ///
    /// The autofocus algorithm is paused whether or not any kind of scan
    /// is underway. AfPauseState will subsequently report
    /// AfPauseStatePaused. AfState may report any of AfStateScanning,
    /// AfStateFocused or AfStateFailed, depending on the algorithm's state
    /// when it received this control.
    Immediate = 0,
    /// Pause the continuous autofocus algorithm at the end of the scan.
    ///
    /// This is similar to AfPauseImmediate, and if the AfState is
    /// currently reporting AfStateFocused or AfStateFailed it will remain
    /// in that state and AfPauseState will report AfPauseStatePaused.
    ///
    /// However, if the algorithm is scanning (AfStateScanning),
    /// AfPauseState will report AfPauseStatePausing until the scan is
    /// finished, at which point AfState will report one of AfStateFocused
    /// or AfStateFailed, and AfPauseState will change to
    /// AfPauseStatePaused.
    Deferred = 1,
    /// Resume continuous autofocus operation.
    ///
    /// The algorithm starts again from exactly where it left off, and
    /// AfPauseState will report AfPauseStateRunning.
    Resume = 2,
}
impl TryFrom<ControlValue> for AfPause {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Self::try_from(i32::try_from(value.clone())?)
            .map_err(|_| ControlValueError::UnknownVariant(value))
    }
}
impl From<AfPause> for ControlValue {
    fn from(val: AfPause) -> Self {
        ControlValue::from(<i32>::from(val))
    }
}
impl ControlEntry for AfPause {
    const ID: u32 = ControlId::AfPause as _;
}
impl Control for AfPause {}
/// Set and report the focus lens position.
///
/// This control instructs the lens to move to a particular position and
/// also reports back the position of the lens for each frame.
///
/// The LensPosition control is ignored unless the AfMode is set to
/// AfModeManual, though the value is reported back unconditionally in all
/// modes.
///
/// This value, which is generally a non-integer, is the reciprocal of the
/// focal distance in metres, also known as dioptres. That is, to set a
/// focal distance D, the lens position LP is given by
///
/// \f$LP = \frac{1\mathrm{m}}{D}\f$
///
/// For example:
///
/// - 0 moves the lens to infinity.
/// - 0.5 moves the lens to focus on objects 2m away.
/// - 2 moves the lens to focus on objects 50cm away.
/// - And larger values will focus the lens closer.
///
/// The default value of the control should indicate a good general
/// position for the lens, often corresponding to the hyperfocal distance
/// (the closest position for which objects at infinity are still
/// acceptably sharp). The minimum will often be zero (meaning infinity),
/// and the maximum value defines the closest focus position.
///
/// \todo Define a property to report the Hyperfocal distance of calibrated
/// lenses.
#[derive(Debug, Clone)]
pub struct LensPosition(pub f32);
impl Deref for LensPosition {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for LensPosition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for LensPosition {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<f32>::try_from(value)?))
    }
}
impl From<LensPosition> for ControlValue {
    fn from(val: LensPosition) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for LensPosition {
    const ID: u32 = ControlId::LensPosition as _;
}
impl Control for LensPosition {}
/// The current state of the AF algorithm.
///
/// This control reports the current state of the AF algorithm in
/// conjunction with the reported AfMode value and (in continuous AF mode)
/// the AfPauseState value. The possible state changes are described below,
/// though we note the following state transitions that occur when the
/// AfMode is changed.
///
/// If the AfMode is set to AfModeManual, then the AfState will always
/// report AfStateIdle (even if the lens is subsequently moved). Changing
/// to the AfModeManual state does not initiate any lens movement.
///
/// If the AfMode is set to AfModeAuto then the AfState will report
/// AfStateIdle. However, if AfModeAuto and AfTriggerStart are sent
/// together then AfState will omit AfStateIdle and move straight to
/// AfStateScanning (and start a scan).
///
/// If the AfMode is set to AfModeContinuous then the AfState will
/// initially report AfStateScanning.
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum AfState {
    /// The AF algorithm is in manual mode (AfModeManual) or in auto mode
    /// (AfModeAuto) and a scan has not yet been triggered, or an
    /// in-progress scan was cancelled.
    Idle = 0,
    /// The AF algorithm is in auto mode (AfModeAuto), and a scan has been
    /// started using the AfTrigger control.
    ///
    /// The scan can be cancelled by sending AfTriggerCancel at which point
    /// the algorithm will either move back to AfStateIdle or, if the scan
    /// actually completes before the cancel request is processed, to one
    /// of AfStateFocused or AfStateFailed.
    ///
    /// Alternatively the AF algorithm could be in continuous mode
    /// (AfModeContinuous) at which point it may enter this state
    /// spontaneously whenever it determines that a rescan is needed.
    Scanning = 1,
    /// The AF algorithm is in auto (AfModeAuto) or continuous
    /// (AfModeContinuous) mode and a scan has completed with the result
    /// that the algorithm believes the image is now in focus.
    Focused = 2,
    /// The AF algorithm is in auto (AfModeAuto) or continuous
    /// (AfModeContinuous) mode and a scan has completed with the result
    /// that the algorithm did not find a good focus position.
    Failed = 3,
}
impl TryFrom<ControlValue> for AfState {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Self::try_from(i32::try_from(value.clone())?)
            .map_err(|_| ControlValueError::UnknownVariant(value))
    }
}
impl From<AfState> for ControlValue {
    fn from(val: AfState) -> Self {
        ControlValue::from(<i32>::from(val))
    }
}
impl ControlEntry for AfState {
    const ID: u32 = ControlId::AfState as _;
}
impl Control for AfState {}
/// Report whether the autofocus is currently running, paused or pausing.
///
/// This control is only applicable in continuous (AfModeContinuous) mode,
/// and reports whether the algorithm is currently running, paused or
/// pausing (that is, will pause as soon as any in-progress scan
/// completes).
///
/// Any change to AfMode will cause AfPauseStateRunning to be reported.
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum AfPauseState {
    /// Continuous AF is running and the algorithm may restart a scan
    /// spontaneously.
    Running = 0,
    /// Continuous AF has been sent an AfPauseDeferred control, and will
    /// pause as soon as any in-progress scan completes.
    ///
    /// When the scan completes, the AfPauseState control will report
    /// AfPauseStatePaused. No new scans will be start spontaneously until
    /// the AfPauseResume control is sent.
    Pausing = 1,
    /// Continuous AF is paused.
    ///
    /// No further state changes or lens movements will occur until the
    /// AfPauseResume control is sent.
    Paused = 2,
}
impl TryFrom<ControlValue> for AfPauseState {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Self::try_from(i32::try_from(value.clone())?)
            .map_err(|_| ControlValueError::UnknownVariant(value))
    }
}
impl From<AfPauseState> for ControlValue {
    fn from(val: AfPauseState) -> Self {
        ControlValue::from(<i32>::from(val))
    }
}
impl ControlEntry for AfPauseState {
    const ID: u32 = ControlId::AfPauseState as _;
}
impl Control for AfPauseState {}
/// Set the mode to be used for High Dynamic Range (HDR) imaging.
///
/// HDR techniques typically include multiple exposure, image fusion and
/// tone mapping techniques to improve the dynamic range of the resulting
/// images.
///
/// When using an HDR mode, images are captured with different sets of AGC
/// settings called HDR channels. Channels indicate in particular the type
/// of exposure (short, medium or long) used to capture the raw image,
/// before fusion. Each HDR image is tagged with the corresponding channel
/// using the HdrChannel control.
///
/// \sa HdrChannel
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum HdrMode {
    /// HDR is disabled.
    ///
    /// Metadata for this frame will not include the HdrChannel control.
    Off = 0,
    /// Multiple exposures will be generated in an alternating fashion.
    ///
    /// The multiple exposures will not be merged together and will be
    /// returned to the application as they are. Each image will be tagged
    /// with the correct HDR channel, indicating what kind of exposure it
    /// is. The tag should be the same as in the HdrModeMultiExposure case.
    ///
    /// The expectation is that an application using this mode would merge
    /// the frames to create HDR images for itself if it requires them.
    MultiExposureUnmerged = 1,
    /// Multiple exposures will be generated and merged to create HDR
    /// images.
    ///
    /// Each image will be tagged with the HDR channel (long, medium or
    /// short) that arrived and which caused this image to be output.
    ///
    /// Systems that use two channels for HDR will return images tagged
    /// alternately as the short and long channel. Systems that use three
    /// channels for HDR will cycle through the short, medium and long
    /// channel before repeating.
    MultiExposure = 2,
    /// Multiple frames all at a single exposure will be used to create HDR
    /// images.
    ///
    /// These images should be reported as all corresponding to the HDR
    /// short channel.
    SingleExposure = 3,
    /// Multiple frames will be combined to produce "night mode" images.
    ///
    /// It is up to the implementation exactly which HDR channels it uses,
    /// and the images will all be tagged accordingly with the correct HDR
    /// channel information.
    Night = 4,
}
impl TryFrom<ControlValue> for HdrMode {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Self::try_from(i32::try_from(value.clone())?)
            .map_err(|_| ControlValueError::UnknownVariant(value))
    }
}
impl From<HdrMode> for ControlValue {
    fn from(val: HdrMode) -> Self {
        ControlValue::from(<i32>::from(val))
    }
}
impl ControlEntry for HdrMode {
    const ID: u32 = ControlId::HdrMode as _;
}
impl Control for HdrMode {}
/// The HDR channel used to capture the frame.
///
/// This value is reported back to the application so that it can discover
/// whether this capture corresponds to the short or long exposure image
/// (or any other image used by the HDR procedure). An application can
/// monitor the HDR channel to discover when the differently exposed images
/// have arrived.
///
/// This metadata is only available when an HDR mode has been enabled.
///
/// \sa HdrMode
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum HdrChannel {
    /// This image does not correspond to any of the captures used to create
    /// an HDR image.
    None = 0,
    /// This is a short exposure image.
    Short = 1,
    /// This is a medium exposure image.
    Medium = 2,
    /// This is a long exposure image.
    Long = 3,
}
impl TryFrom<ControlValue> for HdrChannel {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Self::try_from(i32::try_from(value.clone())?)
            .map_err(|_| ControlValueError::UnknownVariant(value))
    }
}
impl From<HdrChannel> for ControlValue {
    fn from(val: HdrChannel) -> Self {
        ControlValue::from(<i32>::from(val))
    }
}
impl ControlEntry for HdrChannel {
    const ID: u32 = ControlId::HdrChannel as _;
}
impl Control for HdrChannel {}
/// Specify a fixed gamma value.
///
/// The default gamma value must be 2.2 which closely mimics sRGB gamma.
/// Note that this is camera gamma, so it is applied as 1.0/gamma.
#[derive(Debug, Clone)]
pub struct Gamma(pub f32);
impl Deref for Gamma {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Gamma {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for Gamma {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<f32>::try_from(value)?))
    }
}
impl From<Gamma> for ControlValue {
    fn from(val: Gamma) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for Gamma {
    const ID: u32 = ControlId::Gamma as _;
}
impl Control for Gamma {}
/// Enable or disable the debug metadata.
#[derive(Debug, Clone)]
pub struct DebugMetadataEnable(pub bool);
impl Deref for DebugMetadataEnable {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for DebugMetadataEnable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for DebugMetadataEnable {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<bool>::try_from(value)?))
    }
}
impl From<DebugMetadataEnable> for ControlValue {
    fn from(val: DebugMetadataEnable) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for DebugMetadataEnable {
    const ID: u32 = ControlId::DebugMetadataEnable as _;
}
impl Control for DebugMetadataEnable {}
/// Control for AE metering trigger. Currently identical to
/// ANDROID_CONTROL_AE_PRECAPTURE_TRIGGER.
///
/// Whether the camera device will trigger a precapture metering sequence
/// when it processes this request.
#[cfg(feature = "vendor_draft")]
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum AePrecaptureTrigger {
    /// The trigger is idle.
    Idle = 0,
    /// The pre-capture AE metering is started by the camera.
    Start = 1,
    /// The camera will cancel any active or completed metering sequence.
    /// The AE algorithm is reset to its initial state.
    Cancel = 2,
}
#[cfg(feature = "vendor_draft")]
impl TryFrom<ControlValue> for AePrecaptureTrigger {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Self::try_from(i32::try_from(value.clone())?)
            .map_err(|_| ControlValueError::UnknownVariant(value))
    }
}
#[cfg(feature = "vendor_draft")]
impl From<AePrecaptureTrigger> for ControlValue {
    fn from(val: AePrecaptureTrigger) -> Self {
        ControlValue::from(<i32>::from(val))
    }
}
#[cfg(feature = "vendor_draft")]
impl ControlEntry for AePrecaptureTrigger {
    const ID: u32 = ControlId::AePrecaptureTrigger as _;
}
#[cfg(feature = "vendor_draft")]
impl Control for AePrecaptureTrigger {}
/// Control to select the noise reduction algorithm mode. Currently
/// identical to ANDROID_NOISE_REDUCTION_MODE.
///
///  Mode of operation for the noise reduction algorithm.
#[cfg(feature = "vendor_draft")]
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum NoiseReductionMode {
    /// No noise reduction is applied
    Off = 0,
    /// Noise reduction is applied without reducing the frame rate.
    Fast = 1,
    /// High quality noise reduction at the expense of frame rate.
    HighQuality = 2,
    /// Minimal noise reduction is applied without reducing the frame rate.
    Minimal = 3,
    /// Noise reduction is applied at different levels to different streams.
    ZSL = 4,
}
#[cfg(feature = "vendor_draft")]
impl TryFrom<ControlValue> for NoiseReductionMode {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Self::try_from(i32::try_from(value.clone())?)
            .map_err(|_| ControlValueError::UnknownVariant(value))
    }
}
#[cfg(feature = "vendor_draft")]
impl From<NoiseReductionMode> for ControlValue {
    fn from(val: NoiseReductionMode) -> Self {
        ControlValue::from(<i32>::from(val))
    }
}
#[cfg(feature = "vendor_draft")]
impl ControlEntry for NoiseReductionMode {
    const ID: u32 = ControlId::NoiseReductionMode as _;
}
#[cfg(feature = "vendor_draft")]
impl Control for NoiseReductionMode {}
/// Control to select the color correction aberration mode. Currently
/// identical to ANDROID_COLOR_CORRECTION_ABERRATION_MODE.
///
///  Mode of operation for the chromatic aberration correction algorithm.
#[cfg(feature = "vendor_draft")]
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ColorCorrectionAberrationMode {
    /// No aberration correction is applied.
    ColorCorrectionAberrationOff = 0,
    /// Aberration correction will not slow down the frame rate.
    ColorCorrectionAberrationFast = 1,
    /// High quality aberration correction which might reduce the frame
    /// rate.
    ColorCorrectionAberrationHighQuality = 2,
}
#[cfg(feature = "vendor_draft")]
impl TryFrom<ControlValue> for ColorCorrectionAberrationMode {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Self::try_from(i32::try_from(value.clone())?)
            .map_err(|_| ControlValueError::UnknownVariant(value))
    }
}
#[cfg(feature = "vendor_draft")]
impl From<ColorCorrectionAberrationMode> for ControlValue {
    fn from(val: ColorCorrectionAberrationMode) -> Self {
        ControlValue::from(<i32>::from(val))
    }
}
#[cfg(feature = "vendor_draft")]
impl ControlEntry for ColorCorrectionAberrationMode {
    const ID: u32 = ControlId::ColorCorrectionAberrationMode as _;
}
#[cfg(feature = "vendor_draft")]
impl Control for ColorCorrectionAberrationMode {}
/// Control to report the current AE algorithm state. Currently identical to
/// ANDROID_CONTROL_AE_STATE.
///
///  Current state of the AE algorithm.
#[cfg(feature = "vendor_draft")]
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum AeState {
    /// The AE algorithm is inactive.
    Inactive = 0,
    /// The AE algorithm has not converged yet.
    Searching = 1,
    /// The AE algorithm has converged.
    Converged = 2,
    /// The AE algorithm is locked.
    Locked = 3,
    /// The AE algorithm would need a flash for good results
    FlashRequired = 4,
    /// The AE algorithm has started a pre-capture metering session.
    /// \sa AePrecaptureTrigger
    Precapture = 5,
}
#[cfg(feature = "vendor_draft")]
impl TryFrom<ControlValue> for AeState {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Self::try_from(i32::try_from(value.clone())?)
            .map_err(|_| ControlValueError::UnknownVariant(value))
    }
}
#[cfg(feature = "vendor_draft")]
impl From<AeState> for ControlValue {
    fn from(val: AeState) -> Self {
        ControlValue::from(<i32>::from(val))
    }
}
#[cfg(feature = "vendor_draft")]
impl ControlEntry for AeState {
    const ID: u32 = ControlId::AeState as _;
}
#[cfg(feature = "vendor_draft")]
impl Control for AeState {}
/// Control to report the current AWB algorithm state. Currently identical
/// to ANDROID_CONTROL_AWB_STATE.
///
///  Current state of the AWB algorithm.
#[cfg(feature = "vendor_draft")]
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum AwbState {
    /// The AWB algorithm is inactive.
    Inactive = 0,
    /// The AWB algorithm has not converged yet.
    Searching = 1,
    /// The AWB algorithm has converged.
    AwbConverged = 2,
    /// The AWB algorithm is locked.
    AwbLocked = 3,
}
#[cfg(feature = "vendor_draft")]
impl TryFrom<ControlValue> for AwbState {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Self::try_from(i32::try_from(value.clone())?)
            .map_err(|_| ControlValueError::UnknownVariant(value))
    }
}
#[cfg(feature = "vendor_draft")]
impl From<AwbState> for ControlValue {
    fn from(val: AwbState) -> Self {
        ControlValue::from(<i32>::from(val))
    }
}
#[cfg(feature = "vendor_draft")]
impl ControlEntry for AwbState {
    const ID: u32 = ControlId::AwbState as _;
}
#[cfg(feature = "vendor_draft")]
impl Control for AwbState {}
/// Control to report the time between the start of exposure of the first
/// row and the start of exposure of the last row. Currently identical to
/// ANDROID_SENSOR_ROLLING_SHUTTER_SKEW
#[cfg(feature = "vendor_draft")]
#[derive(Debug, Clone)]
pub struct SensorRollingShutterSkew(pub i64);
#[cfg(feature = "vendor_draft")]
impl Deref for SensorRollingShutterSkew {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(feature = "vendor_draft")]
impl DerefMut for SensorRollingShutterSkew {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[cfg(feature = "vendor_draft")]
impl TryFrom<ControlValue> for SensorRollingShutterSkew {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<i64>::try_from(value)?))
    }
}
#[cfg(feature = "vendor_draft")]
impl From<SensorRollingShutterSkew> for ControlValue {
    fn from(val: SensorRollingShutterSkew) -> Self {
        ControlValue::from(val.0)
    }
}
#[cfg(feature = "vendor_draft")]
impl ControlEntry for SensorRollingShutterSkew {
    const ID: u32 = ControlId::SensorRollingShutterSkew as _;
}
#[cfg(feature = "vendor_draft")]
impl Control for SensorRollingShutterSkew {}
/// Control to report if the lens shading map is available. Currently
/// identical to ANDROID_STATISTICS_LENS_SHADING_MAP_MODE.
#[cfg(feature = "vendor_draft")]
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum LensShadingMapMode {
    /// No lens shading map mode is available.
    Off = 0,
    /// The lens shading map mode is available.
    On = 1,
}
#[cfg(feature = "vendor_draft")]
impl TryFrom<ControlValue> for LensShadingMapMode {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Self::try_from(i32::try_from(value.clone())?)
            .map_err(|_| ControlValueError::UnknownVariant(value))
    }
}
#[cfg(feature = "vendor_draft")]
impl From<LensShadingMapMode> for ControlValue {
    fn from(val: LensShadingMapMode) -> Self {
        ControlValue::from(<i32>::from(val))
    }
}
#[cfg(feature = "vendor_draft")]
impl ControlEntry for LensShadingMapMode {
    const ID: u32 = ControlId::LensShadingMapMode as _;
}
#[cfg(feature = "vendor_draft")]
impl Control for LensShadingMapMode {}
/// Specifies the number of pipeline stages the frame went through from when
/// it was exposed to when the final completed result was available to the
/// framework. Always less than or equal to PipelineMaxDepth. Currently
/// identical to ANDROID_REQUEST_PIPELINE_DEPTH.
///
/// The typical value for this control is 3 as a frame is first exposed,
/// captured and then processed in a single pass through the ISP. Any
/// additional processing step performed after the ISP pass (in example face
/// detection, additional format conversions etc) count as an additional
/// pipeline stage.
#[cfg(feature = "vendor_draft")]
#[derive(Debug, Clone)]
pub struct PipelineDepth(pub i32);
#[cfg(feature = "vendor_draft")]
impl Deref for PipelineDepth {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(feature = "vendor_draft")]
impl DerefMut for PipelineDepth {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[cfg(feature = "vendor_draft")]
impl TryFrom<ControlValue> for PipelineDepth {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<i32>::try_from(value)?))
    }
}
#[cfg(feature = "vendor_draft")]
impl From<PipelineDepth> for ControlValue {
    fn from(val: PipelineDepth) -> Self {
        ControlValue::from(val.0)
    }
}
#[cfg(feature = "vendor_draft")]
impl ControlEntry for PipelineDepth {
    const ID: u32 = ControlId::PipelineDepth as _;
}
#[cfg(feature = "vendor_draft")]
impl Control for PipelineDepth {}
/// The maximum number of frames that can occur after a request (different
/// than the previous) has been submitted, and before the result's state
/// becomes synchronized. A value of -1 indicates unknown latency, and 0
/// indicates per-frame control. Currently identical to
/// ANDROID_SYNC_MAX_LATENCY.
#[cfg(feature = "vendor_draft")]
#[derive(Debug, Clone)]
pub struct MaxLatency(pub i32);
#[cfg(feature = "vendor_draft")]
impl Deref for MaxLatency {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(feature = "vendor_draft")]
impl DerefMut for MaxLatency {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[cfg(feature = "vendor_draft")]
impl TryFrom<ControlValue> for MaxLatency {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<i32>::try_from(value)?))
    }
}
#[cfg(feature = "vendor_draft")]
impl From<MaxLatency> for ControlValue {
    fn from(val: MaxLatency) -> Self {
        ControlValue::from(val.0)
    }
}
#[cfg(feature = "vendor_draft")]
impl ControlEntry for MaxLatency {
    const ID: u32 = ControlId::MaxLatency as _;
}
#[cfg(feature = "vendor_draft")]
impl Control for MaxLatency {}
/// Control to select the test pattern mode. Currently identical to
/// ANDROID_SENSOR_TEST_PATTERN_MODE.
#[cfg(feature = "vendor_draft")]
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum TestPatternMode {
    /// No test pattern mode is used. The camera device returns frames from
    /// the image sensor.
    Off = 0,
    /// Each pixel in [R, G_even, G_odd, B] is replaced by its respective
    /// color channel provided in test pattern data.
    /// \todo Add control for test pattern data.
    SolidColor = 1,
    /// All pixel data is replaced with an 8-bar color pattern. The vertical
    /// bars (left-to-right) are as follows; white, yellow, cyan, green,
    /// magenta, red, blue and black. Each bar should take up 1/8 of the
    /// sensor pixel array width. When this is not possible, the bar size
    /// should be rounded down to the nearest integer and the pattern can
    /// repeat on the right side. Each bar's height must always take up the
    /// full sensor pixel array height.
    ColorBars = 2,
    /// The test pattern is similar to TestPatternModeColorBars,
    /// except that each bar should start at its specified color at the top
    /// and fade to gray at the bottom. Furthermore each bar is further
    /// subdevided into a left and right half. The left half should have a
    /// smooth gradient, and the right half should have a quantized
    /// gradient. In particular, the right half's should consist of blocks
    /// of the same color for 1/16th active sensor pixel array width. The
    /// least significant bits in the quantized gradient should be copied
    /// from the most significant bits of the smooth gradient. The height of
    /// each bar should always be a multiple of 128. When this is not the
    /// case, the pattern should repeat at the bottom of the image.
    ColorBarsFadeToGray = 3,
    /// All pixel data is replaced by a pseudo-random sequence generated
    /// from a PN9 512-bit sequence (typically implemented in hardware with
    /// a linear feedback shift register). The generator should be reset at
    /// the beginning of each frame, and thus each subsequent raw frame with
    /// this test pattern should be exactly the same as the last.
    Pn9 = 4,
    /// The first custom test pattern. All custom patterns that are
    /// available only on this camera device are at least this numeric
    /// value. All of the custom test patterns will be static (that is the
    /// raw image must not vary from frame to frame).
    Custom1 = 256,
}
#[cfg(feature = "vendor_draft")]
impl TryFrom<ControlValue> for TestPatternMode {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Self::try_from(i32::try_from(value.clone())?)
            .map_err(|_| ControlValueError::UnknownVariant(value))
    }
}
#[cfg(feature = "vendor_draft")]
impl From<TestPatternMode> for ControlValue {
    fn from(val: TestPatternMode) -> Self {
        ControlValue::from(<i32>::from(val))
    }
}
#[cfg(feature = "vendor_draft")]
impl ControlEntry for TestPatternMode {
    const ID: u32 = ControlId::TestPatternMode as _;
}
#[cfg(feature = "vendor_draft")]
impl Control for TestPatternMode {}
/// Control to select the face detection mode used by the pipeline.
///
/// Currently identical to ANDROID_STATISTICS_FACE_DETECT_MODE.
///
/// \sa FaceDetectFaceRectangles
/// \sa FaceDetectFaceScores
/// \sa FaceDetectFaceLandmarks
/// \sa FaceDetectFaceIds
#[cfg(feature = "vendor_draft")]
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum FaceDetectMode {
    /// Pipeline doesn't perform face detection and doesn't report any
    /// control related to face detection.
    Off = 0,
    /// Pipeline performs face detection and reports the
    /// FaceDetectFaceRectangles and FaceDetectFaceScores controls for each
    /// detected face. FaceDetectFaceLandmarks and FaceDetectFaceIds are
    /// optional.
    Simple = 1,
    /// Pipeline performs face detection and reports all the controls
    /// related to face detection including FaceDetectFaceRectangles,
    /// FaceDetectFaceScores, FaceDetectFaceLandmarks, and
    /// FaceDeteceFaceIds for each detected face.
    Full = 2,
}
#[cfg(feature = "vendor_draft")]
impl TryFrom<ControlValue> for FaceDetectMode {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Self::try_from(i32::try_from(value.clone())?)
            .map_err(|_| ControlValueError::UnknownVariant(value))
    }
}
#[cfg(feature = "vendor_draft")]
impl From<FaceDetectMode> for ControlValue {
    fn from(val: FaceDetectMode) -> Self {
        ControlValue::from(<i32>::from(val))
    }
}
#[cfg(feature = "vendor_draft")]
impl ControlEntry for FaceDetectMode {
    const ID: u32 = ControlId::FaceDetectMode as _;
}
#[cfg(feature = "vendor_draft")]
impl Control for FaceDetectMode {}
/// Boundary rectangles of the detected faces. The number of values is
/// the number of detected faces.
///
/// The FaceDetectFaceRectangles control can only be returned in metadata.
///
/// Currently identical to ANDROID_STATISTICS_FACE_RECTANGLES.
#[cfg(feature = "vendor_draft")]
#[derive(Debug, Clone)]
pub struct FaceDetectFaceRectangles(pub Vec<Rectangle>);
#[cfg(feature = "vendor_draft")]
impl Deref for FaceDetectFaceRectangles {
    type Target = Vec<Rectangle>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(feature = "vendor_draft")]
impl DerefMut for FaceDetectFaceRectangles {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[cfg(feature = "vendor_draft")]
impl TryFrom<ControlValue> for FaceDetectFaceRectangles {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<Vec<Rectangle>>::try_from(value)?))
    }
}
#[cfg(feature = "vendor_draft")]
impl From<FaceDetectFaceRectangles> for ControlValue {
    fn from(val: FaceDetectFaceRectangles) -> Self {
        ControlValue::from(val.0)
    }
}
#[cfg(feature = "vendor_draft")]
impl ControlEntry for FaceDetectFaceRectangles {
    const ID: u32 = ControlId::FaceDetectFaceRectangles as _;
}
#[cfg(feature = "vendor_draft")]
impl Control for FaceDetectFaceRectangles {}
/// Confidence score of each of the detected faces. The range of score is
/// [0, 100]. The number of values should be the number of faces reported
/// in FaceDetectFaceRectangles.
///
/// The FaceDetectFaceScores control can only be returned in metadata.
///
/// Currently identical to ANDROID_STATISTICS_FACE_SCORES.
#[cfg(feature = "vendor_draft")]
#[derive(Debug, Clone)]
pub struct FaceDetectFaceScores(pub Vec<u8>);
#[cfg(feature = "vendor_draft")]
impl Deref for FaceDetectFaceScores {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(feature = "vendor_draft")]
impl DerefMut for FaceDetectFaceScores {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[cfg(feature = "vendor_draft")]
impl TryFrom<ControlValue> for FaceDetectFaceScores {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<Vec<u8>>::try_from(value)?))
    }
}
#[cfg(feature = "vendor_draft")]
impl From<FaceDetectFaceScores> for ControlValue {
    fn from(val: FaceDetectFaceScores) -> Self {
        ControlValue::from(val.0)
    }
}
#[cfg(feature = "vendor_draft")]
impl ControlEntry for FaceDetectFaceScores {
    const ID: u32 = ControlId::FaceDetectFaceScores as _;
}
#[cfg(feature = "vendor_draft")]
impl Control for FaceDetectFaceScores {}
/// Array of human face landmark coordinates in format [..., left_eye_i,
/// right_eye_i, mouth_i, left_eye_i+1, ...], with i = index of face. The
/// number of values should be 3 * the number of faces reported in
/// FaceDetectFaceRectangles.
///
/// The FaceDetectFaceLandmarks control can only be returned in metadata.
///
/// Currently identical to ANDROID_STATISTICS_FACE_LANDMARKS.
#[cfg(feature = "vendor_draft")]
#[derive(Debug, Clone)]
pub struct FaceDetectFaceLandmarks(pub Vec<Point>);
#[cfg(feature = "vendor_draft")]
impl Deref for FaceDetectFaceLandmarks {
    type Target = Vec<Point>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(feature = "vendor_draft")]
impl DerefMut for FaceDetectFaceLandmarks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[cfg(feature = "vendor_draft")]
impl TryFrom<ControlValue> for FaceDetectFaceLandmarks {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<Vec<Point>>::try_from(value)?))
    }
}
#[cfg(feature = "vendor_draft")]
impl From<FaceDetectFaceLandmarks> for ControlValue {
    fn from(val: FaceDetectFaceLandmarks) -> Self {
        ControlValue::from(val.0)
    }
}
#[cfg(feature = "vendor_draft")]
impl ControlEntry for FaceDetectFaceLandmarks {
    const ID: u32 = ControlId::FaceDetectFaceLandmarks as _;
}
#[cfg(feature = "vendor_draft")]
impl Control for FaceDetectFaceLandmarks {}
/// Each detected face is given a unique ID that is valid for as long as the
/// face is visible to the camera device. A face that leaves the field of
/// view and later returns may be assigned a new ID. The number of values
/// should be the number of faces reported in FaceDetectFaceRectangles.
///
/// The FaceDetectFaceIds control can only be returned in metadata.
///
/// Currently identical to ANDROID_STATISTICS_FACE_IDS.
#[cfg(feature = "vendor_draft")]
#[derive(Debug, Clone)]
pub struct FaceDetectFaceIds(pub Vec<i32>);
#[cfg(feature = "vendor_draft")]
impl Deref for FaceDetectFaceIds {
    type Target = Vec<i32>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(feature = "vendor_draft")]
impl DerefMut for FaceDetectFaceIds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[cfg(feature = "vendor_draft")]
impl TryFrom<ControlValue> for FaceDetectFaceIds {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<Vec<i32>>::try_from(value)?))
    }
}
#[cfg(feature = "vendor_draft")]
impl From<FaceDetectFaceIds> for ControlValue {
    fn from(val: FaceDetectFaceIds) -> Self {
        ControlValue::from(val.0)
    }
}
#[cfg(feature = "vendor_draft")]
impl ControlEntry for FaceDetectFaceIds {
    const ID: u32 = ControlId::FaceDetectFaceIds as _;
}
#[cfg(feature = "vendor_draft")]
impl Control for FaceDetectFaceIds {}
/// Toggles the Raspberry Pi IPA to output the hardware generated statistics.
///
/// When this control is set to true, the IPA outputs a binary dump of the
/// hardware generated statistics through the Request metadata in the
/// Bcm2835StatsOutput control.
///
/// \sa Bcm2835StatsOutput
#[cfg(feature = "vendor_rpi")]
#[derive(Debug, Clone)]
pub struct StatsOutputEnable(pub bool);
#[cfg(feature = "vendor_rpi")]
impl Deref for StatsOutputEnable {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(feature = "vendor_rpi")]
impl DerefMut for StatsOutputEnable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[cfg(feature = "vendor_rpi")]
impl TryFrom<ControlValue> for StatsOutputEnable {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<bool>::try_from(value)?))
    }
}
#[cfg(feature = "vendor_rpi")]
impl From<StatsOutputEnable> for ControlValue {
    fn from(val: StatsOutputEnable) -> Self {
        ControlValue::from(val.0)
    }
}
#[cfg(feature = "vendor_rpi")]
impl ControlEntry for StatsOutputEnable {
    const ID: u32 = ControlId::StatsOutputEnable as _;
}
#[cfg(feature = "vendor_rpi")]
impl Control for StatsOutputEnable {}
/// Span of the BCM2835 ISP generated statistics for the current frame.
///
/// This is sent in the Request metadata if the StatsOutputEnable is set to
/// true.  The statistics struct definition can be found in
/// include/linux/bcm2835-isp.h.
///
/// \sa StatsOutputEnable
#[cfg(feature = "vendor_rpi")]
#[derive(Debug, Clone)]
pub struct Bcm2835StatsOutput(pub Vec<u8>);
#[cfg(feature = "vendor_rpi")]
impl Deref for Bcm2835StatsOutput {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(feature = "vendor_rpi")]
impl DerefMut for Bcm2835StatsOutput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[cfg(feature = "vendor_rpi")]
impl TryFrom<ControlValue> for Bcm2835StatsOutput {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<Vec<u8>>::try_from(value)?))
    }
}
#[cfg(feature = "vendor_rpi")]
impl From<Bcm2835StatsOutput> for ControlValue {
    fn from(val: Bcm2835StatsOutput) -> Self {
        ControlValue::from(val.0)
    }
}
#[cfg(feature = "vendor_rpi")]
impl ControlEntry for Bcm2835StatsOutput {
    const ID: u32 = ControlId::Bcm2835StatsOutput as _;
}
#[cfg(feature = "vendor_rpi")]
impl Control for Bcm2835StatsOutput {}
/// An array of rectangles, where each singular value has identical
/// functionality to the ScalerCrop control. This control allows the
/// Raspberry Pi pipeline handler to control individual scaler crops per
/// output stream.
///
/// The order of rectangles passed into the control must match the order of
/// streams configured by the application. The pipeline handler will only
/// configure crop retangles up-to the number of output streams configured.
/// All subsequent rectangles passed into this control are ignored by the
/// pipeline handler.
///
/// If both rpi::ScalerCrops and ScalerCrop controls are present in a
/// ControlList, the latter is discarded, and crops are obtained from this
/// control.
///
/// Note that using different crop rectangles for each output stream with
/// this control is only applicable on the Pi5/PiSP platform. This control
/// should also be considered temporary/draft and will be replaced with
/// official libcamera API support for per-stream controls in the future.
///
/// \sa ScalerCrop
#[cfg(feature = "vendor_rpi")]
#[derive(Debug, Clone)]
pub struct ScalerCrops(pub Vec<Rectangle>);
#[cfg(feature = "vendor_rpi")]
impl Deref for ScalerCrops {
    type Target = Vec<Rectangle>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(feature = "vendor_rpi")]
impl DerefMut for ScalerCrops {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[cfg(feature = "vendor_rpi")]
impl TryFrom<ControlValue> for ScalerCrops {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<Vec<Rectangle>>::try_from(value)?))
    }
}
#[cfg(feature = "vendor_rpi")]
impl From<ScalerCrops> for ControlValue {
    fn from(val: ScalerCrops) -> Self {
        ControlValue::from(val.0)
    }
}
#[cfg(feature = "vendor_rpi")]
impl ControlEntry for ScalerCrops {
    const ID: u32 = ControlId::ScalerCrops as _;
}
#[cfg(feature = "vendor_rpi")]
impl Control for ScalerCrops {}
pub fn make_dyn(
    id: ControlId,
    val: ControlValue,
) -> Result<Box<dyn DynControlEntry>, ControlValueError> {
    match id {
        ControlId::AeEnable => Ok(Box::new(AeEnable::try_from(val)?)),
        ControlId::AeLocked => Ok(Box::new(AeLocked::try_from(val)?)),
        ControlId::AeMeteringMode => Ok(Box::new(AeMeteringMode::try_from(val)?)),
        ControlId::AeConstraintMode => Ok(Box::new(AeConstraintMode::try_from(val)?)),
        ControlId::AeExposureMode => Ok(Box::new(AeExposureMode::try_from(val)?)),
        ControlId::ExposureValue => Ok(Box::new(ExposureValue::try_from(val)?)),
        ControlId::ExposureTime => Ok(Box::new(ExposureTime::try_from(val)?)),
        ControlId::AnalogueGain => Ok(Box::new(AnalogueGain::try_from(val)?)),
        ControlId::AeFlickerMode => Ok(Box::new(AeFlickerMode::try_from(val)?)),
        ControlId::AeFlickerPeriod => Ok(Box::new(AeFlickerPeriod::try_from(val)?)),
        ControlId::AeFlickerDetected => Ok(Box::new(AeFlickerDetected::try_from(val)?)),
        ControlId::Brightness => Ok(Box::new(Brightness::try_from(val)?)),
        ControlId::Contrast => Ok(Box::new(Contrast::try_from(val)?)),
        ControlId::Lux => Ok(Box::new(Lux::try_from(val)?)),
        ControlId::AwbEnable => Ok(Box::new(AwbEnable::try_from(val)?)),
        ControlId::AwbMode => Ok(Box::new(AwbMode::try_from(val)?)),
        ControlId::AwbLocked => Ok(Box::new(AwbLocked::try_from(val)?)),
        ControlId::ColourGains => Ok(Box::new(ColourGains::try_from(val)?)),
        ControlId::ColourTemperature => Ok(Box::new(ColourTemperature::try_from(val)?)),
        ControlId::Saturation => Ok(Box::new(Saturation::try_from(val)?)),
        ControlId::SensorBlackLevels => Ok(Box::new(SensorBlackLevels::try_from(val)?)),
        ControlId::Sharpness => Ok(Box::new(Sharpness::try_from(val)?)),
        ControlId::FocusFoM => Ok(Box::new(FocusFoM::try_from(val)?)),
        ControlId::ColourCorrectionMatrix => {
            Ok(Box::new(ColourCorrectionMatrix::try_from(val)?))
        }
        ControlId::ScalerCrop => Ok(Box::new(ScalerCrop::try_from(val)?)),
        ControlId::DigitalGain => Ok(Box::new(DigitalGain::try_from(val)?)),
        ControlId::FrameDuration => Ok(Box::new(FrameDuration::try_from(val)?)),
        ControlId::FrameDurationLimits => {
            Ok(Box::new(FrameDurationLimits::try_from(val)?))
        }
        ControlId::SensorTemperature => Ok(Box::new(SensorTemperature::try_from(val)?)),
        ControlId::SensorTimestamp => Ok(Box::new(SensorTimestamp::try_from(val)?)),
        ControlId::AfMode => Ok(Box::new(AfMode::try_from(val)?)),
        ControlId::AfRange => Ok(Box::new(AfRange::try_from(val)?)),
        ControlId::AfSpeed => Ok(Box::new(AfSpeed::try_from(val)?)),
        ControlId::AfMetering => Ok(Box::new(AfMetering::try_from(val)?)),
        ControlId::AfWindows => Ok(Box::new(AfWindows::try_from(val)?)),
        ControlId::AfTrigger => Ok(Box::new(AfTrigger::try_from(val)?)),
        ControlId::AfPause => Ok(Box::new(AfPause::try_from(val)?)),
        ControlId::LensPosition => Ok(Box::new(LensPosition::try_from(val)?)),
        ControlId::AfState => Ok(Box::new(AfState::try_from(val)?)),
        ControlId::AfPauseState => Ok(Box::new(AfPauseState::try_from(val)?)),
        ControlId::HdrMode => Ok(Box::new(HdrMode::try_from(val)?)),
        ControlId::HdrChannel => Ok(Box::new(HdrChannel::try_from(val)?)),
        ControlId::Gamma => Ok(Box::new(Gamma::try_from(val)?)),
        ControlId::DebugMetadataEnable => {
            Ok(Box::new(DebugMetadataEnable::try_from(val)?))
        }
        #[cfg(feature = "vendor_draft")]
        ControlId::AePrecaptureTrigger => {
            Ok(Box::new(AePrecaptureTrigger::try_from(val)?))
        }
        #[cfg(feature = "vendor_draft")]
        ControlId::NoiseReductionMode => Ok(Box::new(NoiseReductionMode::try_from(val)?)),
        #[cfg(feature = "vendor_draft")]
        ControlId::ColorCorrectionAberrationMode => {
            Ok(Box::new(ColorCorrectionAberrationMode::try_from(val)?))
        }
        #[cfg(feature = "vendor_draft")]
        ControlId::AeState => Ok(Box::new(AeState::try_from(val)?)),
        #[cfg(feature = "vendor_draft")]
        ControlId::AwbState => Ok(Box::new(AwbState::try_from(val)?)),
        #[cfg(feature = "vendor_draft")]
        ControlId::SensorRollingShutterSkew => {
            Ok(Box::new(SensorRollingShutterSkew::try_from(val)?))
        }
        #[cfg(feature = "vendor_draft")]
        ControlId::LensShadingMapMode => Ok(Box::new(LensShadingMapMode::try_from(val)?)),
        #[cfg(feature = "vendor_draft")]
        ControlId::PipelineDepth => Ok(Box::new(PipelineDepth::try_from(val)?)),
        #[cfg(feature = "vendor_draft")]
        ControlId::MaxLatency => Ok(Box::new(MaxLatency::try_from(val)?)),
        #[cfg(feature = "vendor_draft")]
        ControlId::TestPatternMode => Ok(Box::new(TestPatternMode::try_from(val)?)),
        #[cfg(feature = "vendor_draft")]
        ControlId::FaceDetectMode => Ok(Box::new(FaceDetectMode::try_from(val)?)),
        #[cfg(feature = "vendor_draft")]
        ControlId::FaceDetectFaceRectangles => {
            Ok(Box::new(FaceDetectFaceRectangles::try_from(val)?))
        }
        #[cfg(feature = "vendor_draft")]
        ControlId::FaceDetectFaceScores => {
            Ok(Box::new(FaceDetectFaceScores::try_from(val)?))
        }
        #[cfg(feature = "vendor_draft")]
        ControlId::FaceDetectFaceLandmarks => {
            Ok(Box::new(FaceDetectFaceLandmarks::try_from(val)?))
        }
        #[cfg(feature = "vendor_draft")]
        ControlId::FaceDetectFaceIds => Ok(Box::new(FaceDetectFaceIds::try_from(val)?)),
        #[cfg(feature = "vendor_rpi")]
        ControlId::StatsOutputEnable => Ok(Box::new(StatsOutputEnable::try_from(val)?)),
        #[cfg(feature = "vendor_rpi")]
        ControlId::Bcm2835StatsOutput => Ok(Box::new(Bcm2835StatsOutput::try_from(val)?)),
        #[cfg(feature = "vendor_rpi")]
        ControlId::ScalerCrops => Ok(Box::new(ScalerCrops::try_from(val)?)),
    }
}
