use libcamera_sys::*;

#[derive(Debug, Clone, Copy)]
pub enum StreamRole {
    Raw,
    StillCapture,
    VideoRecording,
    ViewFinder,
}

impl TryFrom<libcamera_stream_role::Type> for StreamRole {
    type Error = ();

    fn try_from(value: libcamera_stream_role::Type) -> Result<Self, Self::Error> {
        match value {
            libcamera_stream_role::LIBCAMERA_STREAM_ROLE_RAW => Ok(StreamRole::Raw),
            libcamera_stream_role::LIBCAMERA_STREAM_ROLE_STILL_CAPTURE => Ok(StreamRole::StillCapture),
            libcamera_stream_role::LIBCAMERA_STREAM_ROLE_VIDEO_RECORDING => Ok(StreamRole::VideoRecording),
            libcamera_stream_role::LIBCAMERA_STREAM_ROLE_VIEW_FINDER => Ok(StreamRole::ViewFinder),
            _ => Err(()),
        }
    }
}

impl From<StreamRole> for libcamera_stream_role::Type {
    fn from(role: StreamRole) -> Self {
        match role {
            StreamRole::Raw => libcamera_stream_role::LIBCAMERA_STREAM_ROLE_RAW,
            StreamRole::StillCapture => libcamera_stream_role::LIBCAMERA_STREAM_ROLE_STILL_CAPTURE,
            StreamRole::VideoRecording => libcamera_stream_role::LIBCAMERA_STREAM_ROLE_VIDEO_RECORDING,
            StreamRole::ViewFinder => libcamera_stream_role::LIBCAMERA_STREAM_ROLE_VIEW_FINDER,
        }
    }
}
