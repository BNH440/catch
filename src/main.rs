use std::io;
use std::io::{BufRead};
use chrono::prelude::{DateTime, Local};
use std::time::{SystemTime};

fn iso8601(st: &std::time::SystemTime) -> String {
    let dt: DateTime<Local> = st.clone().into(); // TODO change to UTC?
    format!("{}", dt.format("%+"))
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut line = String::new();
    let mut eof = false;


    while !eof {
        match handle.read_line(&mut line) {
            Ok(0) => {
                eof = true;
            }
            Ok(_) => {
                let now = SystemTime::now();
                let timestamp = iso8601(&now);
                print!("{}", format!("{timestamp}: {line}"));
                line.clear();
            }
            Err(_error) => { panic!("something went wrong"); }
        }
    }
}
