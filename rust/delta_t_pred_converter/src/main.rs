//! Converter for generating predicted delta_t values used in the caluclation of TT from UTC.
//! Open https://cddis.nasa.gov/archive/products/iers/deltat.preds in a browser,
//! select all and copy into text editor. Save file as deltat.preds in the base
//! folder with the top-level Cargo.toml file.
//! Then execute
//! ```
//! cargo run --package delta_t_pred_converter --bin delta_t_pred_converter -- ../deltat.pred
//! ```
//! Copy the content in the output file deltat.pred.rs to file
//! tabular/src/time/delta_t_table.rs. Delete all "predictions" that are already covered
//! in https://cddis.nasa.gov/archive/products/iers/finals2000A.all, delta_t_converter.
use clap::{App, Arg};
use moonlib::date::jd::JD;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() -> Result<(), std::io::Error> {
    let app = App::new("delta_t_pred_converter")
        .about("Extracts predicted delta t data from NASA to compute TT from UT")
        .arg(Arg::new("file").required(true))
        .get_matches();

    let filemame = app.value_of("file").unwrap();
    let f = File::open(filemame)?;
    let mut reader = BufReader::new(f);

    let dest_filemame = format!("{filemame}.rs");
    let dest_f = File::create(dest_filemame)?;
    let mut writer = BufWriter::new(dest_f);

    let mut lines_count = 0;

    let mut line = String::new();

    // SS: skip header line
    reader.read_line(&mut line)?;
    line.truncate(0);

    while reader.read_line(&mut line)? > 0 {
        line = line.trim_end().to_string();
        if line.is_empty() {
            break;
        }

        lines_count += 1;

        let mjd_str = line[3..12].trim();
        let mjd = mjd_str.parse::<f64>().unwrap();
        let jd = JD::from_mjd(mjd);

        let delta_t_str = line[24..29].trim();
        let delta_t = delta_t_str.parse::<f64>().unwrap();

        line.truncate(0);

        let date = jd.to_calendar_date();
        let month_text = month_text(date.month);

        let dest_line = format!(
            "DeltaTValue{{jd: {0:.2}, delta_t: {delta_t:.7}}}, // {1} {month_text} {2}",
            jd.jd, date.day, date.year
        );
        writeln!(writer, "{}", dest_line)?;
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
