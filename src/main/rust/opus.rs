use audiopus_sys::{opus_decode, opus_decoder_create, opus_decoder_destroy, opus_encode, opus_encoder_create, opus_encoder_ctl, opus_encoder_destroy, OPUS_OK, OPUS_SET_COMPLEXITY_REQUEST, OpusDecoder, OpusEncoder};
use jni::JNIEnv;
use jni::objects::{JByteBuffer, JClass};
use jni::sys::{jint, jlong};
use log::debug;

use crate::util::get_direct_short_buffer_address;

type OpusDecoderHandle = *mut OpusDecoder;
type OpusEncoderHandle = *mut OpusEncoder;

// decoder
#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_opus_OpusDecoderLibrary_create(
    _: JNIEnv,
    _: JClass,
    sample_rate: jint,
    channel_count: jint,
) -> jlong {
    debug!("(opus) decoder:create, sample_rate: {}, channel_count: {}", sample_rate, channel_count);

    /* create the decoder */
    let mut opus_code = 0;
    let decoder = opus_decoder_create(sample_rate, channel_count, &mut opus_code);

    /* check for errors. */
    if opus_code == OPUS_OK || !decoder.is_null() {
        /* return the pointer? */
        return decoder as jlong;
    };

    opus_code as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_opus_OpusDecoderLibrary_destroy(
    _: JNIEnv,
    _: JClass,
    decoder_ptr: jlong,
) {
    debug!("(opus) decoder:destroy, instance: {}", decoder_ptr);

    let ptr = decoder_ptr as OpusDecoderHandle;
    if !ptr.is_null() {
        let decoder = from_ptr!(ptr);
        unsafe { opus_decoder_destroy(decoder) }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_opus_OpusDecoderLibrary_decode(
    mut env: JNIEnv,
    _: JClass,
    decoder_ptr: jlong,
    input_buffer: JByteBuffer,
    input_size: jint,
    output_buffer: JByteBuffer,
    frame_size: jint,
) -> jint {
    debug!("(opus) decoder:decode, instance: {}, input_size: {}, frame_size: {}", decoder_ptr, input_size, frame_size);

    let decoder = from_ptr!(decoder_ptr as OpusDecoderHandle);

    env.with_env(|mut env| -> jni::errors::Result<jint> {
        let input_ptr = env.get_direct_buffer_address(&input_buffer)?;
        
        let output_ptr = env.get_direct_buffer_address(&output_buffer)?;
        let output_capacity = env.get_direct_buffer_capacity(&output_buffer)?;
        let output = unsafe { std::slice::from_raw_parts_mut(output_ptr as *mut i16, (output_capacity / 2) as usize) };

        Ok(unsafe { opus_decode(decoder, input_ptr as *const u8, input_size, output.as_mut_ptr(), frame_size, 0) as i32 })
    }).resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

// encoder
#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_opus_OpusEncoderLibrary_create(
    _: JNIEnv,
    _: JClass,
    sample_rate: jint,
    channel_count: jint,
    application: jint,
    quality: jint,
) -> jlong {
    debug!("(opus) encoder:create, sample_rate: {}, channel_count: {}, application: {}, quality: {}", sample_rate, channel_count, application, quality);

    let mut opus_code = 0;
    let encoder = opus_encoder_create(sample_rate, channel_count, application, &mut opus_code);

    if opus_code == OPUS_OK || !encoder.is_null() {
        opus_encoder_ctl(encoder, OPUS_SET_COMPLEXITY_REQUEST, quality);
        return encoder as jlong;
    };

    opus_code as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_opus_OpusEncoderLibrary_destroy(
    _: JNIEnv,
    _: JClass,
    encoder_ptr: jlong,
) {
    debug!("(opus) encoder:destroy, instance: {}", encoder_ptr);

    let ptr = encoder_ptr as OpusEncoderHandle;
    if !ptr.is_null() {
        let encoder = from_ptr!(ptr);
        unsafe { opus_encoder_destroy(encoder) }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_opus_OpusEncoderLibrary_encode(
    mut jni: JNIEnv,
    _: JClass,
    encoder_ptr: jlong,
    input_buffer: JByteBuffer,
    frame_size: jint,
    output_buffer: JByteBuffer,
    output_capacity: jint,
) -> jint {
    debug!("(opus) encoder:encode, instance: {}, frame_size: {}, output_capacity: {}", encoder_ptr, frame_size, output_capacity);

    let encoder = from_ptr!(encoder_ptr as OpusEncoderHandle);

    jni.with_env(|mut env| -> jni::errors::Result<jint> {
        let input_ptr = env.get_direct_buffer_address(&input_buffer)?;
        let input_capacity = env.get_direct_buffer_capacity(&input_buffer)?;
        let input = unsafe { std::slice::from_raw_parts_mut(input_ptr as *mut i16, (input_capacity / 2) as usize) };
        
        let output_ptr = env.get_direct_buffer_address(&output_buffer)?;

        Ok(unsafe { opus_encode(encoder, input.as_ptr(), frame_size, output_ptr, output_capacity) as i32 })
    }).resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}
