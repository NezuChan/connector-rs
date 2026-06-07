use std::os::raw::c_void;
use std::slice;
use jni::JNIEnv;
use jni::errors::Result;
use jni::objects::JByteBuffer;

pub fn get_direct_short_buffer_address<'a>(jni: &JNIEnv, buf: &JByteBuffer) -> Result<&'a mut [i16]> {
    let ptr = jni.get_direct_buffer_address(buf)?;
    let capacity = jni.get_direct_buffer_capacity(buf)?;
    
    unsafe { Ok(slice::from_raw_parts_mut(ptr as *mut i16, (capacity / 2) as usize)) }
}

pub fn get_direct_buffer_address<'a>(jni: &JNIEnv, buf: &JByteBuffer) -> Result<&'a mut [u8]> {
    let ptr = jni.get_direct_buffer_address(buf)?;
    let capacity = jni.get_direct_buffer_capacity(buf)?;
    
    unsafe { Ok(slice::from_raw_parts_mut(ptr, capacity as usize)) }
}
