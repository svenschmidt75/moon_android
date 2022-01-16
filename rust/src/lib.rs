mod earth;
mod ecliptic;
mod jd;
mod moon;
mod nutation;
mod sun;
mod time;
mod util;

/// Expose the JNI interface for android below
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use super::*;

    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jbyte, jclass, jdouble, jint, jobject, jstring};
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
        moon_data: jobject,
    ) {
        let phase_angle = moon::phase::phase_angle_360(jd);
        env.set_field(
            moon_data,
            "phaseAngle",
            "D",
            self::jni::objects::JValue::Double(phase_angle.0),
        )
        .unwrap();

        let fraction_illuminated = moon::phase::fraction_illuminated(jd);
        env.set_field(
            moon_data,
            "illuminatedFraction",
            "D",
            self::jni::objects::JValue::Double(fraction_illuminated),
        )
        .unwrap();

        let phase_desc = moon::phase::phase_description(jd);
        let phase_desc: JString = env.new_string(phase_desc).unwrap();
        env.set_field(
            moon_data,
            "phaseDesc",
            "Ljava/lang/String;",
            self::jni::objects::JValue::Object(phase_desc.into()),
        )
        .unwrap();

        let longitude = moon::position::geocentric_longitude(jd);
        env.set_field(
            moon_data,
            "geocentricLongitude",
            "D",
            self::jni::objects::JValue::Double(longitude.0),
        )
        .unwrap();

        let latitude = moon::position::geocentric_latitude(jd);
        env.set_field(
            moon_data,
            "geocentricLatitude",
            "D",
            self::jni::objects::JValue::Double(latitude.0),
        )
        .unwrap();

        let distance = moon::position::distance_from_earth(jd);
        env.set_field(
            moon_data,
            "distanceFromEarth",
            "D",
            self::jni::objects::JValue::Double(distance),
        )
        .unwrap();

        let siderial_time = time::apparent_siderial_time(jd);
        env.set_field(
            moon_data,
            "siderialTime",
            "D",
            self::jni::objects::JValue::Double(siderial_time.0),
        )
        .unwrap();
    }

    #[no_mangle]
    pub extern "system" fn Java_com_svenschmidt_kitana_core_NativeAccess_00024Companion_rust_1to_1dms(
        env: JNIEnv,
        _: JClass,
        degrees: jdouble,
        width: jbyte,
    ) -> jstring {
        let dms_str = util::Degrees(degrees).to_dms_str(width as u8);
        let string: JString = env.new_string(dms_str).unwrap();
        string.into_inner()
    }
}
