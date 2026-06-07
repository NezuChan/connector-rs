use jni::JNIEnv;
use jni::objects::JByteBuffer;

fn test(env: JNIEnv, buf: &JByteBuffer) {
    let ptr = env.get_direct_buffer_address(buf);
}
