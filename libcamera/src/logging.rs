use std::{
    ffi::{CStr, CString},
    io,
};

use libcamera_sys::*;

use crate::utils::handle_result;

/// Log destination type.
#[derive(Copy, Clone, Debug)]
pub enum LoggingTarget {
    None,
    Syslog,
}

impl From<LoggingTarget> for libcamera_logging_target_t {
    fn from(value: LoggingTarget) -> Self {
        match value {
            LoggingTarget::None => libcamera_logging_target::LIBCAMERA_LOGGING_TARGET_NONE,
            LoggingTarget::Syslog => libcamera_logging_target::LIBCAMERA_LOGGING_TARGET_SYSLOG,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum LoggingLevel {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

impl From<LoggingLevel> for &'static CStr {
    fn from(value: LoggingLevel) -> Self {
        match value {
            LoggingLevel::Debug => CStr::from_bytes_with_nul(b"DEBUG\0").expect("Static null-terminated string"),
            LoggingLevel::Info => CStr::from_bytes_with_nul(b"INFO\0").expect("Static null-terminated string"),
            LoggingLevel::Warn => CStr::from_bytes_with_nul(b"WARN\0").expect("Static null-terminated string"),
            LoggingLevel::Error => CStr::from_bytes_with_nul(b"ERROR\0").expect("Static null-terminated string"),
            LoggingLevel::Fatal => CStr::from_bytes_with_nul(b"FATAL\0").expect("Static null-terminated string"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum LoggingStream {
    StdOut,
    StdErr,
}

impl From<LoggingStream> for libcamera_logging_stream_t {
    fn from(value: LoggingStream) -> Self {
        match value {
            LoggingStream::StdOut => libcamera_logging_stream::LIBCAMERA_LOGGING_STREAM_STDOUT,
            LoggingStream::StdErr => libcamera_logging_stream::LIBCAMERA_LOGGING_STREAM_STDERR,
        }
    }
}

/// Direct logging to a file.
pub fn log_set_file(file: &str, color: bool) -> io::Result<()> {
    let file = CString::new(file).expect("file contains null byte");
    let ret = unsafe { libcamera_log_set_file(file.as_ptr(), color) };
    handle_result(ret)
}

/// Direct logging to a stream.
pub fn log_set_stream(stream: LoggingStream, color: bool) -> io::Result<()> {
    let ret = unsafe { libcamera_log_set_stream(stream.into(), color) };
    handle_result(ret)
}

/// Set the logging target.
pub fn log_set_target(target: LoggingTarget) -> io::Result<()> {
    let ret = unsafe { libcamera_log_set_target(target.into()) };
    handle_result(ret)
}
