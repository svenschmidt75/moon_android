use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter};
use std::path::PathBuf;
use clap::{App, Arg};

fn main() -> Result<(), io::Error> {
    let app = App::new("delta_t_converter")
        .about("Converts UT1 - UTC data file from NASA into delta t to compute TT from UT")
        .arg(Arg::new("file").required(true))
        .get_matches();

    let filemame = app.value_of("file").unwrap();
    let f = File::open(filemame)?;
    let mut reader = BufReader::new(f);
    let f = File::open(filemame)?;
    let mut reader = BufReader::new(f);

    let mut path = PathBuf::from(filemame);
    path.set_extension("rs");
    let dest_filemame = path.to_string_lossy();
    let dest_f = File::create(dest_filemame.as_ref())?;
    let mut writer = BufWriter::new(dest_f);

    let mut line = String::new();
    while  reader.read_line(&mut line)? >= 68 {
        let mjd_str = &line[8..16];
        let mjd = mjd_str.parse::<f64>().unwrap();
        let jd = moonlib::jd::mjd_to_jd(mjd);

    }

    Ok(())
}
