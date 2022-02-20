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

#[cfg(target_os = "android")]
#[macro_use] extern crate log;

/// Expose the JNI interface for android below
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;
    extern crate android_logger;

    use crate::date::{date::Date, jd::JD};
    use crate::util::degrees::Degrees;
    use crate::*;

    use self::jni::objects::{JClass, JString, JObject, JValue};
    use self::jni::sys::{jbyte, jdouble, jint, jobject, jstring};
    use self::jni::JNIEnv;

    use log::Level;
    use android_logger::Config;

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
        let jd = JD::from_date(Date::new(year as i16, month as u8, day));
        jd.jd as jdouble
    }

    #[no_mangle]
    pub extern "system" fn Java_com_svenschmidt_kitana_core_NativeAccess_00024Companion_rust_1local_1siderial_1time(
        env: JNIEnv,
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

        // SS: rise/transit/set times
        let tt = time::utc_2_tt(jd);

        let target_altitude = moon::rise_set_transit::target_altitude(
            tt,
            Degrees::new(0.0),
            longitude_observer,
            latitude_observer,
            pressure,
            temperature,
        );

        let rise_date_time = env.get_field(moon_output_data, "riseTime", "Lcom/svenschmidt/kitana/core/NativeAccess$DateTime;").unwrap().l().unwrap();

        match moon::rise_set_transit::rise(tt, target_altitude, longitude_observer, latitude_observer) {
            moon::rise_set_transit::OutputKind::Time(jd) => {
                let date = jd.to_calendar_date();
                let (h, m, s) = Date::from_fract_day(date.day);

                env.set_field(
                    rise_date_time,
                    "isValid",
                    "Z",
                    self::jni::objects::JValue::Bool(1),
                );

                env.set_field(
                    rise_date_time,
                    "year",
                    "S",
                    self::jni::objects::JValue::Short(date.year),
                );

                env.set_field(
                    rise_date_time,
                    "month",
                    "S",
                    self::jni::objects::JValue::Short(date.month as i16),
                );

                env.set_field(
                    rise_date_time,
                    "day",
                    "S",
                    self::jni::objects::JValue::Short(date.day.trunc() as i16),
                );

                env.set_field(
                    rise_date_time,
                    "hours",
                    "S",
                    self::jni::objects::JValue::Short(h as i16),
                );

                env.set_field(
                    rise_date_time,
                    "minutes",
                    "S",
                    self::jni::objects::JValue::Short(m as i16),
                );

                env.set_field(
                    rise_date_time,
                    "seconds",
                    "D",
                    self::jni::objects::JValue::Double(s),
                );

            }

            moon::rise_set_transit::OutputKind::NeverRises => {
                env.set_field(
                    rise_date_time,
                    "isValid",
                    "Z",
                    self::jni::objects::JValue::Bool(0),
                );
            }

            moon::rise_set_transit::OutputKind::NeverSets => {
                env.set_field(
                    rise_date_time,
                    "isValid",
                    "Z",
                    self::jni::objects::JValue::Bool(0),
                );
            }
        }

        let set_date_time = env.get_field(moon_output_data, "setTime", "Lcom/svenschmidt/kitana/core/NativeAccess$DateTime;").unwrap().l().unwrap();

        match moon::rise_set_transit::set(tt, target_altitude, longitude_observer, latitude_observer) {
            moon::rise_set_transit::OutputKind::Time(jd) => {
                let date = jd.to_calendar_date();
                let (h, m, s) = Date::from_fract_day(date.day);

                env.set_field(
                    set_date_time,
                    "isValid",
                    "Z",
                    self::jni::objects::JValue::Bool(1),
                );

                env.set_field(
                    set_date_time,
                    "year",
                    "S",
                    self::jni::objects::JValue::Short(date.year),
                );

                env.set_field(
                    set_date_time,
                    "month",
                    "S",
                    self::jni::objects::JValue::Short(date.month as i16),
                );

                env.set_field(
                    set_date_time,
                    "day",
                    "S",
                    self::jni::objects::JValue::Short(date.day.trunc() as i16),
                );

                env.set_field(
                    set_date_time,
                    "hours",
                    "S",
                    self::jni::objects::JValue::Short(h as i16),
                );

                env.set_field(
                    set_date_time,
                    "minutes",
                    "S",
                    self::jni::objects::JValue::Short(m as i16),
                );

                env.set_field(
                    set_date_time,
                    "seconds",
                    "D",
                    self::jni::objects::JValue::Double(s),
                );

            }

            moon::rise_set_transit::OutputKind::NeverRises => {
                env.set_field(
                    set_date_time,
                    "isValid",
                    "Z",
                    self::jni::objects::JValue::Bool(0),
                );
            }

            moon::rise_set_transit::OutputKind::NeverSets => {
                env.set_field(
                    set_date_time,
                    "isValid",
                    "Z",
                    self::jni::objects::JValue::Bool(0),
                );
            }
        }

        let transit_date_time = env.get_field(moon_output_data, "transitTime", "Lcom/svenschmidt/kitana/core/NativeAccess$DateTime;").unwrap().l().unwrap();

        match moon::rise_set_transit::transit(tt, target_altitude, longitude_observer, latitude_observer) {
            moon::rise_set_transit::OutputKind::Time(jd) => {
                let date = jd.to_calendar_date();
                let (h, m, s) = Date::from_fract_day(date.day);

                env.set_field(
                    transit_date_time,
                    "isValid",
                    "Z",
                    self::jni::objects::JValue::Bool(1),
                );

                env.set_field(
                    transit_date_time,
                    "year",
                    "S",
                    self::jni::objects::JValue::Short(date.year),
                );

                env.set_field(
                    transit_date_time,
                    "month",
                    "S",
                    self::jni::objects::JValue::Short(date.month as i16),
                );

                env.set_field(
                    transit_date_time,
                    "day",
                    "S",
                    self::jni::objects::JValue::Short(date.day.trunc() as i16),
                );

                env.set_field(
                    transit_date_time,
                    "hours",
                    "S",
                    self::jni::objects::JValue::Short(h as i16),
                );

                env.set_field(
                    transit_date_time,
                    "minutes",
                    "S",
                    self::jni::objects::JValue::Short(m as i16),
                );

                env.set_field(
                    transit_date_time,
                    "seconds",
                    "D",
                    self::jni::objects::JValue::Double(s),
                );

            }

            moon::rise_set_transit::OutputKind::NeverRises => {
                env.set_field(
                    transit_date_time,
                    "isValid",
                    "Z",
                    self::jni::objects::JValue::Bool(0),
                );
            }

            moon::rise_set_transit::OutputKind::NeverSets => {
                env.set_field(
                    transit_date_time,
                    "isValid",
                    "Z",
                    self::jni::objects::JValue::Bool(0),
                );
            }
        }

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
