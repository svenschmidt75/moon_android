mod earth;
mod ecliptic;
mod jd;
mod moon;
mod nutation;
mod sun;
mod util;

/// Expose the JNI interface for android below
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use jni::sys::{jfieldID, jvalue};
    use crate::moon::phase::phase_angle_360;
    use self::jni::objects::{JClass, JValue};
    use self::jni::sys::{jdouble, jint, jobject};
    use self::jni::JNIEnv;
    use super::*;

    /*
     * Julian Day
     */

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

    /*
     * Moon
     */


    #[repr(C)]
    pub struct MoonData {
        phase_angle: f64
    }

    #[no_mangle]
    pub extern "system" fn Java_com_svenschmidt_kitana_viewmodel_MoonActivityViewModel_rust_1moon_1data(
        env: JNIEnv,
        _: JClass,
        jd: jdouble,
        moon_data: jobject
    ) {

        println!("Invoked native method Java_com_svenschmidt_kitana_viewmodel_MoonActivityViewModel_rust_1moon_1data");

        let phase_angle = phase_angle_360(jd);
        env.set_field(moon_data, "phaseAngle", "D", self::jni::objects::JValue::Double(phase_angle.0));
    }


}
