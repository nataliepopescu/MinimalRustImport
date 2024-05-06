use std::os::raw::{c_char};
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn hello() -> *mut c_char {
    CString::new("Hello from Rust!").unwrap().into_raw()
}

#[no_mangle]
pub extern fn add(left: usize, right: usize) -> usize {
    left + right
}

// Expose JNI for android
#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jstring};

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_example_minimalrustimport_MainActivity_hello(env: JNIEnv, _: JClass, _: JString) -> jstring {
        let greeting = hello();
        let greeting_ptr = CString::from_raw(greeting);
        let output = env.new_string(greeting_ptr.to_str().unwrap()).expect("Couldn't create Java string!");

        output.into_raw()
    }

    //#[no_mangle]
    //pub unsafe extern "system" fn Java_com_example_minimalrustimport_MainActivity_add(env:JNIEnv, _: JClass, java_pattern: JString) -> c_uint {
    //    let result = add(1, 3);
    //}
}
