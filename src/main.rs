use std::io;
use std::io::{BufRead, BufWriter, Write};
use chrono::prelude::{DateTime, Local};
use std::time::{SystemTime};

fn iso8601(st: &std::time::SystemTime) -> String {
    let dt: DateTime<Local> = st.clone().into(); // TODO change to UTC?
    format!("{}", dt.format("%+"))
}

// fn handleLine(line: &std::io::Result<String>) -> String {
//
// }

fn main() {
    let stdin = io::stdin();
    let mut in_handle = stdin.lock();
    let mut out_handle = BufWriter::new(io::stdout());
    let mut in_line = String::new();
    let mut eof = false;


    while !eof {
        match in_handle.read_line(&mut in_line) {
            Ok(0) => {
                eof = true;
            }
            Ok(_) => {
                let now = SystemTime::now();
                let timestamp = iso8601(&now);

                out_handle.write_all(format!("{timestamp}: ").as_bytes()).unwrap();
                out_handle.write_all(in_line.as_bytes()).unwrap();
                out_handle.flush().unwrap();
                in_line.clear();
            }
            Err(_error) => { panic!("error reading stdin"); }
        }
    }
}
