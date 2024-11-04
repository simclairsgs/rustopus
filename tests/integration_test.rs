use rustopus::decoder::OpusDecoder;
use rustopus::encoder::OpusEncoder;
use rustopus::{OPUS_APPLICATION_AUDIO, OPUS_APPLICATION_RESTRICTED_LOW_DELAY};

#[test]
fn encoder_creation_test() {
    let result = OpusEncoder::new(48000, OPUS_APPLICATION_RESTRICTED_LOW_DELAY, false);
    println!("ENCODER {:?}", result);
    assert!(result.is_ok(), "encoder creation failed")
}

#[test]
fn decoder_creation_test() {
    let result = OpusDecoder::new(48000, false, 512);
    println!("DECODER {:?}", result);
    assert!(result.is_ok(), "decoder creation failed")
}

#[test]
fn test_opus_cycle(){
    let encoder = OpusEncoder::new(48000, OPUS_APPLICATION_AUDIO, false).unwrap();
    let pcm = [1i16 ;960];
    let mut opus = [0u8 ;960];
    let res = encoder.encode(&pcm, &mut opus);
    assert!(res.is_ok(), " opus encoding error ");
    let s = res.unwrap();
    println!("ENCODED SIZE {s}");
    let decoder = OpusDecoder::new(48000, false, 480).unwrap();
    let mut pc = [0f32; 960];
    let res = decoder.decode_float(&opus, &mut pc);
    assert!(res.is_ok(), " opus decoding error ");
    println!("DECODED SIZE {}", res.unwrap());
    let res = decoder.notify_loss_plc(&mut pc);
    assert!(res.is_ok(), " opus plc decoding error ");
    println!("DECODED SIZE 1 {}", res.unwrap());
}