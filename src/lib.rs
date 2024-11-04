mod bindings;
mod decoder;
mod encoder;

use std::ffi::c_void;
use bindings::*;
use libc::c_int;

/// Errors returned by the Opus codec.
#[derive(Debug)]
pub enum OpusError {
    InvalidArgument,
    BufferTooSmall,
    InternalError,
    UnknownError(c_int),
}

impl From<c_int> for OpusError {
    fn from(err_code: c_int) -> Self {
        match err_code {
            OPUS_BAD_ARG => OpusError::InvalidArgument,
            OPUS_BUFFER_TOO_SMALL => OpusError::BufferTooSmall,
            OPUS_INTERNAL_ERROR => OpusError::InternalError,
            code => OpusError::UnknownError(code),
        }
    }
}