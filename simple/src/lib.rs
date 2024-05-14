//use std::collections::HashMap;

// Test simple functions

pub fn hello() -> String {
    String::from("Hello from Rust!")
}

pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

// Test more complicated objects

// inspired by https://users.rust-lang.org/t/using-jni-to-call-a-rust-function-that-passes-and-returns-a-struct/67488

struct Foo {
    data1: i32,
    data2: i32,
}

impl Foo {
    pub fn new(data1: i32, data2: i32) -> Foo {
      Self { data1, data2 }
    }

    pub fn get_sum(&self) -> i32 {
      self.data1 + self.data2
    }
}

// no idea how to import this/where it comes from, presumably it is defined
// in flapigen(-rs) but I can't find the definition, and importing
// flapigen::foreign_class does not work. 
// see https://dushistov.github.io/flapigen-rs/foreign-class.html
//foreign_class!(class Foo {
//    self_type Foo;
//    constructor Foo::new(data1: i32, data2: i32) -> Foo;
//    fn Foo::get_sum(&self) -> i32;
//});

//struct Data {
//    id: String,
//    val: String,
//}
//
//impl Data {
//    pub fn new(id: String, val: String) -> Self {
//        Data { id, val }
//    }
//}
//
//struct Store {
//    map: HashMap<String, Data>,
//}
//
//impl Store {
//    pub fn new() -> Self {
//        Store {
//            map: HashMap::new(),
//        }
//    }
//
//    pub fn get_data(&self, id: &String) -> Option<&Data> {
//        self.map.get(id)
//    }
//
//    pub fn set_data(&mut self, id: String, val: Data) -> Option<Data> {
//        self.map.insert(id, val)
//    }
//}

// TODO Test asynchronous code

// Expose JNI for android
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use self::jni::objects::{JClass}; //, JString, JObject};
    use self::jni::sys::{jint, jlong, jstring}; //, jlong};
    use self::jni::JNIEnv;
    use super::*;

    use std::ffi::CString;

    //use flapigen::foreign_class;

    #[cfg(target_pointer_width = "64")]
    unsafe fn jlong_to_pointer<T>(val: jlong) -> *mut T {
        core::mem::transmute::<jlong, *mut T>(val) // as u64)
    }

    #[cfg(target_pointer_width = "32")]
    unsafe fn jlong_to_pointer<T>(val: jlong) -> *mut T {
        core::mem::transmute::<u32, *mut T>(val as u32)
    }

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
    pub unsafe extern "C" fn Java_com_example_minimalrustimport_FooWrapper_newFoo(
        _env: JNIEnv,
        _: JClass,
        java_data1: jint,
        java_data2: jint,
    ) -> jlong {
        let foo = Foo::new(java_data1, java_data2);
        let boxed_foo: Box<Foo> = Box::new(foo);
        let foo_ptr: *mut Foo = Box::into_raw(boxed_foo);
        foo_ptr as jlong
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_minimalrustimport_FooWrapper_getSum(
        _env: JNIEnv,
        _: JClass,
        java_foo: jlong,
    ) -> jint {
        let foo_obj: &Foo = unsafe { jlong_to_pointer::<Foo>(java_foo).as_mut().unwrap() };
        let sum = foo_obj.get_sum();
        sum
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_minimalrustimport_MainActivity_getSum(
        _env: JNIEnv,
        _: JClass,
        java_foo: jlong,
    ) -> jint {
        let foo_obj: &Foo = unsafe { jlong_to_pointer::<Foo>(java_foo).as_mut().unwrap() };
        let sum = foo_obj.get_sum();
        sum
    }

    //#[no_mangle]
    //pub unsafe extern "C" fn Java_com_example_minimalrustimport_MainActivity_newData(
    //    env: JNIEnv,
    //    _: JClass,
    //    java_id: JString,
    //    java_value: JString,
    //) -> jobject {
    //    // I don't think this is what we want to be doing; we're calling from
    //    // Kotlin, into Rust, to call back out to a Kotlin constructor
    //    let id = env.get_string(&java_id).expect("invalid id string").to_str().expect("could not create str slice for id");
    //    let value = env.get_string(&java_value).expect("invalid value string").to_str().expect("could not create str slice for value");
    //    let data = Data::new(id.to_string(), value.to_string());

    //    let java_class: jclass = env.find_class("com/example/minimalrustimport/SimpleData");
    //    let simple_data = env.new_object(java_class, 

    //    simple_data.into_raw()
    //}

    //#[no_mangle]
    //pub unsafe extern "C" fn Java_com_example_minimalrustimport_MainActivity_newStore(
    //    env: JNIEnv,
    //    _: JClass,
    //) -> jobject {
    //    let rust_store = Store::new();
    //    let java_class: jclass = env.find_class("Store");
    //    //let output = env.new_object(Store,

    //    rust_store.into_raw()
    //}

    //#[no_mangle]
    //pub unsafe extern "C" fn
    // Java_com_example_minimalrustimport_MainActivity_getData(env: JNIEnv, _: JClass,
    // java_id: JString) -> jobject {    let id =
    // env.get_string(java_id).expect("invalid id string").as_ptr();    let id_ptr =
    // CString::from_raw(id);    // TODO access Store
    //}
}
