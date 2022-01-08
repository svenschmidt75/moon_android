mod earth;
mod ecliptic;
mod jd;
mod moon;
mod nutation;
mod sun;
mod util;

/*
 * Julian Day
 */

// #[no_mangle]
// pub extern "C" fn julian_day() -> *mut c_char {
//     let c_str = unsafe { CStr::from_ptr(to) };
//     let recipient = match c_str.to_str() {
//         Err(_) => "there",
//         Ok(string) => string,
//     };
//
//     CString::new("Hello ".to_owned() + recipient)
//         .unwrap()
//         .into_raw()
// }




/// Expose the JNI interface for android below
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jstring, jint};
    use self::jni::JNIEnv;
    use super::*;

    #[no_mangle]
    pub extern "system" fn Java_com_svenschmidt_kitana_viewmodel_DateTimeViewModel_rust_1julian_1day (
        env: JNIEnv,
        _: JClass,
        year: jint,
        month: jint,
        day: jint,
    ) -> jint {
//    let jd = jd::
        12 as jint
    }
}

