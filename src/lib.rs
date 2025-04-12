use std::io;
use std::io::{BufRead, BufWriter, Write};
use chrono::prelude::{DateTime, Local, Utc, SecondsFormat};
use colored::Colorize;
use std::fs::OpenOptions;

#[derive(
    clap::ValueEnum, Clone, Default, Debug
)]
pub enum TZ {
    #[default]
    Local,
    Utc,
}


fn timestamp(tz: &TZ, dt: &DateTime<Utc>) -> String {
    match tz {
        TZ::Local => {
            let local_dt: DateTime<Local> = dt.with_timezone(&Local);
            local_dt.to_rfc3339_opts(SecondsFormat::Secs, true)
        }
        TZ::Utc => {
            dt.to_rfc3339_opts(SecondsFormat::Secs, true)
        }
    }
}

fn generate_log_filename(prefix: &str, tz: &TZ, dt: &DateTime<Utc>) -> String {
    match tz {
        TZ::Local => {
            let local_dt: DateTime<Local> = dt.with_timezone(&Local);
            format!("{}_{}.log", prefix, local_dt.format("%Y-%m-%d_%H-%M-%S"))

        }
        TZ::Utc => {
            format!("{}_{}.log", prefix, dt.format("%Y-%m-%d_%H-%M-%S"))
        }
    }
}

pub fn create_logger(prefix: &str, tz: TZ) {
    let stdin = io::stdin();
    let mut in_handle = stdin.lock();
    let mut out_handle = BufWriter::new(io::stdout());
    let mut in_line = String::new();
    let mut eof = false;



    let now = Utc::now();
    let log_file_name = generate_log_filename(prefix, &tz, &now);

    let log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(format!("/tmp/{}", log_file_name))
        .expect("An error occurred while opening the log file");
    let mut log_handle = BufWriter::new(log_file);



    while !eof {
        match in_handle.read_line(&mut in_line) {
            Ok(0) => {
                eof = true;
            }
            Ok(_) => {
                let now = Utc::now();
                let timestamp = timestamp(&tz, &now);

                // write to console
                out_handle.write_all(format!("{}: ", timestamp.bold()).as_bytes()).unwrap();
                out_handle.write_all(in_line.as_bytes()).unwrap();
                
                // write to log file
                log_handle.write_all(format!("{}: ", timestamp).as_bytes()).unwrap();
                log_handle.write_all(in_line.as_bytes()).unwrap();

                // flush buffers
                out_handle.flush().unwrap();
                log_handle.flush().unwrap();

                in_line.clear();
            }
            Err(_error) => { panic!("error reading stdin"); }
        }
    }
}
