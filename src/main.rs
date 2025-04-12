use clap::Parser;
use catch::{create_logger, TZ};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t, value_enum)]
    timezone: TZ,

    #[arg(short, long, default_value_t = String::from("log"))]
    prefix: String,
}

fn main() {
    let args = Args::parse();
    create_logger(&args.prefix, args.timezone);
}
