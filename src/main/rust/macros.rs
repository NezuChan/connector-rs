#[allow(unused_macros)]
macro_rules! jni_unchecked {
    ($jni:expr) => {
        jni::JNIEnv::from_raw($jni).unwrap()
    };
}

#[allow(unused_macros)]
macro_rules! jni_method {
    ($jni:expr, $class:expr, $name:expr, $sig:expr) => {
        jni_unchecked!($jni)
            .get_method_id($class, $name, $sig)
            .unwrap()
    };
}

macro_rules! to_ptr {
    ($obj:expr) => {
        Box::into_raw(Box::new($obj)) as jlong
    };
}

#[allow(unused_macros)]
macro_rules! deref {
    ($handle:expr) => {
        unsafe { Box::from_raw($handle as *mut _) }
    };
}

macro_rules! from_ptr {
    ($handle:expr) => {
        unsafe { &mut *($handle) }
    };
}
