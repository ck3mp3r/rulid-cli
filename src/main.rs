use clap::Parser;
use serde_json;
use serde_yaml;
use ulid::Ulid;

/// Simple program to generate ULIDs
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of ULIDs to generate
    #[arg(short, long, default_value_t = 1)]
    count: u32,

    /// Type of output to print: string, json, yaml
    #[arg(short, long, default_value = "string")]
    output_format: String,

    /// Output in lower case
    #[arg(short, long, default_value_t = false)]
    lower: bool,
}

fn main() {
    let args = Args::parse();

    let mut ulids: Vec<String> = Vec::new();

    for _ in 0..args.count {
        match args.lower {
            true =>  ulids.push(Ulid::new().to_string().to_lowercase()),
            false => ulids.push(Ulid::new().to_string()),
        }
    }

    match args.output_format.as_str() {
        "string" => {
            for ulid in ulids {
                println!("{}", ulid);
            }
        }
        "json" => {
            println!("{}", serde_json::to_string(&ulids).unwrap());
        }
        "yaml" => {
            println!("{}", serde_yaml::to_string(&ulids).unwrap());
        }
        _ => {
            println!("Invalid output format");
        }
    }
}
