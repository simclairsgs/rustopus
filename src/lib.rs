mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use std::ffi::c_void;
use std::ptr;
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

/// A safe wrapper around the Opus encoder.
pub struct OpusEncoder {
    encoder: *mut c_void,
}

impl OpusEncoder {
    /// Creates a new Opus encoder.
    ///
    /// # Arguments
    /// * `sample_rate` - The sample rate of the audio (usually 48000).
    /// * `channels` - Number of audio channels (1 for mono, 2 for stereo).
    ///
    /// # Errors
    /// Returns an `OpusError` if the encoder could not be created.
    pub fn new(sample_rate: c_int, channels: c_int) -> Result<Self, OpusError> {
        let mut error: c_int = 0;

        // Create the Opus encoder
        let encoder = unsafe { opus_encoder_create(sample_rate, channels, OPUS_APPLICATION_AUDIO as c_int, &mut error) };

        if encoder.is_null() || error != OPUS_OK as i32 {
            Err(OpusError::from(error))
        } else {
            Ok(OpusEncoder { encoder : encoder as *mut c_void })
        }
    }

    /// Encodes PCM audio data into Opus format.
    ///
    /// # Arguments
    /// * `pcm` - A slice of PCM audio samples.
    /// * `opus` - A buffer to hold the encoded Opus data.
    ///
    /// # Returns
    /// Returns the number of bytes written to `opus` on success, or an `OpusError` on failure.
    pub fn encode(&self, pcm: &[i16], opus: &mut [u8]) -> Result<usize, OpusError> {
        let frame_size = pcm.len() / 2; // Assuming stereo input

        let ret = unsafe {
            opus_encode(
                self.encoder as *mut bindings::OpusEncoder,
                pcm.as_ptr(),
                frame_size as c_int,
                opus.as_mut_ptr(),
                opus.len() as c_int,
            )
        };

        if ret < 0 {
            Err(OpusError::from(ret))
        } else {
            Ok(ret as usize)
        }
    }
}

impl Drop for OpusEncoder {
    fn drop(&mut self) {
        // Clean up the encoder when it goes out of scope
        unsafe {
            opus_encoder_destroy(self.encoder as *mut bindings::OpusEncoder);
        }
    }
}