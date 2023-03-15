use clap::Parser;
use ulid::Ulid;

/// Simple program to generate ULIDs
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of ULIDs to generate
    #[arg(short, long, default_value_t = 1)]
    count: u32,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        let ulid = Ulid::new();
        println!("{}", ulid.to_string());
    }
}
