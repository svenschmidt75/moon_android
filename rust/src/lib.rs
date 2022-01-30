mod constants;
mod coordinates;
mod earth;
mod ecliptic;
mod jd;
mod moon;
mod nutation;
mod parallax;
mod refraction;
mod sun;
mod time;
mod util;

/// Expose the JNI interface for android below
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use super::*;
    use crate::util::degrees::Degrees;

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

    #[no_mangle]
    pub extern "system" fn Java_com_svenschmidt_kitana_core_NativeAccess_00024Companion_rust_1local_1siderial_1time(
        env: JNIEnv,
        _: JClass,
        jd: jdouble,
        longitude_observer: jdouble,
    ) -> jdouble {
        let sd = time::apparent_siderial_time(jd);
        let lst = time::local_siderial_time(sd, util::degrees::Degrees::new(longitude_observer));
        lst.0 as jdouble
    }

    /*
     * Moon
     */

    #[no_mangle]
    pub extern "system" fn Java_com_svenschmidt_kitana_core_NativeAccess_00024Companion_rust_1moon_1data(
        env: JNIEnv,
        _: JClass,
        moon_input_data: jobject,
        moon_output_data: jobject,
    ) {
        let jd: f64 = env
            .get_field(moon_input_data, "jd", "D")
            .unwrap()
            .d()
            .unwrap();

        let longitude_observer = Degrees::new(
            env.get_field(moon_input_data, "longitudeObserver", "D")
                .unwrap()
                .d()
                .unwrap(),
        );

        let latitude_observer = Degrees::new(
            env.get_field(moon_input_data, "latitudeObserver", "D")
                .unwrap()
                .d()
                .unwrap(),
        );

        let height_above_sea_observer: f64 = env
            .get_field(moon_input_data, "heightAboveSeaObserver", "D")
            .unwrap()
            .d()
            .unwrap();

        let phase_angle = moon::phase::phase_angle_360(jd);
        env.set_field(
            moon_output_data,
            "phaseAngle",
            "D",
            self::jni::objects::JValue::Double(phase_angle.0),
        )
        .unwrap();

        let fraction_illuminated = moon::phase::fraction_illuminated(jd);
        env.set_field(
            moon_output_data,
            "illuminatedFraction",
            "D",
            self::jni::objects::JValue::Double(fraction_illuminated),
        )
        .unwrap();

        let phase_desc = moon::phase::phase_description(jd);
        let phase_desc: JString = env.new_string(phase_desc).unwrap();
        env.set_field(
            moon_output_data,
            "phaseDesc",
            "Ljava/lang/String;",
            self::jni::objects::JValue::Object(phase_desc.into()),
        )
        .unwrap();

        let longitude = moon::position::geocentric_longitude(jd);
        env.set_field(
            moon_output_data,
            "geocentricLongitude",
            "D",
            self::jni::objects::JValue::Double(longitude.0),
        )
        .unwrap();

        let latitude = moon::position::geocentric_latitude(jd);
        env.set_field(
            moon_output_data,
            "geocentricLatitude",
            "D",
            self::jni::objects::JValue::Double(latitude.0),
        )
        .unwrap();

        let distance = moon::position::distance_from_earth(jd);
        env.set_field(
            moon_output_data,
            "distanceFromEarth",
            "D",
            self::jni::objects::JValue::Double(distance),
        )
        .unwrap();

        // SS: Moon's equatorial coordinates
        let eps = ecliptic::true_obliquity(jd);
        let (ra, decl) = coordinates::ecliptic_2_equatorial(longitude, latitude, eps);
        let (ra_topocentric, decl_topocentric) = parallax::equatorial_2_topocentric(
            ra,
            decl,
            longitude_observer,
            latitude_observer,
            height_above_sea_observer,
            distance,
            jd,
        );

        env.set_field(
            moon_output_data,
            "rightAscension",
            "D",
            self::jni::objects::JValue::Double(ra_topocentric.0),
        )
        .unwrap();

        env.set_field(
            moon_output_data,
            "declination",
            "D",
            self::jni::objects::JValue::Double(decl_topocentric.0),
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
        let dms_str = util::degrees::Degrees(degrees).to_dms_str(width as u8);
        let string: JString = env.new_string(dms_str).unwrap();
        string.into_inner()
    }

    #[no_mangle]
    pub extern "system" fn Java_com_svenschmidt_kitana_core_NativeAccess_00024Companion_rust_1to_1hms(
        env: JNIEnv,
        _: JClass,
        degrees: jdouble,
        width: jbyte,
    ) -> jstring {
        let dms_str = util::degrees::Degrees(degrees).to_hms_str(width as u8);
        let string: JString = env.new_string(dms_str).unwrap();
        string.into_inner()
    }
}
