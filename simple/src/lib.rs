use std::collections::HashMap;

// Test simple functions

pub fn hello() -> String {
    String::from("Hello from Rust!")
}

pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

// Test more complicated objects

struct Data {
    id: String,
    val: String,
}

impl Data {
    pub fn new(id: String, val: String) -> Self {
        Data { id, val }
    }
}

struct Store {
    map: HashMap<String, Data>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            map: HashMap::new(),
        }
    }

    pub fn get_data(&self, id: &String) -> Option<&Data> {
        self.map.get(id)
    }

    pub fn set_data(&mut self, id: String, val: Data) -> Option<Data> {
        self.map.insert(id, val)
    }
}

// TODO Test asynchronous code

// Expose JNI for android
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use self::jni::objects::JClass; //, JObject};
    use self::jni::sys::{jint, jstring}; //, jobject};
    use self::jni::JNIEnv;
    use super::*;

    use std::ffi::CString;

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_minimalrustimport_MainActivity_hello(
        env: JNIEnv,
        _: JClass,
    ) -> jstring {
        let c_string = CString::new(hello().as_str()).unwrap();
        let java_string = env
            .new_string(c_string.to_str().unwrap())
            .expect("couldn't create Java string");

        java_string.into_raw()
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_minimalrustimport_MainActivity_add(
        _env: JNIEnv,
        _: JClass,
        java_left: jint,
        java_right: jint,
    ) -> jint {
        add(java_left, java_right)
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_minimalrustimport_MainActivity_newStore(
        env: JNIEnv,
        _: JClass,
    ) -> jobject {
        let rust_store = Store::new();
        let java_class: jclass = env.find_class("Store");
        //let output = env.new_object(Store,

        rust_store.into_raw()
    }

    //#[no_mangle]
    //pub unsafe extern "C" fn
    // Java_com_example_minimalrustimport_MainActivity_getData(env: JNIEnv, _: JClass,
    // java_id: JString) -> jobject {    let id =
    // env.get_string(java_id).expect("invalid id string").as_ptr();    let id_ptr =
    // CString::from_raw(id);    // TODO access Store
    //}
}
