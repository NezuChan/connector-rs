use std::os::raw::c_long;
use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::{jboolean, jdouble, jfloatArray, jint, jintArray, jlong};
use libsamplerate::*;
use log::debug;

#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_samplerate_SampleRateLibrary_create(
    _: JNIEnv,
    _: JClass,
    src_type: jint,
    channels: jint,
) -> jlong {
    debug!("(samplerate) create, src_type: {}, channels: {}", src_type, channels);

    let mut error = 0;
    let handle = src_new(src_type, channels, &mut error) as jlong;
    debug!("(samplerate) new: {}", handle);

    handle
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_samplerate_SampleRateLibrary_destroy(
    _: JNIEnv,
    _: JClass,
    instance: jlong,
) {
    debug!("(samplerate) destroy, handle: {}", instance);

    let handle = instance as *mut SRC_STATE;

    /* destroy given instance */
    src_delete(handle);
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_samplerate_SampleRateLibrary_reset(
    _: JNIEnv,
    _: JClass,
    instance: jlong,
) {
    debug!("(samplerate) reset, handle: {}", instance);
    src_reset(instance as *mut SRC_STATE);
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_samplerate_SampleRateLibrary_process(
    mut env: JNIEnv,
    _: JClass,
    instance: jlong,
    input_array: jfloatArray,
    input_offset: jint,
    input_length: jint,
    output_array: jfloatArray,
    output_offset: jint,
    output_length: jint,
    eof: jboolean,
    source_ratio: jdouble,
    progress: jintArray,
) -> jint {
    debug!(
        "(samplerate) process, handle: {}, output_length: {}, output_offset: {}, input_length: {}, input_offset: {}, source_ratio: {}, is eof: {}",
        instance,
        output_length,
        output_offset,
        input_length,
        input_offset,
        source_ratio,
        eof,
    );

    env.with_env(|env| -> jni::errors::Result<jint> {
        let jni_env = env.get_raw();

        // Get input array elements using raw JNI
        let in_ptr = ((**jni_env).v1_1.GetFloatArrayElements)(jni_env, input_array, std::ptr::null_mut());
        if in_ptr.is_null() {
            return Ok(-1);
        }
        let in_size = ((**jni_env).v1_1.GetArrayLength)(jni_env, input_array) as usize;
        let input = std::slice::from_raw_parts(in_ptr, in_size);

        // Get output array elements using raw JNI
        let out_ptr = ((**jni_env).v1_1.GetFloatArrayElements)(jni_env, output_array, std::ptr::null_mut());
        if out_ptr.is_null() {
            ((**jni_env).v1_1.ReleaseFloatArrayElements)(jni_env, input_array, in_ptr, 0);
            return Ok(-1);
        }
        let out_size = ((**jni_env).v1_1.GetArrayLength)(jni_env, output_array) as usize;
        let output = std::slice::from_raw_parts_mut(out_ptr, out_size);

        let mut src_data = SRC_DATA {
            data_in: input[input_offset as usize..].as_ptr(),
            input_frames: input_length as c_long,
            input_frames_used: 0,
            end_of_input: eof as i32,
            data_out: output[output_offset as usize..].as_mut_ptr(),
            output_frames: output_length as c_long,
            output_frames_gen: 0,
            src_ratio: source_ratio,
        };

        let result = src_process(instance as *mut SRC_STATE, &mut src_data);
        let prog = [src_data.input_frames_used as jint, src_data.output_frames_gen as jint];

        // Write progress using raw JNI
        ((**jni_env).v1_1.SetIntArrayRegion)(jni_env, progress, 0, 2, prog.as_ptr());

        // Release arrays
        ((**jni_env).v1_1.ReleaseFloatArrayElements)(jni_env, input_array, in_ptr, 0);
        ((**jni_env).v1_1.ReleaseFloatArrayElements)(jni_env, output_array, out_ptr, 0);

        Ok(result)
    }).resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}
