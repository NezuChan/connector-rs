use std::slice;
use jni::Env;
use jni::errors::Result;
use jni::objects::JByteBuffer;

#[allow(dead_code)]
pub fn get_direct_short_buffer_address<'local>(env: &mut Env<'local>, buf: &JByteBuffer<'local>) -> Result<&'local mut [i16]> {
    let ptr = env.get_direct_buffer_address(buf)?;
    let capacity = env.get_direct_buffer_capacity(buf)?;
    unsafe { Ok(slice::from_raw_parts_mut(ptr as *mut i16, (capacity / 2) as usize)) }
}

#[allow(dead_code)]
pub fn get_direct_buffer_address<'local>(env: &mut Env<'local>, buf: &JByteBuffer<'local>) -> Result<&'local mut [u8]> {
    let ptr = env.get_direct_buffer_address(buf)?;
    let capacity = env.get_direct_buffer_capacity(buf)?;
    unsafe { Ok(slice::from_raw_parts_mut(ptr, capacity as usize)) }
}
