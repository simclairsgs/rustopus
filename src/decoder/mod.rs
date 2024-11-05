use std::ptr;
use core::ffi::c_int;
use crate::{bindings, OpusError};
use crate::bindings::{opus_decode, opus_decode_float, opus_decoder_create, opus_decoder_destroy, opus_int32, OPUS_OK};

#[derive(Debug)]
pub struct OpusDecoder {
    sample_rate : u32,
    is_stereo : bool,
    frame_size : u16,
    decoder : *mut bindings::OpusDecoder
}

impl OpusDecoder {
    pub fn new(sample_rate : u32, is_stereo : bool, frame_size : u16)->Result<Self, OpusError>{
        let mut error: c_int = 0;

        let mut channel_count : c_int = 1;
        if is_stereo {
            channel_count = 2;
        }
        let decoder = unsafe { opus_decoder_create(sample_rate as c_int,  channel_count, &mut error) };
        if decoder.is_null() || error != OPUS_OK as i32 {
            return Err(OpusError::from(error));
        }

        Ok(OpusDecoder { is_stereo, sample_rate, decoder, frame_size })
    }

    pub fn decode(&self, opus : &[u8], pcm : &mut [i16])->Result<usize, OpusError>{

        let ret = unsafe {
            opus_decode(
                self.decoder,
                opus.as_ptr(),
                opus.len() as opus_int32,
                pcm.as_mut_ptr(),
                self.frame_size as c_int,
                0
            )
        };

        if ret < 0 {
            Err(OpusError::from(ret))
        } else {
            Ok(ret as usize)
        }
    }

    pub fn decode_float(&self, opus : &[u8], pcm : &mut [f32])->Result<usize, OpusError>{
        let mut frame_size = pcm.len();

        if self.is_stereo {
            frame_size = frame_size / 2;
        }

        let ret = unsafe {
            opus_decode_float(
                self.decoder,
                opus.as_ptr(),
                opus.len() as opus_int32,
                pcm.as_mut_ptr(),
                frame_size as c_int,
                0
            )
        };

        if ret < 0 {
            Err(OpusError::from(ret))
        } else {
            Ok(ret as usize)
        }
    }

    pub fn notify_loss_plc(&self, pcm : &mut [f32]) -> Result<usize, OpusError>{
        let ret = unsafe {
            opus_decode_float(
                self.decoder,
                ptr::null(),
                0,
                pcm.as_mut_ptr(),
                self.frame_size as c_int,
                0
            )
        };

        if ret < 0 {
            Err(OpusError::from(ret))
        } else {
            Ok(ret as usize)
        }
    }
}

impl Drop for OpusDecoder{
    fn drop(&mut self) {
        unsafe {
            opus_decoder_destroy(self.decoder);
        }
    }
}