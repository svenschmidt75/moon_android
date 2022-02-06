use clap::{App, Arg};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, LineWriter, Write};
use std::path::PathBuf;

fn main() -> Result<(), std::io::Error> {
    let app = App::new("delta_t_converter")
        .about("Converts UT1 - UTC data file from NASA into delta t to compute TT from UT")
        .arg(Arg::new("file").required(true))
        .get_matches();

    let filemame = app.value_of("file").unwrap();
    let f = File::open(filemame)?;
    let mut reader = BufReader::new(f);

    let mut path = PathBuf::from(filemame);
    path.set_extension("rs");
    let dest_filemame = path.to_string_lossy();
    let dest_f = File::create(dest_filemame.as_ref())?;
    let mut writer = BufWriter::new(dest_f);

    let mut lines_count = 0;

    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        line = line.trim_end().to_string();
        if line.len() < 68 {
            break;
        }

        lines_count += 1;

        let mjd_str = &line[7..15];
        let mjd = mjd_str.parse::<f64>().unwrap();
        let jd = moonlib::jd::mjd_to_jd(mjd);

        if lines_count == 18297 {
            let a = 1;
        }

        let delta_ut_str = &line[58..68].trim();
        let delta_ut = delta_ut_str.parse::<f64>().unwrap();

        line.truncate(0);

        let cumulative_leap_secs = moonlib::time::cumulative_leap_seconds(jd);
        let delta_t = -delta_ut + cumulative_leap_secs + 32.184;

        let (year, month, day) = moonlib::jd::to_calendar_date(jd);
        let month_text = month_text(month);

        let dest_line = format!("DeltaTValue{{jd: {jd:.2}, delta_t: {delta_t:.7}}}, // {day} {month_text} {year}, UT1-UTC={delta_ut:.7}, Cumulative leap seconds={cumulative_leap_secs}");
        write!(writer, "{}\n", dest_line);
    }

    println!("Processed {lines_count} lines...");

    Ok(())
}

fn month_text(m: u8) -> &'static str {
    match m {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "Mai",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "Invalid",
    }
}
