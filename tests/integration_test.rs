use rustopus::encoder::OpusEncoder;
use rustopus::OPUS_APPLICATION_AUDIO;

#[test]
fn encoder_creation_test() {
    let result = OpusEncoder::new(48000, OPUS_APPLICATION_AUDIO, false);
    println!("ENCODER {:?}", result);
}