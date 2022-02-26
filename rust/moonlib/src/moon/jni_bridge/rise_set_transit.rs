/// Expose the JNI interface for android below
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub(crate) mod android {
    use self::jni::objects::JObject;
    use self::jni::JNIEnv;
    use jni;
    use log::debug;

    use crate::date::date::Date;
    use crate::date::jd::JD;
    use crate::moon;
    use crate::time;
    use crate::util::degrees::Degrees;

    pub(crate) fn rise(
        env: JNIEnv,
        rise_date_time: JObject,
        jd: JD,
        timezone_offset: i8,
        longitude_observer: Degrees,
        latitude_observer: Degrees,
        pressure: f64,
        temperature: f64,
    ) {
        debug!("Calculating Moon's rise time");
        debug!("Time zone offset: {timezone_offset}");

        // SS: calculate dynamical time
        let tt = time::utc_2_tt(jd);

        let target_altitude = moon::rise_set_transit::target_altitude(
            tt,
            Degrees::new(0.0),
            longitude_observer,
            latitude_observer,
            pressure,
            temperature,
        );

        match moon::rise_set_transit::rise(
            tt,
            timezone_offset,
            target_altitude,
            longitude_observer,
            latitude_observer,
        ) {
            moon::rise_set_transit::OutputKind::Time(jd) => {
                let date = jd.to_calendar_date();
                let (h, m, s) = Date::from_fract_day(date.day);

                debug!(
                    "Moon rises on {}/{}/{} at {h}:{m}:{s}",
                    date.year,
                    date.month,
                    date.day.trunc() as u8
                );

                env.set_field(
                    rise_date_time,
                    "isValid",
                    "Z",
                    self::jni::objects::JValue::Bool(1),
                )
                .unwrap();

                env.set_field(
                    rise_date_time,
                    "year",
                    "S",
                    self::jni::objects::JValue::Short(date.year),
                )
                .unwrap();

                env.set_field(
                    rise_date_time,
                    "month",
                    "S",
                    self::jni::objects::JValue::Short(date.month as i16),
                )
                .unwrap();

                env.set_field(
                    rise_date_time,
                    "day",
                    "S",
                    self::jni::objects::JValue::Short(date.day.trunc() as i16),
                )
                .unwrap();

                env.set_field(
                    rise_date_time,
                    "hours",
                    "S",
                    self::jni::objects::JValue::Short(h as i16),
                )
                .unwrap();

                env.set_field(
                    rise_date_time,
                    "minutes",
                    "S",
                    self::jni::objects::JValue::Short(m as i16),
                )
                .unwrap();

                env.set_field(
                    rise_date_time,
                    "seconds",
                    "D",
                    self::jni::objects::JValue::Double(s),
                )
                .unwrap();
            }

            moon::rise_set_transit::OutputKind::NeverRises => {
                debug!("Moon doesn't rise");

                env.set_field(
                    rise_date_time,
                    "isValid",
                    "Z",
                    self::jni::objects::JValue::Bool(0),
                )
                .unwrap();
            }

            moon::rise_set_transit::OutputKind::NeverSets => {
                env.set_field(
                    rise_date_time,
                    "isValid",
                    "Z",
                    self::jni::objects::JValue::Bool(0),
                )
                .unwrap();
            }
        }
    }

    pub(crate) fn set(
        env: JNIEnv,
        set_date_time: JObject,
        jd: JD,
        timezone_offset: i8,
        longitude_observer: Degrees,
        latitude_observer: Degrees,
        pressure: f64,
        temperature: f64,
    ) {
        debug!("Calculating Moon's set time");

        // SS: calculate dynamical time
        let tt = time::utc_2_tt(jd);

        let target_altitude = moon::rise_set_transit::target_altitude(
            tt,
            Degrees::new(0.0),
            longitude_observer,
            latitude_observer,
            pressure,
            temperature,
        );

        match moon::rise_set_transit::set(
            tt,
            timezone_offset,
            target_altitude,
            longitude_observer,
            latitude_observer,
        ) {
            moon::rise_set_transit::OutputKind::Time(jd) => {
                let date = jd.to_calendar_date();
                let (h, m, s) = Date::from_fract_day(date.day);

                debug!(
                    "Moon sets on {}/{}/{} at {h}:{m}:{s}",
                    date.year,
                    date.month,
                    date.day.trunc() as u8
                );

                env.set_field(
                    set_date_time,
                    "isValid",
                    "Z",
                    self::jni::objects::JValue::Bool(1),
                )
                .unwrap();

                env.set_field(
                    set_date_time,
                    "year",
                    "S",
                    self::jni::objects::JValue::Short(date.year),
                )
                .unwrap();

                env.set_field(
                    set_date_time,
                    "month",
                    "S",
                    self::jni::objects::JValue::Short(date.month as i16),
                )
                .unwrap();

                env.set_field(
                    set_date_time,
                    "day",
                    "S",
                    self::jni::objects::JValue::Short(date.day.trunc() as i16),
                )
                .unwrap();

                env.set_field(
                    set_date_time,
                    "hours",
                    "S",
                    self::jni::objects::JValue::Short(h as i16),
                )
                .unwrap();

                env.set_field(
                    set_date_time,
                    "minutes",
                    "S",
                    self::jni::objects::JValue::Short(m as i16),
                )
                .unwrap();

                env.set_field(
                    set_date_time,
                    "seconds",
                    "D",
                    self::jni::objects::JValue::Double(s),
                )
                .unwrap();
            }

            moon::rise_set_transit::OutputKind::NeverRises => {
                env.set_field(
                    set_date_time,
                    "isValid",
                    "Z",
                    self::jni::objects::JValue::Bool(0),
                )
                .unwrap();
            }

            moon::rise_set_transit::OutputKind::NeverSets => {
                debug!("Moon doesn't set");

                env.set_field(
                    set_date_time,
                    "isValid",
                    "Z",
                    self::jni::objects::JValue::Bool(0),
                )
                .unwrap();
            }
        }
    }

    pub(crate) fn transit(
        env: JNIEnv,
        transit_date_time: JObject,
        jd: JD,
        timezone_offset: i8,
        longitude_observer: Degrees,
        latitude_observer: Degrees,
        pressure: f64,
        temperature: f64,
    ) {
        debug!("Calculating Moon's transit time");

        // SS: calculate dynamical time
        let tt = time::utc_2_tt(jd);

        let target_altitude = moon::rise_set_transit::target_altitude(
            tt,
            Degrees::new(0.0),
            longitude_observer,
            latitude_observer,
            pressure,
            temperature,
        );
    }
}
