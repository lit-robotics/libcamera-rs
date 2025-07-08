use std::ops::{Deref, DerefMut};
use num_enum::{IntoPrimitive, TryFromPrimitive};
#[allow(unused_imports)]
use crate::control::{Control, Property, ControlEntry, DynControlEntry};
use crate::control_value::{ControlValue, ControlValueError};
#[allow(unused_imports)]
use crate::geometry::{Rectangle, Point, Size};
#[allow(unused_imports)]
use libcamera_sys::*;
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum PropertyId {
    /// Camera mounting location
    Location = LOCATION,
    /// The camera physical mounting rotation. It is expressed as the angular
    /// difference in degrees between two reference systems, one relative to the
    /// camera module, and one defined on the external world scene to be
    /// captured when projected on the image sensor pixel array.
    ///
    /// A camera sensor has a 2-dimensional reference system 'Rc' defined by
    /// its pixel array read-out order. The origin is set to the first pixel
    /// being read out, the X-axis points along the column read-out direction
    /// towards the last columns, and the Y-axis along the row read-out
    /// direction towards the last row.
    ///
    /// A typical example for a sensor with a 2592x1944 pixel array matrix
    /// observed from the front is
    ///
    /// ```text
    ///             2591       X-axis          0
    ///               <------------------------+ 0
    ///               .......... ... ..........!
    ///               .......... ... ..........! Y-axis
    ///                          ...           !
    ///               .......... ... ..........!
    ///               .......... ... ..........! 1943
    ///                                        V
    /// ```
    ///
    ///
    /// The external world scene reference system 'Rs' is a 2-dimensional
    /// reference system on the focal plane of the camera module. The origin is
    /// placed on the top-left corner of the visible scene, the X-axis points
    /// towards the right, and the Y-axis points towards the bottom of the
    /// scene. The top, bottom, left and right directions are intentionally not
    /// defined and depend on the environment in which the camera is used.
    ///
    /// A typical example of a (very common) picture of a shark swimming from
    /// left to right, as seen from the camera, is
    ///
    /// ```text
    ///              0               X-axis
    ///            0 +------------------------------------->
    ///              !
    ///              !
    ///              !
    ///              !           |\____)\___
    ///              !           ) _____  __`<
    ///              !           |/     )/
    ///              !
    ///              !
    ///              !
    ///              V
    ///            Y-axis
    /// ```
    ///
    /// With the reference system 'Rs' placed on the camera focal plane.
    ///
    /// ```text
    ///                                 ¸.·˙!
    ///                             ¸.·˙    !
    ///                 _       ¸.·˙        !
    ///              +-/ \-+¸.·˙            !
    ///              | (o) |                ! Camera focal plane
    ///              +-----+˙·.¸            !
    ///                         ˙·.¸        !
    ///                             ˙·.¸    !
    ///                                 ˙·.¸!
    /// ```
    ///
    /// When projected on the sensor's pixel array, the image and the associated
    /// reference system 'Rs' are typically (but not always) inverted, due to
    /// the camera module's lens optical inversion effect.
    ///
    /// Assuming the above represented scene of the swimming shark, the lens
    /// inversion projects the scene and its reference system onto the sensor
    /// pixel array, seen from the front of the camera sensor, as follow
    ///
    /// ```text
    ///           Y-axis
    ///              ^
    ///              !
    ///              !
    ///              !
    ///              !            |\_____)\__
    ///              !            ) ____  ___.<
    ///              !            |/    )/
    ///              !
    ///              !
    ///              !
    ///            0 +------------------------------------->
    ///              0               X-axis
    /// ```
    ///
    /// Note the shark being upside-down.
    ///
    /// The resulting projected reference system is named 'Rp'.
    ///
    /// The camera rotation property is then defined as the angular difference
    /// in the counter-clockwise direction between the camera reference system
    /// 'Rc' and the projected scene reference system 'Rp'. It is expressed in
    /// degrees as a number in the range [0, 360[.
    ///
    /// Examples
    ///
    /// 0 degrees camera rotation
    ///
    ///
    /// ```text
    ///                   Y-Rp
    ///                    ^
    ///             Y-Rc   !
    ///              ^     !
    ///              !     !
    ///              !     !
    ///              !     !
    ///              !     !
    ///              !     !
    ///              !     !
    ///              !     !
    ///              !   0 +------------------------------------->
    ///              !     0               X-Rp
    ///            0 +------------------------------------->
    ///              0               X-Rc
    /// ```
    ///
    ///
    /// ```text
    ///                               X-Rc                0
    ///              <------------------------------------+ 0
    ///                          X-Rp                 0   !
    ///          <------------------------------------+ 0 !
    ///                                               !   !
    ///                                               !   !
    ///                                               !   !
    ///                                               !   !
    ///                                               !   !
    ///                                               !   !
    ///                                               !   !
    ///                                               !   V
    ///                                               !  Y-Rc
    ///                                               V
    ///                                              Y-Rp
    /// ```
    ///
    /// 90 degrees camera rotation
    ///
    /// ```text
    ///              0        Y-Rc
    ///            0 +-------------------->
    ///              !   Y-Rp
    ///              !    ^
    ///              !    !
    ///              !    !
    ///              !    !
    ///              !    !
    ///              !    !
    ///              !    !
    ///              !    !
    ///              !    !
    ///              !    !
    ///              !  0 +------------------------------------->
    ///              !    0              X-Rp
    ///              !
    ///              !
    ///              !
    ///              !
    ///              V
    ///             X-Rc
    /// ```
    ///
    /// 180 degrees camera rotation
    ///
    /// ```text
    ///                                           0
    ///      <------------------------------------+ 0
    ///                       X-Rc                !
    ///             Y-Rp                          !
    ///              ^                            !
    ///              !                            !
    ///              !                            !
    ///              !                            !
    ///              !                            !
    ///              !                            !
    ///              !                            !
    ///              !                            V
    ///              !                           Y-Rc
    ///            0 +------------------------------------->
    ///              0              X-Rp
    /// ```
    ///
    /// 270 degrees camera rotation
    ///
    /// ```text
    ///              0        Y-Rc
    ///            0 +-------------------->
    ///              !                                        0
    ///              !    <-----------------------------------+ 0
    ///              !                    X-Rp                !
    ///              !                                        !
    ///              !                                        !
    ///              !                                        !
    ///              !                                        !
    ///              !                                        !
    ///              !                                        !
    ///              !                                        !
    ///              !                                        !
    ///              !                                        V
    ///              !                                       Y-Rp
    ///              !
    ///              !
    ///              !
    ///              !
    ///              V
    ///             X-Rc
    /// ```
    ///
    ///
    /// Example one - Webcam
    ///
    /// A camera module installed on the user facing part of a laptop screen
    /// casing used for video calls. The captured images are meant to be
    /// displayed in landscape mode (width > height) on the laptop screen.
    ///
    /// The camera is typically mounted upside-down to compensate the lens
    /// optical inversion effect.
    ///
    /// ```text
    ///                   Y-Rp
    ///             Y-Rc   ^
    ///              ^     !
    ///              !     !
    ///              !     !       |\_____)\__
    ///              !     !       ) ____  ___.<
    ///              !     !       |/    )/
    ///              !     !
    ///              !     !
    ///              !     !
    ///              !   0 +------------------------------------->
    ///              !     0           X-Rp
    ///            0 +------------------------------------->
    ///              0            X-Rc
    /// ```
    ///
    /// The two reference systems are aligned, the resulting camera rotation is
    /// 0 degrees, no rotation correction needs to be applied to the resulting
    /// image once captured to memory buffers to correctly display it to users.
    ///
    /// ```text
    ///              +--------------------------------------+
    ///              !                                      !
    ///              !                                      !
    ///              !                                      !
    ///              !             |\____)\___              !
    ///              !             ) _____  __`<            !
    ///              !             |/     )/                !
    ///              !                                      !
    ///              !                                      !
    ///              !                                      !
    ///              +--------------------------------------+
    /// ```
    ///
    /// If the camera sensor is not mounted upside-down to compensate for the
    /// lens optical inversion, the two reference systems will not be aligned,
    /// with 'Rp' being rotated 180 degrees relatively to 'Rc'.
    ///
    ///
    /// ```text
    ///                       X-Rc                0
    ///      <------------------------------------+ 0
    ///                                           !
    ///             Y-Rp                          !
    ///              ^                            !
    ///              !                            !
    ///              !       |\_____)\__          !
    ///              !       ) ____  ___.<        !
    ///              !       |/    )/             !
    ///              !                            !
    ///              !                            !
    ///              !                            V
    ///              !                           Y-Rc
    ///            0 +------------------------------------->
    ///              0            X-Rp
    /// ```
    ///
    /// The image once captured to memory will then be rotated by 180 degrees
    ///
    /// ```text
    ///              +--------------------------------------+
    ///              !                                      !
    ///              !                                      !
    ///              !                                      !
    ///              !              __/(_____/|             !
    ///              !            >.___  ____ (             !
    ///              !                 \(    \|             !
    ///              !                                      !
    ///              !                                      !
    ///              !                                      !
    ///              +--------------------------------------+
    /// ```
    ///
    /// A software rotation correction of 180 degrees should be applied to
    /// correctly display the image.
    ///
    /// ```text
    ///              +--------------------------------------+
    ///              !                                      !
    ///              !                                      !
    ///              !                                      !
    ///              !             |\____)\___              !
    ///              !             ) _____  __`<            !
    ///              !             |/     )/                !
    ///              !                                      !
    ///              !                                      !
    ///              !                                      !
    ///              +--------------------------------------+
    /// ```
    ///
    /// Example two - Phone camera
    ///
    /// A camera installed on the back side of a mobile device facing away from
    /// the user. The captured images are meant to be displayed in portrait mode
    /// (height > width) to match the device screen orientation and the device
    /// usage orientation used when taking the picture.
    ///
    /// The camera sensor is typically mounted with its pixel array longer side
    /// aligned to the device longer side, upside-down mounted to compensate for
    /// the lens optical inversion effect.
    ///
    /// ```text
    ///              0        Y-Rc
    ///            0 +-------------------->
    ///              !   Y-Rp
    ///              !    ^
    ///              !    !
    ///              !    !
    ///              !    !
    ///              !    !            |\_____)\__
    ///              !    !            ) ____  ___.<
    ///              !    !            |/    )/
    ///              !    !
    ///              !    !
    ///              !    !
    ///              !  0 +------------------------------------->
    ///              !    0                X-Rp
    ///              !
    ///              !
    ///              !
    ///              !
    ///              V
    ///             X-Rc
    /// ```
    ///
    /// The two reference systems are not aligned and the 'Rp' reference
    /// system is rotated by 90 degrees in the counter-clockwise direction
    /// relatively to the 'Rc' reference system.
    ///
    /// The image once captured to memory will be rotated.
    ///
    /// ```text
    ///              +-------------------------------------+
    ///              |                 _ _                 |
    ///              |                \   /                |
    ///              |                 | |                 |
    ///              |                 | |                 |
    ///              |                 |  >                |
    ///              |                <  |                 |
    ///              |                 | |                 |
    ///              |                   .                 |
    ///              |                  V                  |
    ///              +-------------------------------------+
    /// ```
    ///
    /// A correction of 90 degrees in counter-clockwise direction has to be
    /// applied to correctly display the image in portrait mode on the device
    /// screen.
    ///
    /// ```text
    ///                       +--------------------+
    ///                       |                    |
    ///                       |                    |
    ///                       |                    |
    ///                       |                    |
    ///                       |                    |
    ///                       |                    |
    ///                       |   |\____)\___      |
    ///                       |   ) _____  __`<    |
    ///                       |   |/     )/        |
    ///                       |                    |
    ///                       |                    |
    ///                       |                    |
    ///                       |                    |
    ///                       |                    |
    ///                       +--------------------+
    Rotation = ROTATION,
    /// The model name shall to the extent possible describe the sensor. For
    /// most devices this is the model name of the sensor. While for some
    /// devices the sensor model is unavailable as the sensor or the entire
    /// camera is part of a larger unit and exposed as a black-box to the
    /// system. In such cases the model name of the smallest device that
    /// contains the camera sensor shall be used.
    ///
    /// The model name is not meant to be a camera name displayed to the
    /// end-user, but may be combined with other camera information to create a
    /// camera name.
    ///
    /// The model name is not guaranteed to be unique in the system nor is
    /// it guaranteed to be stable or have any other properties required to make
    /// it a good candidate to be used as a permanent identifier of a camera.
    ///
    /// The model name shall describe the camera in a human readable format and
    /// shall be encoded in ASCII.
    ///
    /// Example model names are 'ov5670', 'imx219' or 'Logitech Webcam C930e'.
    Model = MODEL,
    /// The pixel unit cell physical size, in nanometers.
    ///
    /// The UnitCellSize properties defines the horizontal and vertical sizes of
    /// a single pixel unit, including its active and non-active parts. In
    /// other words, it expresses the horizontal and vertical distance between
    /// the top-left corners of adjacent pixels.
    ///
    /// The property can be used to calculate the physical size of the sensor's
    /// pixel array area and for calibration purposes.
    UnitCellSize = UNIT_CELL_SIZE,
    /// The camera sensor pixel array readable area vertical and horizontal
    /// sizes, in pixels.
    ///
    /// The PixelArraySize property defines the size in pixel units of the
    /// readable part of full pixel array matrix, including optical black
    /// pixels used for calibration, pixels which are not considered valid for
    /// capture and active pixels containing valid image data.
    ///
    /// The property describes the maximum size of the raw data captured by the
    /// camera, which might not correspond to the physical size of the sensor
    /// pixel array matrix, as some portions of the physical pixel array matrix
    /// are not accessible and cannot be transmitted out.
    ///
    /// For example, let's consider a pixel array matrix assembled as follows
    ///
    /// ```text
    ///      +--------------------------------------------------+
    ///      |xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx|
    ///      |xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx|
    ///      |xxDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDxx|
    ///      |xxDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDxx|
    ///      |xxDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDxx|
    ///      |xxDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDxx|
    ///      |xxDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDxx|
    ///      |xxDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDxx|
    ///      ...          ...           ...      ...          ...
    /// ```
    ///
    /// ```text
    ///      ...          ...           ...      ...          ...
    ///      |xxDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDxx|
    ///      |xxDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDxx|
    ///      |xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx|
    ///      |xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx|
    ///      +--------------------------------------------------+
    /// ```
    ///
    /// starting with two lines of non-readable pixels (x), followed by N lines
    /// of readable data (D) surrounded by two columns of non-readable pixels on
    /// each side, and ending with two more lines of non-readable pixels. Only
    /// the readable portion is transmitted to the receiving side, defining the
    /// sizes of the largest possible buffer of raw data that can be presented
    /// to applications.
    ///
    /// ```text
    ///                      PixelArraySize.width
    ///        /----------------------------------------------/
    ///        +----------------------------------------------+ /
    ///        |DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD| |
    ///        |DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD| |
    ///        |DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD| |
    ///        |DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD| |
    ///        |DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD| |
    ///        |DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD| | PixelArraySize.height
    ///        ...        ...           ...      ...        ...
    ///        ...        ...           ...      ...        ...
    ///        |DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD| |
    ///        |DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD| |
    ///        +----------------------------------------------+ /
    /// ```
    ///
    /// This defines a rectangle whose top-left corner is placed in position (0,
    /// 0) and whose vertical and horizontal sizes are defined by this property.
    /// All other rectangles that describe portions of the pixel array, such as
    /// the optical black pixels rectangles and active pixel areas, are defined
    /// relatively to this rectangle.
    ///
    /// All the coordinates are expressed relative to the default sensor readout
    /// direction, without any transformation (such as horizontal and vertical
    /// flipping) applied. When mapping them to the raw pixel buffer,
    /// applications shall take any configured transformation into account.
    ///
    /// \todo Rename this property to Size once we will have property
    /// ```text
    ///       categories (i.e. Properties::PixelArray::Size)
    PixelArraySize = PIXEL_ARRAY_SIZE,
    /// The pixel array region(s) which contain optical black pixels
    /// considered valid for calibration purposes.
    ///
    /// This property describes the position and size of optical black pixel
    /// regions in the raw data buffer as stored in memory, which might differ
    /// from their actual physical location in the pixel array matrix.
    ///
    /// It is important to note, in fact, that camera sensors might
    /// automatically reorder or skip portions of their pixels array matrix when
    /// transmitting data to the receiver. For instance, a sensor may merge the
    /// top and bottom optical black rectangles into a single rectangle,
    /// transmitted at the beginning of the frame.
    ///
    /// The pixel array contains several areas with different purposes,
    /// interleaved by lines and columns which are said not to be valid for
    /// capturing purposes. Invalid lines and columns are defined as invalid as
    /// they could be positioned too close to the chip margins or to the optical
    /// black shielding placed on top of optical black pixels.
    ///
    /// ```text
    ///                      PixelArraySize.width
    ///        /----------------------------------------------/
    ///           x1                                       x2
    ///        +--o---------------------------------------o---+ /
    ///        |IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII| |
    ///        |IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII| |
    ///     y1 oIIOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOII| |
    ///        |IIOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOII| |
    ///        |IIOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOII| |
    ///     y2 oIIOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOII| |
    ///        |IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII| |
    ///        |IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII| |
    ///     y3 |IIOOPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPOOII| |
    ///        |IIOOPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPOOII| | PixelArraySize.height
    ///        |IIOOPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPOOII| |
    ///        ...          ...           ...     ...       ...
    ///        ...          ...           ...     ...       ...
    ///     y4 |IIOOPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPOOII| |
    ///        |IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII| |
    ///        |IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII| |
    ///        +----------------------------------------------+ /
    /// ```
    ///
    /// The readable pixel array matrix is composed by
    /// 2 invalid lines (I)
    /// 4 lines of valid optical black pixels (O)
    /// 2 invalid lines (I)
    /// n lines of valid pixel data (P)
    /// 2 invalid lines (I)
    ///
    /// And the position of the optical black pixel rectangles is defined by
    ///
    /// ```text
    ///     PixelArrayOpticalBlackRectangles = {
    ///        { x1, y1, x2 - x1 + 1, y2 - y1 + 1 },
    ///        { x1, y3, 2, y4 - y3 + 1 },
    ///        { x2, y3, 2, y4 - y3 + 1 },
    ///     };
    /// ```
    ///
    /// If the camera, when capturing the full pixel array matrix, automatically
    /// skips the invalid lines and columns, producing the following data
    /// buffer, when captured to memory
    ///
    /// ```text
    ///                      PixelArraySize.width
    ///        /----------------------------------------------/
    ///                                                    x1
    ///        +--------------------------------------------o-+ /
    ///        |OOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO| |
    ///        |OOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO| |
    ///        |OOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO| |
    ///        |OOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO| |
    ///     y1 oOOPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPOO| |
    ///        |OOPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPOO| |
    ///        |OOPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPOO| | PixelArraySize.height
    ///        ...       ...          ...       ...         ... |
    ///        ...       ...          ...       ...         ... |
    ///        |OOPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPOO| |
    ///        |OOPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPOO| |
    ///        +----------------------------------------------+ /
    /// ```
    ///
    /// then the invalid lines and columns should not be reported as part of the
    /// PixelArraySize property in first place.
    ///
    /// In this case, the position of the black pixel rectangles will be
    ///
    /// ```text
    ///     PixelArrayOpticalBlackRectangles = {
    ///        { 0, 0, y1 + 1, PixelArraySize[0] },
    ///        { 0, y1, 2, PixelArraySize[1] - y1 + 1 },
    ///        { x1, y1, 2, PixelArraySize[1] - y1 + 1 },
    ///     };
    /// ```
    ///
    /// \todo Rename this property to Size once we will have property
    /// ```text
    ///       categories (i.e. Properties::PixelArray::OpticalBlackRectangles)
    PixelArrayOpticalBlackRectangles = PIXEL_ARRAY_OPTICAL_BLACK_RECTANGLES,
    /// The PixelArrayActiveAreas property defines the (possibly multiple and
    /// overlapping) portions of the camera sensor readable pixel matrix
    /// which are considered valid for image acquisition purposes.
    ///
    /// This property describes an arbitrary number of overlapping rectangles,
    /// with each rectangle representing the maximum image size that the camera
    /// sensor can produce for a particular aspect ratio. They are defined
    /// relatively to the PixelArraySize rectangle.
    ///
    /// When multiple rectangles are reported, they shall be ordered from the
    /// tallest to the shortest.
    ///
    /// Example 1
    /// A camera sensor which only produces images in the 4:3 image resolution
    /// will report a single PixelArrayActiveAreas rectangle, from which all
    /// other image formats are obtained by either cropping the field-of-view
    /// and/or applying pixel sub-sampling techniques such as pixel skipping or
    /// binning.
    ///
    /// ```text
    ///            PixelArraySize.width
    ///             /----------------/
    ///               x1          x2
    ///     (0,0)-> +-o------------o-+  /
    ///          y1 o +------------+ |  |
    ///             | |////////////| |  |
    ///             | |////////////| |  | PixelArraySize.height
    ///             | |////////////| |  |
    ///          y2 o +------------+ |  |
    ///             +----------------+  /
    /// ```
    ///
    /// The property reports a single rectangle
    ///
    /// ```text
    ///          PixelArrayActiveAreas = (x1, y1, x2 - x1 + 1, y2 - y1 + 1)
    /// ```
    ///
    /// Example 2
    /// A camera sensor which can produce images in different native
    /// resolutions will report several overlapping rectangles, one for each
    /// natively supported resolution.
    ///
    /// ```text
    ///              PixelArraySize.width
    ///             /------------------/
    ///               x1  x2    x3  x4
    ///     (0,0)-> +o---o------o---o+  /
    ///          y1 o    +------+    |  |
    ///             |    |//////|    |  |
    ///          y2 o+---+------+---+|  |
    ///             ||///|//////|///||  | PixelArraySize.height
    ///          y3 o+---+------+---+|  |
    ///             |    |//////|    |  |
    ///          y4 o    +------+    |  |
    ///             +----+------+----+  /
    /// ```
    ///
    /// The property reports two rectangles
    ///
    /// ```text
    ///         PixelArrayActiveAreas = ((x2, y1, x3 - x2 + 1, y4 - y1 + 1),
    ///                                  (x1, y2, x4 - x1 + 1, y3 - y2 + 1))
    /// ```
    ///
    /// The first rectangle describes the maximum field-of-view of all image
    /// formats in the 4:3 resolutions, while the second one describes the
    /// maximum field of view for all image formats in the 16:9 resolutions.
    ///
    /// Multiple rectangles shall only be reported when the sensor can't capture
    /// the pixels in the corner regions. If all the pixels in the (x1,y1) -
    /// (x4,y4) area can be captured, the PixelArrayActiveAreas property shall
    /// contains the single rectangle (x1,y1) - (x4,y4).
    ///
    /// \todo Rename this property to ActiveAreas once we will have property
    /// ```text
    ///       categories (i.e. Properties::PixelArray::ActiveAreas)
    PixelArrayActiveAreas = PIXEL_ARRAY_ACTIVE_AREAS,
    /// The maximum valid rectangle for the controls::ScalerCrop control. This
    /// reflects the minimum mandatory cropping applied in the camera sensor and
    /// the rest of the pipeline. Just as the ScalerCrop control, it defines a
    /// rectangle taken from the sensor's active pixel array.
    ///
    /// This property is valid only after the camera has been successfully
    /// configured and its value may change whenever a new configuration is
    /// applied.
    ///
    /// \todo Turn this property into a "maximum control value" for the
    /// ScalerCrop control once "dynamic" controls have been implemented.
    ScalerCropMaximum = SCALER_CROP_MAXIMUM,
    /// The relative sensitivity of the chosen sensor mode.
    ///
    /// Some sensors have readout modes with different sensitivities. For example,
    /// a binned camera mode might, with the same exposure and gains, produce
    /// twice the signal level of the full resolution readout. This would be
    /// signalled by the binned mode, when it is chosen, indicating a value here
    /// that is twice that of the full resolution mode. This value will be valid
    /// after the configure method has returned successfully.
    SensorSensitivity = SENSOR_SENSITIVITY,
    /// A list of integer values of type dev_t denoting the major and minor
    /// device numbers of the underlying devices used in the operation of this
    /// camera.
    ///
    /// Different cameras may report identical devices.
    SystemDevices = SYSTEM_DEVICES,
    /// The arrangement of color filters on sensor; represents the colors in the
    /// top-left 2x2 section of the sensor, in reading order. Currently
    /// identical to ANDROID_SENSOR_INFO_COLOR_FILTER_ARRANGEMENT.
    #[cfg(feature = "vendor_draft")]
    ColorFilterArrangement = COLOR_FILTER_ARRANGEMENT,
}
impl PropertyId {
    pub fn id(&self) -> u32 {
        u32::from(*self)
    }
}
/// Camera mounting location
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum Location {
    /// The camera is mounted on the front side of the device, facing the
    /// user
    CameraFront = 0,
    /// The camera is mounted on the back side of the device, facing away
    /// from the user
    CameraBack = 1,
    /// The camera is attached to the device in a way that allows it to
    /// be moved freely
    CameraExternal = 2,
}
impl TryFrom<ControlValue> for Location {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Self::try_from(i32::try_from(value.clone())?)
            .map_err(|_| ControlValueError::UnknownVariant(value))
    }
}
impl From<Location> for ControlValue {
    fn from(val: Location) -> Self {
        ControlValue::from(<i32>::from(val))
    }
}
impl ControlEntry for Location {
    const ID: u32 = PropertyId::Location as _;
}
impl Property for Location {}
/// The camera physical mounting rotation. It is expressed as the angular
/// difference in degrees between two reference systems, one relative to the
/// camera module, and one defined on the external world scene to be
/// captured when projected on the image sensor pixel array.
///
/// A camera sensor has a 2-dimensional reference system 'Rc' defined by
/// its pixel array read-out order. The origin is set to the first pixel
/// being read out, the X-axis points along the column read-out direction
/// towards the last columns, and the Y-axis along the row read-out
/// direction towards the last row.
///
/// A typical example for a sensor with a 2592x1944 pixel array matrix
/// observed from the front is
///
/// ```text
///             2591       X-axis          0
///               <------------------------+ 0
///               .......... ... ..........!
///               .......... ... ..........! Y-axis
///                          ...           !
///               .......... ... ..........!
///               .......... ... ..........! 1943
///                                        V
/// ```
///
///
/// The external world scene reference system 'Rs' is a 2-dimensional
/// reference system on the focal plane of the camera module. The origin is
/// placed on the top-left corner of the visible scene, the X-axis points
/// towards the right, and the Y-axis points towards the bottom of the
/// scene. The top, bottom, left and right directions are intentionally not
/// defined and depend on the environment in which the camera is used.
///
/// A typical example of a (very common) picture of a shark swimming from
/// left to right, as seen from the camera, is
///
/// ```text
///              0               X-axis
///            0 +------------------------------------->
///              !
///              !
///              !
///              !           |\____)\___
///              !           ) _____  __`<
///              !           |/     )/
///              !
///              !
///              !
///              V
///            Y-axis
/// ```
///
/// With the reference system 'Rs' placed on the camera focal plane.
///
/// ```text
///                                 ¸.·˙!
///                             ¸.·˙    !
///                 _       ¸.·˙        !
///              +-/ \-+¸.·˙            !
///              | (o) |                ! Camera focal plane
///              +-----+˙·.¸            !
///                         ˙·.¸        !
///                             ˙·.¸    !
///                                 ˙·.¸!
/// ```
///
/// When projected on the sensor's pixel array, the image and the associated
/// reference system 'Rs' are typically (but not always) inverted, due to
/// the camera module's lens optical inversion effect.
///
/// Assuming the above represented scene of the swimming shark, the lens
/// inversion projects the scene and its reference system onto the sensor
/// pixel array, seen from the front of the camera sensor, as follow
///
/// ```text
///           Y-axis
///              ^
///              !
///              !
///              !
///              !            |\_____)\__
///              !            ) ____  ___.<
///              !            |/    )/
///              !
///              !
///              !
///            0 +------------------------------------->
///              0               X-axis
/// ```
///
/// Note the shark being upside-down.
///
/// The resulting projected reference system is named 'Rp'.
///
/// The camera rotation property is then defined as the angular difference
/// in the counter-clockwise direction between the camera reference system
/// 'Rc' and the projected scene reference system 'Rp'. It is expressed in
/// degrees as a number in the range [0, 360[.
///
/// Examples
///
/// 0 degrees camera rotation
///
///
/// ```text
///                   Y-Rp
///                    ^
///             Y-Rc   !
///              ^     !
///              !     !
///              !     !
///              !     !
///              !     !
///              !     !
///              !     !
///              !     !
///              !   0 +------------------------------------->
///              !     0               X-Rp
///            0 +------------------------------------->
///              0               X-Rc
/// ```
///
///
/// ```text
///                               X-Rc                0
///              <------------------------------------+ 0
///                          X-Rp                 0   !
///          <------------------------------------+ 0 !
///                                               !   !
///                                               !   !
///                                               !   !
///                                               !   !
///                                               !   !
///                                               !   !
///                                               !   !
///                                               !   V
///                                               !  Y-Rc
///                                               V
///                                              Y-Rp
/// ```
///
/// 90 degrees camera rotation
///
/// ```text
///              0        Y-Rc
///            0 +-------------------->
///              !   Y-Rp
///              !    ^
///              !    !
///              !    !
///              !    !
///              !    !
///              !    !
///              !    !
///              !    !
///              !    !
///              !    !
///              !  0 +------------------------------------->
///              !    0              X-Rp
///              !
///              !
///              !
///              !
///              V
///             X-Rc
/// ```
///
/// 180 degrees camera rotation
///
/// ```text
///                                           0
///      <------------------------------------+ 0
///                       X-Rc                !
///             Y-Rp                          !
///              ^                            !
///              !                            !
///              !                            !
///              !                            !
///              !                            !
///              !                            !
///              !                            !
///              !                            V
///              !                           Y-Rc
///            0 +------------------------------------->
///              0              X-Rp
/// ```
///
/// 270 degrees camera rotation
///
/// ```text
///              0        Y-Rc
///            0 +-------------------->
///              !                                        0
///              !    <-----------------------------------+ 0
///              !                    X-Rp                !
///              !                                        !
///              !                                        !
///              !                                        !
///              !                                        !
///              !                                        !
///              !                                        !
///              !                                        !
///              !                                        !
///              !                                        V
///              !                                       Y-Rp
///              !
///              !
///              !
///              !
///              V
///             X-Rc
/// ```
///
///
/// Example one - Webcam
///
/// A camera module installed on the user facing part of a laptop screen
/// casing used for video calls. The captured images are meant to be
/// displayed in landscape mode (width > height) on the laptop screen.
///
/// The camera is typically mounted upside-down to compensate the lens
/// optical inversion effect.
///
/// ```text
///                   Y-Rp
///             Y-Rc   ^
///              ^     !
///              !     !
///              !     !       |\_____)\__
///              !     !       ) ____  ___.<
///              !     !       |/    )/
///              !     !
///              !     !
///              !     !
///              !   0 +------------------------------------->
///              !     0           X-Rp
///            0 +------------------------------------->
///              0            X-Rc
/// ```
///
/// The two reference systems are aligned, the resulting camera rotation is
/// 0 degrees, no rotation correction needs to be applied to the resulting
/// image once captured to memory buffers to correctly display it to users.
///
/// ```text
///              +--------------------------------------+
///              !                                      !
///              !                                      !
///              !                                      !
///              !             |\____)\___              !
///              !             ) _____  __`<            !
///              !             |/     )/                !
///              !                                      !
///              !                                      !
///              !                                      !
///              +--------------------------------------+
/// ```
///
/// If the camera sensor is not mounted upside-down to compensate for the
/// lens optical inversion, the two reference systems will not be aligned,
/// with 'Rp' being rotated 180 degrees relatively to 'Rc'.
///
///
/// ```text
///                       X-Rc                0
///      <------------------------------------+ 0
///                                           !
///             Y-Rp                          !
///              ^                            !
///              !                            !
///              !       |\_____)\__          !
///              !       ) ____  ___.<        !
///              !       |/    )/             !
///              !                            !
///              !                            !
///              !                            V
///              !                           Y-Rc
///            0 +------------------------------------->
///              0            X-Rp
/// ```
///
/// The image once captured to memory will then be rotated by 180 degrees
///
/// ```text
///              +--------------------------------------+
///              !                                      !
///              !                                      !
///              !                                      !
///              !              __/(_____/|             !
///              !            >.___  ____ (             !
///              !                 \(    \|             !
///              !                                      !
///              !                                      !
///              !                                      !
///              +--------------------------------------+
/// ```
///
/// A software rotation correction of 180 degrees should be applied to
/// correctly display the image.
///
/// ```text
///              +--------------------------------------+
///              !                                      !
///              !                                      !
///              !                                      !
///              !             |\____)\___              !
///              !             ) _____  __`<            !
///              !             |/     )/                !
///              !                                      !
///              !                                      !
///              !                                      !
///              +--------------------------------------+
/// ```
///
/// Example two - Phone camera
///
/// A camera installed on the back side of a mobile device facing away from
/// the user. The captured images are meant to be displayed in portrait mode
/// (height > width) to match the device screen orientation and the device
/// usage orientation used when taking the picture.
///
/// The camera sensor is typically mounted with its pixel array longer side
/// aligned to the device longer side, upside-down mounted to compensate for
/// the lens optical inversion effect.
///
/// ```text
///              0        Y-Rc
///            0 +-------------------->
///              !   Y-Rp
///              !    ^
///              !    !
///              !    !
///              !    !
///              !    !            |\_____)\__
///              !    !            ) ____  ___.<
///              !    !            |/    )/
///              !    !
///              !    !
///              !    !
///              !  0 +------------------------------------->
///              !    0                X-Rp
///              !
///              !
///              !
///              !
///              V
///             X-Rc
/// ```
///
/// The two reference systems are not aligned and the 'Rp' reference
/// system is rotated by 90 degrees in the counter-clockwise direction
/// relatively to the 'Rc' reference system.
///
/// The image once captured to memory will be rotated.
///
/// ```text
///              +-------------------------------------+
///              |                 _ _                 |
///              |                \   /                |
///              |                 | |                 |
///              |                 | |                 |
///              |                 |  >                |
///              |                <  |                 |
///              |                 | |                 |
///              |                   .                 |
///              |                  V                  |
///              +-------------------------------------+
/// ```
///
/// A correction of 90 degrees in counter-clockwise direction has to be
/// applied to correctly display the image in portrait mode on the device
/// screen.
///
/// ```text
///                       +--------------------+
///                       |                    |
///                       |                    |
///                       |                    |
///                       |                    |
///                       |                    |
///                       |                    |
///                       |   |\____)\___      |
///                       |   ) _____  __`<    |
///                       |   |/     )/        |
///                       |                    |
///                       |                    |
///                       |                    |
///                       |                    |
///                       |                    |
///                       +--------------------+
#[derive(Debug, Clone)]
pub struct Rotation(pub i32);
impl Deref for Rotation {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Rotation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for Rotation {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<i32>::try_from(value)?))
    }
}
impl From<Rotation> for ControlValue {
    fn from(val: Rotation) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for Rotation {
    const ID: u32 = PropertyId::Rotation as _;
}
impl Property for Rotation {}
/// The model name shall to the extent possible describe the sensor. For
/// most devices this is the model name of the sensor. While for some
/// devices the sensor model is unavailable as the sensor or the entire
/// camera is part of a larger unit and exposed as a black-box to the
/// system. In such cases the model name of the smallest device that
/// contains the camera sensor shall be used.
///
/// The model name is not meant to be a camera name displayed to the
/// end-user, but may be combined with other camera information to create a
/// camera name.
///
/// The model name is not guaranteed to be unique in the system nor is
/// it guaranteed to be stable or have any other properties required to make
/// it a good candidate to be used as a permanent identifier of a camera.
///
/// The model name shall describe the camera in a human readable format and
/// shall be encoded in ASCII.
///
/// Example model names are 'ov5670', 'imx219' or 'Logitech Webcam C930e'.
#[derive(Debug, Clone)]
pub struct Model(pub String);
impl Deref for Model {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Model {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for Model {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<String>::try_from(value)?))
    }
}
impl From<Model> for ControlValue {
    fn from(val: Model) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for Model {
    const ID: u32 = PropertyId::Model as _;
}
impl Property for Model {}
/// The pixel unit cell physical size, in nanometers.
///
/// The UnitCellSize properties defines the horizontal and vertical sizes of
/// a single pixel unit, including its active and non-active parts. In
/// other words, it expresses the horizontal and vertical distance between
/// the top-left corners of adjacent pixels.
///
/// The property can be used to calculate the physical size of the sensor's
/// pixel array area and for calibration purposes.
#[derive(Debug, Clone)]
pub struct UnitCellSize(pub Size);
impl Deref for UnitCellSize {
    type Target = Size;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for UnitCellSize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for UnitCellSize {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<Size>::try_from(value)?))
    }
}
impl From<UnitCellSize> for ControlValue {
    fn from(val: UnitCellSize) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for UnitCellSize {
    const ID: u32 = PropertyId::UnitCellSize as _;
}
impl Property for UnitCellSize {}
/// The camera sensor pixel array readable area vertical and horizontal
/// sizes, in pixels.
///
/// The PixelArraySize property defines the size in pixel units of the
/// readable part of full pixel array matrix, including optical black
/// pixels used for calibration, pixels which are not considered valid for
/// capture and active pixels containing valid image data.
///
/// The property describes the maximum size of the raw data captured by the
/// camera, which might not correspond to the physical size of the sensor
/// pixel array matrix, as some portions of the physical pixel array matrix
/// are not accessible and cannot be transmitted out.
///
/// For example, let's consider a pixel array matrix assembled as follows
///
/// ```text
///      +--------------------------------------------------+
///      |xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx|
///      |xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx|
///      |xxDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDxx|
///      |xxDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDxx|
///      |xxDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDxx|
///      |xxDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDxx|
///      |xxDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDxx|
///      |xxDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDxx|
///      ...          ...           ...      ...          ...
/// ```
///
/// ```text
///      ...          ...           ...      ...          ...
///      |xxDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDxx|
///      |xxDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDxx|
///      |xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx|
///      |xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx|
///      +--------------------------------------------------+
/// ```
///
/// starting with two lines of non-readable pixels (x), followed by N lines
/// of readable data (D) surrounded by two columns of non-readable pixels on
/// each side, and ending with two more lines of non-readable pixels. Only
/// the readable portion is transmitted to the receiving side, defining the
/// sizes of the largest possible buffer of raw data that can be presented
/// to applications.
///
/// ```text
///                      PixelArraySize.width
///        /----------------------------------------------/
///        +----------------------------------------------+ /
///        |DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD| |
///        |DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD| |
///        |DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD| |
///        |DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD| |
///        |DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD| |
///        |DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD| | PixelArraySize.height
///        ...        ...           ...      ...        ...
///        ...        ...           ...      ...        ...
///        |DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD| |
///        |DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD| |
///        +----------------------------------------------+ /
/// ```
///
/// This defines a rectangle whose top-left corner is placed in position (0,
/// 0) and whose vertical and horizontal sizes are defined by this property.
/// All other rectangles that describe portions of the pixel array, such as
/// the optical black pixels rectangles and active pixel areas, are defined
/// relatively to this rectangle.
///
/// All the coordinates are expressed relative to the default sensor readout
/// direction, without any transformation (such as horizontal and vertical
/// flipping) applied. When mapping them to the raw pixel buffer,
/// applications shall take any configured transformation into account.
///
/// \todo Rename this property to Size once we will have property
/// ```text
///       categories (i.e. Properties::PixelArray::Size)
#[derive(Debug, Clone)]
pub struct PixelArraySize(pub Size);
impl Deref for PixelArraySize {
    type Target = Size;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for PixelArraySize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for PixelArraySize {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<Size>::try_from(value)?))
    }
}
impl From<PixelArraySize> for ControlValue {
    fn from(val: PixelArraySize) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for PixelArraySize {
    const ID: u32 = PropertyId::PixelArraySize as _;
}
impl Property for PixelArraySize {}
/// The pixel array region(s) which contain optical black pixels
/// considered valid for calibration purposes.
///
/// This property describes the position and size of optical black pixel
/// regions in the raw data buffer as stored in memory, which might differ
/// from their actual physical location in the pixel array matrix.
///
/// It is important to note, in fact, that camera sensors might
/// automatically reorder or skip portions of their pixels array matrix when
/// transmitting data to the receiver. For instance, a sensor may merge the
/// top and bottom optical black rectangles into a single rectangle,
/// transmitted at the beginning of the frame.
///
/// The pixel array contains several areas with different purposes,
/// interleaved by lines and columns which are said not to be valid for
/// capturing purposes. Invalid lines and columns are defined as invalid as
/// they could be positioned too close to the chip margins or to the optical
/// black shielding placed on top of optical black pixels.
///
/// ```text
///                      PixelArraySize.width
///        /----------------------------------------------/
///           x1                                       x2
///        +--o---------------------------------------o---+ /
///        |IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII| |
///        |IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII| |
///     y1 oIIOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOII| |
///        |IIOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOII| |
///        |IIOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOII| |
///     y2 oIIOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOII| |
///        |IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII| |
///        |IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII| |
///     y3 |IIOOPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPOOII| |
///        |IIOOPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPOOII| | PixelArraySize.height
///        |IIOOPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPOOII| |
///        ...          ...           ...     ...       ...
///        ...          ...           ...     ...       ...
///     y4 |IIOOPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPOOII| |
///        |IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII| |
///        |IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII| |
///        +----------------------------------------------+ /
/// ```
///
/// The readable pixel array matrix is composed by
/// 2 invalid lines (I)
/// 4 lines of valid optical black pixels (O)
/// 2 invalid lines (I)
/// n lines of valid pixel data (P)
/// 2 invalid lines (I)
///
/// And the position of the optical black pixel rectangles is defined by
///
/// ```text
///     PixelArrayOpticalBlackRectangles = {
///        { x1, y1, x2 - x1 + 1, y2 - y1 + 1 },
///        { x1, y3, 2, y4 - y3 + 1 },
///        { x2, y3, 2, y4 - y3 + 1 },
///     };
/// ```
///
/// If the camera, when capturing the full pixel array matrix, automatically
/// skips the invalid lines and columns, producing the following data
/// buffer, when captured to memory
///
/// ```text
///                      PixelArraySize.width
///        /----------------------------------------------/
///                                                    x1
///        +--------------------------------------------o-+ /
///        |OOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO| |
///        |OOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO| |
///        |OOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO| |
///        |OOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO| |
///     y1 oOOPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPOO| |
///        |OOPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPOO| |
///        |OOPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPOO| | PixelArraySize.height
///        ...       ...          ...       ...         ... |
///        ...       ...          ...       ...         ... |
///        |OOPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPOO| |
///        |OOPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPOO| |
///        +----------------------------------------------+ /
/// ```
///
/// then the invalid lines and columns should not be reported as part of the
/// PixelArraySize property in first place.
///
/// In this case, the position of the black pixel rectangles will be
///
/// ```text
///     PixelArrayOpticalBlackRectangles = {
///        { 0, 0, y1 + 1, PixelArraySize[0] },
///        { 0, y1, 2, PixelArraySize[1] - y1 + 1 },
///        { x1, y1, 2, PixelArraySize[1] - y1 + 1 },
///     };
/// ```
///
/// \todo Rename this property to Size once we will have property
/// ```text
///       categories (i.e. Properties::PixelArray::OpticalBlackRectangles)
#[derive(Debug, Clone)]
pub struct PixelArrayOpticalBlackRectangles(pub Vec<Rectangle>);
impl Deref for PixelArrayOpticalBlackRectangles {
    type Target = Vec<Rectangle>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for PixelArrayOpticalBlackRectangles {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for PixelArrayOpticalBlackRectangles {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<Vec<Rectangle>>::try_from(value)?))
    }
}
impl From<PixelArrayOpticalBlackRectangles> for ControlValue {
    fn from(val: PixelArrayOpticalBlackRectangles) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for PixelArrayOpticalBlackRectangles {
    const ID: u32 = PropertyId::PixelArrayOpticalBlackRectangles as _;
}
impl Property for PixelArrayOpticalBlackRectangles {}
/// The PixelArrayActiveAreas property defines the (possibly multiple and
/// overlapping) portions of the camera sensor readable pixel matrix
/// which are considered valid for image acquisition purposes.
///
/// This property describes an arbitrary number of overlapping rectangles,
/// with each rectangle representing the maximum image size that the camera
/// sensor can produce for a particular aspect ratio. They are defined
/// relatively to the PixelArraySize rectangle.
///
/// When multiple rectangles are reported, they shall be ordered from the
/// tallest to the shortest.
///
/// Example 1
/// A camera sensor which only produces images in the 4:3 image resolution
/// will report a single PixelArrayActiveAreas rectangle, from which all
/// other image formats are obtained by either cropping the field-of-view
/// and/or applying pixel sub-sampling techniques such as pixel skipping or
/// binning.
///
/// ```text
///            PixelArraySize.width
///             /----------------/
///               x1          x2
///     (0,0)-> +-o------------o-+  /
///          y1 o +------------+ |  |
///             | |////////////| |  |
///             | |////////////| |  | PixelArraySize.height
///             | |////////////| |  |
///          y2 o +------------+ |  |
///             +----------------+  /
/// ```
///
/// The property reports a single rectangle
///
/// ```text
///          PixelArrayActiveAreas = (x1, y1, x2 - x1 + 1, y2 - y1 + 1)
/// ```
///
/// Example 2
/// A camera sensor which can produce images in different native
/// resolutions will report several overlapping rectangles, one for each
/// natively supported resolution.
///
/// ```text
///              PixelArraySize.width
///             /------------------/
///               x1  x2    x3  x4
///     (0,0)-> +o---o------o---o+  /
///          y1 o    +------+    |  |
///             |    |//////|    |  |
///          y2 o+---+------+---+|  |
///             ||///|//////|///||  | PixelArraySize.height
///          y3 o+---+------+---+|  |
///             |    |//////|    |  |
///          y4 o    +------+    |  |
///             +----+------+----+  /
/// ```
///
/// The property reports two rectangles
///
/// ```text
///         PixelArrayActiveAreas = ((x2, y1, x3 - x2 + 1, y4 - y1 + 1),
///                                  (x1, y2, x4 - x1 + 1, y3 - y2 + 1))
/// ```
///
/// The first rectangle describes the maximum field-of-view of all image
/// formats in the 4:3 resolutions, while the second one describes the
/// maximum field of view for all image formats in the 16:9 resolutions.
///
/// Multiple rectangles shall only be reported when the sensor can't capture
/// the pixels in the corner regions. If all the pixels in the (x1,y1) -
/// (x4,y4) area can be captured, the PixelArrayActiveAreas property shall
/// contains the single rectangle (x1,y1) - (x4,y4).
///
/// \todo Rename this property to ActiveAreas once we will have property
/// ```text
///       categories (i.e. Properties::PixelArray::ActiveAreas)
#[derive(Debug, Clone)]
pub struct PixelArrayActiveAreas(pub Vec<Rectangle>);
impl Deref for PixelArrayActiveAreas {
    type Target = Vec<Rectangle>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for PixelArrayActiveAreas {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for PixelArrayActiveAreas {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<Vec<Rectangle>>::try_from(value)?))
    }
}
impl From<PixelArrayActiveAreas> for ControlValue {
    fn from(val: PixelArrayActiveAreas) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for PixelArrayActiveAreas {
    const ID: u32 = PropertyId::PixelArrayActiveAreas as _;
}
impl Property for PixelArrayActiveAreas {}
/// The maximum valid rectangle for the controls::ScalerCrop control. This
/// reflects the minimum mandatory cropping applied in the camera sensor and
/// the rest of the pipeline. Just as the ScalerCrop control, it defines a
/// rectangle taken from the sensor's active pixel array.
///
/// This property is valid only after the camera has been successfully
/// configured and its value may change whenever a new configuration is
/// applied.
///
/// \todo Turn this property into a "maximum control value" for the
/// ScalerCrop control once "dynamic" controls have been implemented.
#[derive(Debug, Clone)]
pub struct ScalerCropMaximum(pub Rectangle);
impl Deref for ScalerCropMaximum {
    type Target = Rectangle;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for ScalerCropMaximum {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for ScalerCropMaximum {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<Rectangle>::try_from(value)?))
    }
}
impl From<ScalerCropMaximum> for ControlValue {
    fn from(val: ScalerCropMaximum) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for ScalerCropMaximum {
    const ID: u32 = PropertyId::ScalerCropMaximum as _;
}
impl Property for ScalerCropMaximum {}
/// The relative sensitivity of the chosen sensor mode.
///
/// Some sensors have readout modes with different sensitivities. For example,
/// a binned camera mode might, with the same exposure and gains, produce
/// twice the signal level of the full resolution readout. This would be
/// signalled by the binned mode, when it is chosen, indicating a value here
/// that is twice that of the full resolution mode. This value will be valid
/// after the configure method has returned successfully.
#[derive(Debug, Clone)]
pub struct SensorSensitivity(pub f32);
impl Deref for SensorSensitivity {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for SensorSensitivity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for SensorSensitivity {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<f32>::try_from(value)?))
    }
}
impl From<SensorSensitivity> for ControlValue {
    fn from(val: SensorSensitivity) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for SensorSensitivity {
    const ID: u32 = PropertyId::SensorSensitivity as _;
}
impl Property for SensorSensitivity {}
/// A list of integer values of type dev_t denoting the major and minor
/// device numbers of the underlying devices used in the operation of this
/// camera.
///
/// Different cameras may report identical devices.
#[derive(Debug, Clone)]
pub struct SystemDevices(pub Vec<i64>);
impl Deref for SystemDevices {
    type Target = Vec<i64>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for SystemDevices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<ControlValue> for SystemDevices {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Ok(Self(<Vec<i64>>::try_from(value)?))
    }
}
impl From<SystemDevices> for ControlValue {
    fn from(val: SystemDevices) -> Self {
        ControlValue::from(val.0)
    }
}
impl ControlEntry for SystemDevices {
    const ID: u32 = PropertyId::SystemDevices as _;
}
impl Property for SystemDevices {}
/// The arrangement of color filters on sensor; represents the colors in the
/// top-left 2x2 section of the sensor, in reading order. Currently
/// identical to ANDROID_SENSOR_INFO_COLOR_FILTER_ARRANGEMENT.
#[cfg(feature = "vendor_draft")]
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ColorFilterArrangement {
    /// RGGB Bayer pattern
    RGGB = 0,
    /// GRBG Bayer pattern
    GRBG = 1,
    /// GBRG Bayer pattern
    GBRG = 2,
    /// BGGR Bayer pattern
    BGGR = 3,
    /// Sensor is not Bayer; output has 3 16-bit values for each pixel,
    /// instead of just 1 16-bit value per pixel.
    RGB = 4,
    /// Sensor is not Bayer; output consists of a single colour channel.
    MONO = 5,
}
#[cfg(feature = "vendor_draft")]
impl TryFrom<ControlValue> for ColorFilterArrangement {
    type Error = ControlValueError;
    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {
        Self::try_from(i32::try_from(value.clone())?)
            .map_err(|_| ControlValueError::UnknownVariant(value))
    }
}
#[cfg(feature = "vendor_draft")]
impl From<ColorFilterArrangement> for ControlValue {
    fn from(val: ColorFilterArrangement) -> Self {
        ControlValue::from(<i32>::from(val))
    }
}
#[cfg(feature = "vendor_draft")]
impl ControlEntry for ColorFilterArrangement {
    const ID: u32 = PropertyId::ColorFilterArrangement as _;
}
#[cfg(feature = "vendor_draft")]
impl Property for ColorFilterArrangement {}
pub fn make_dyn(
    id: PropertyId,
    val: ControlValue,
) -> Result<Box<dyn DynControlEntry>, ControlValueError> {
    match id {
        PropertyId::Location => Ok(Box::new(Location::try_from(val)?)),
        PropertyId::Rotation => Ok(Box::new(Rotation::try_from(val)?)),
        PropertyId::Model => Ok(Box::new(Model::try_from(val)?)),
        PropertyId::UnitCellSize => Ok(Box::new(UnitCellSize::try_from(val)?)),
        PropertyId::PixelArraySize => Ok(Box::new(PixelArraySize::try_from(val)?)),
        PropertyId::PixelArrayOpticalBlackRectangles => {
            Ok(Box::new(PixelArrayOpticalBlackRectangles::try_from(val)?))
        }
        PropertyId::PixelArrayActiveAreas => {
            Ok(Box::new(PixelArrayActiveAreas::try_from(val)?))
        }
        PropertyId::ScalerCropMaximum => Ok(Box::new(ScalerCropMaximum::try_from(val)?)),
        PropertyId::SensorSensitivity => Ok(Box::new(SensorSensitivity::try_from(val)?)),
        PropertyId::SystemDevices => Ok(Box::new(SystemDevices::try_from(val)?)),
        #[cfg(feature = "vendor_draft")]
        PropertyId::ColorFilterArrangement => {
            Ok(Box::new(ColorFilterArrangement::try_from(val)?))
        }
    }
}
