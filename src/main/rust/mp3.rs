use jni::sys::{jlong, jint};
use jni::objects::{JByteBuffer, JClass};
use jni::JNIEnv;
use log::debug;
use mpg123_sys::*;

#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_mp3_Mp3DecoderLibrary_create(_: JNIEnv, _: JClass) -> jlong {
    debug!("(mp3) create");

    mpg123_init();

    let handle = mpg123_new(std::ptr::null_mut(), std::ptr::null_mut());
    if handle.is_null() {
        return 0
    }

    if mpg123_open_feed(handle) != 0 {
        mpg123_delete(handle);
        return 0
    };

    handle as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_mp3_Mp3DecoderLibrary_destroy(_: JNIEnv, _: JClass, instance: jlong) {
    debug!("(mp3) destroy, instance: {}", instance);

    let handle = instance as *mut mpg123_handle;
    if !handle.is_null() {
        mpg123_close(handle);
        mpg123_delete(handle);
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_mp3_Mp3DecoderLibrary_decode(
    mut env: JNIEnv,
    _: JClass,
    instance: jlong,
    input_buffer: JByteBuffer,
    input_length: jint,
    output_buffer: JByteBuffer,
    output_length: jint,
) -> jlong {
    debug!("(mp3) decode, instance: {}, input_length: {}, output_length: {}", instance, input_length, output_length);

    if instance == 0 {
        return -1;
    }

    env.with_env(|env| -> jni::errors::Result<jlong> {
        let input_ptr = env.get_direct_buffer_address(&input_buffer)?;
        let output_ptr = env.get_direct_buffer_address(&output_buffer)?;

        let mut used_bytes = 0;
        let result = mpg123_decode(
            instance as *mut mpg123_handle,
            input_ptr,
            input_length as usize,
            output_ptr,
            output_length as usize,
            &mut used_bytes
        ) as jlong;

        if result != 0 {
            if result > 0 {
                return Ok(-(result + 100));
            } else {
                return Ok(result);
            }
        }

        Ok(used_bytes as jlong)
    }).resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}
