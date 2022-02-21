mod constants;
mod coordinates;
pub mod date;
mod earth;
mod ecliptic;
mod moon;
mod nutation;
mod parallax;
mod refraction;
mod sun;
pub mod time;
mod util;

/// Expose the JNI interface for android below
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    use android_logger;
    use android_logger::Config;
    use jni;
    use log::Level;

    use crate::date::{date::Date, jd::JD};
    use crate::util::degrees::Degrees;
    use crate::*;

    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jbyte, jdouble, jint, jobject, jstring};
    use self::jni::JNIEnv;

    /*
     * Julian Day
     */

    #[no_mangle]
    pub extern "system" fn Java_com_svenschmidt_kitana_core_NativeAccess_00024Companion_rust_1julian_1day(
        _env: JNIEnv,
        _: JClass,
        year: jint,
        month: jint,
        day: jdouble,
    ) -> jdouble {
        let jd = JD::from_date(Date::new(year as i16, month as u8, day));
        jd.jd as jdouble
    }

    #[no_mangle]
    pub extern "system" fn Java_com_svenschmidt_kitana_core_NativeAccess_00024Companion_rust_1local_1siderial_1time(
        _env: JNIEnv,
        _: JClass,
        jd_value: jdouble,
        longitude_observer: jdouble,
    ) -> jdouble {
        let jd = JD::new(jd_value);
        let sd = earth::apparent_siderial_time(jd);
        let lst = earth::local_siderial_time(sd, Degrees::new(longitude_observer));
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
        // SS: configure Android logger
        android_logger::init_once(Config::default().with_min_level(Level::Trace));

        let jd: JD = JD::new(
            env.get_field(moon_input_data, "jd", "D")
                .unwrap()
                .d()
                .unwrap(),
        );

        let timezone_offset: i8 = env
            .get_field(moon_input_data, "timezoneOffset", "S")
            .unwrap()
            .s()
            .unwrap() as i8;

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

        let pressure: f64 = env
            .get_field(moon_input_data, "pressure", "D")
            .unwrap()
            .d()
            .unwrap();

        let temperature: f64 = env
            .get_field(moon_input_data, "temperature", "D")
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
        let (ra, decl) = coordinates::ecliptical_2_equatorial(longitude, latitude, eps);
        let (ra_topocentric, decl_topocentric) = coordinates::equatorial_2_topocentric(
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

        // SS: horizontal topocentric coordinates of the moon
        let siderial_time_apparent_greenwich = earth::apparent_siderial_time(jd);
        let siderial_time_local =
            earth::local_siderial_time(siderial_time_apparent_greenwich, longitude_observer);
        let hour_angle = earth::hour_angle(siderial_time_local, ra_topocentric);
        let (azimuth, mut altitude) =
            coordinates::equatorial_2_horizontal(decl_topocentric, hour_angle, latitude_observer);

        // SS: add correction for atmospheric refraction
        let refraction_correction =
            refraction::refraction_for_true_altitude(altitude, pressure, temperature);
        altitude += refraction_correction;

        env.set_field(
            moon_output_data,
            "azimuth",
            "D",
            self::jni::objects::JValue::Double(azimuth.0),
        )
        .unwrap();

        env.set_field(
            moon_output_data,
            "altitude",
            "D",
            self::jni::objects::JValue::Double(altitude.0),
        )
        .unwrap();

        env.set_field(
            moon_output_data,
            "hourAngle",
            "D",
            self::jni::objects::JValue::Double(hour_angle.0),
        )
        .unwrap();

        // SS: Moon's rise time
        let rise_date_time = env
            .get_field(
                moon_output_data,
                "riseTime",
                "Lcom/svenschmidt/kitana/core/NativeAccess$DateTime;",
            )
            .unwrap()
            .l()
            .unwrap();

        use crate::moon::jni_bridge::rise_set_transit::android::rise;
        rise(
            env,
            rise_date_time,
            jd,
            timezone_offset,
            longitude_observer,
            latitude_observer,
            pressure,
            temperature,
        );

        // SS: Moon's set time
        let set_date_time = env
            .get_field(
                moon_output_data,
                "setTime",
                "Lcom/svenschmidt/kitana/core/NativeAccess$DateTime;",
            )
            .unwrap()
            .l()
            .unwrap();

        use crate::moon::jni_bridge::rise_set_transit::android::set;
        set(
            env,
            set_date_time,
            jd,
            timezone_offset,
            longitude_observer,
            latitude_observer,
            pressure,
            temperature,
        );

        // SS: Moon's transit time
        let transit_date_time = env
            .get_field(
                moon_output_data,
                "transitTime",
                "Lcom/svenschmidt/kitana/core/NativeAccess$DateTime;",
            )
            .unwrap()
            .l()
            .unwrap();

        use crate::moon::jni_bridge::rise_set_transit::android::transit;
        transit(
            env,
            set_date_time,
            jd,
            timezone_offset,
            longitude_observer,
            latitude_observer,
            pressure,
            temperature,
        );
    }

    #[no_mangle]
    pub extern "system" fn Java_com_svenschmidt_kitana_core_NativeAccess_00024Companion_rust_1to_1dms(
        env: JNIEnv,
        _: JClass,
        degrees: jdouble,
        width: jbyte,
    ) -> jstring {
        let dms_str = Degrees(degrees).to_dms_str(width as u8);
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
        let dms_str = Degrees(degrees).to_hms_str(width as u8);
        let string: JString = env.new_string(dms_str).unwrap();
        string.into_inner()
    }
}
