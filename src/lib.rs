mod bindings;
pub mod decoder;
pub mod encoder;

use bindings::*;
use libc::c_int;

pub const OPUS_APPLICATION_VOIP: u32 = 2048;
pub const OPUS_APPLICATION_AUDIO: u32 = 2049;
pub const OPUS_APPLICATION_RESTRICTED_LOW_DELAY: u32 = 2051;

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