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

/// Expose the JNI interface for android below
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jdouble, jint, jstring};
    use self::jni::JNIEnv;
    use super::*;

    #[no_mangle]
    pub extern "system" fn Java_com_svenschmidt_kitana_viewmodel_DateTimeViewModel_rust_1julian_1day(
        env: JNIEnv,
        _: JClass,
        year: jint,
        month: jint,
        day: jdouble,
    ) -> jdouble {
        let jd = jd::from_date(year as i16, month as u8, day);
        jd as jdouble
    }
}
