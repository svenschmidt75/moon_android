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

    use super::*;

    use self::jni::objects::{JClass, JObject, JString};
    use self::jni::sys::{jdouble, jint, jobject, jstring, jclass};
    use self::jni::JNIEnv;
    use super::*;

    /*
     * Julian Day
     */

    #[no_mangle]
    pub extern "system" fn Java_com_svenschmidt_kitana_core_NativeAccess_00024Companion_rust_1julian_1day(
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

    #[no_mangle]
    pub extern "system" fn Java_com_svenschmidt_kitana_core_NativeAccess_00024Companion_rust_1moon_1data(
        env: JNIEnv,
        _: JClass,
        jd: jdouble,
        moon_data: jobject
    ) {

        println!("Invoked native method Java_com_svenschmidt_kitana_viewmodel_MoonActivityViewModel_rust_1moon_1data");

        let phase_angle = moon::phase::phase_angle_360(jd);
        env.set_field(moon_data, "phaseAngle", "D", self::jni::objects::JValue::Double(phase_angle.0));

        let fraction_illuminated = moon::phase::fraction_illuminated(jd);
        env.set_field(moon_data, "illuminatedFraction", "D", self::jni::objects::JValue::Double(fraction_illuminated));

        let phase_desc = moon::phase::phase_description(jd);
        let phase_desc: JString = env.new_string(phase_desc).unwrap();
        env.set_field(moon_data, "phaseDesc", "Ljava/lang/String;", self::jni::objects::JValue::Object(phase_desc.into()));
    }


}
