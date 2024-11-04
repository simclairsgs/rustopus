use libc::c_int;
use crate::bindings::{opus_encode, opus_encode_float, opus_encoder_create, opus_encoder_ctl, opus_encoder_destroy, OPUS_OK, OPUS_SET_BITRATE_REQUEST, OPUS_SET_COMPLEXITY_REQUEST};
use crate::{bindings, OpusError};

pub struct OpusEncoder {
    is_stereo : bool,
    sample_rate : u32,
    opus_application_type : u32,
    encoder: *mut bindings::OpusEncoder,
}

impl OpusEncoder {
    pub fn new(sample_rate: u32, opus_application_type : u32, is_stereo : bool) -> Result<Self, OpusError> {
        let mut error: c_int = 0;

        let mut channel_count : c_int = 1;
        if is_stereo {
            channel_count = 2;
        }

        let encoder = unsafe { opus_encoder_create(sample_rate as c_int,  channel_count, opus_application_type as c_int, &mut error) };
        if encoder.is_null() || error != OPUS_OK as i32 {
            return Err(OpusError::from(error));
        }

        Ok(OpusEncoder { is_stereo, sample_rate, opus_application_type, encoder })
    }

    pub fn set_bitrate(&mut self, bitrate : u32) -> Result<(), OpusError>{
        let ret = unsafe { opus_encoder_ctl(self.encoder, OPUS_SET_BITRATE_REQUEST as c_int, bitrate as c_int) };
        if ret != OPUS_OK as i32 {
            return Err(OpusError::from(ret));
        }
        Ok(())
    }

    pub fn set_complexity(&mut self, complexity : u8)->Result<(), OpusError>{
        let ret = unsafe { opus_encoder_ctl(self.encoder, OPUS_SET_COMPLEXITY_REQUEST as c_int, complexity as c_int) };
        if ret != OPUS_OK as i32 {
            return Err(OpusError::from(ret));
        }
        Ok(())
    }

    pub fn encode(&self, pcm: &[i16], opus: &mut [u8]) -> Result<usize, OpusError> {
        let mut frame_size = pcm.len();

        if self.is_stereo {
            frame_size = frame_size / 2;
        }

        let ret = unsafe {
            opus_encode(
                self.encoder,
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

    pub fn encode_float(&self, pcm: &[f32], opus: &mut [u8]) -> Result<usize, OpusError> {
        let mut frame_size = pcm.len();

        if self.is_stereo {
            frame_size = frame_size / 2;
        }

        let ret = unsafe {
            opus_encode_float(
                self.encoder,
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
        unsafe {
            opus_encoder_destroy(self.encoder);
        }
    }
}