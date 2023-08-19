use std::{
    fs,
    io::{BufWriter, Write},
    process::exit,
};

use clap::Parser;
use rand::Rng;
use regex::Regex;

/// Program to create large file consisting of random data
#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    /// The size of each file. Suffixes: KB, KiB, MB, MiB, GB, GiB, TB, TiB
    size: String,

    /// The amount of files of the specified size
    #[arg(short, long, default_value_t = 1)]
    amount: usize,
}

fn main() -> Result<(), String> {
    let suffixes: Vec<(String, usize)> = vec![
        ("KB".to_string(), 1000),
        ("KiB".to_string(), 1024),
        ("MB".to_string(), 1000 * 1000),
        ("MiB".to_string(), 1024 * 1024),
        ("GB".to_string(), 1000 * 1000 * 1000),
        ("GiB".to_string(), 1024 * 1024 * 1024),
        ("TB".to_string(), 1000 * 1000 * 1000 * 1000),
        ("TiB".to_string(), 1024 * 1024 * 1024 * 1024),
    ];

    let args = Cli::parse();
    let size = 'size: {
        if let Ok(x) = args.size.parse::<usize>() {
            x
        } else {
            for suffix in &suffixes {
                if let Some(cap) = Regex::new(
                    r"([0-9]+)"
                        .chars()
                        .chain(suffix.0.chars())
                        .collect::<String>()
                        .as_str(),
                )
                .unwrap()
                .captures(&args.size)
                {
                    if let Ok(num) = cap[1].parse::<usize>() {
                        break 'size num * suffix.1;
                    }
                }
            }

            println!("Couldn't parse the file size: {}", args.size);
            exit(1);
        }
    };

    let mut rng = rand::thread_rng();
    for i in 0..args.amount {
        let file = fs::File::create(format!("{:04}", i))
            .map_err(|e| format!("Couldn't create file: {}", e))?;

        let mut writer = BufWriter::new(file);

        let mut bytes_left = size;

        while bytes_left >= 32 {
            writer
                .write_all(&rng.gen::<[u8; 32]>())
                .map_err(|e| format!("Couldn't write to file: {}", e))?;

            bytes_left -= 32;
        }

        writer
            .write_all(
                &(0..bytes_left)
                    .map(|_| rand::random::<u8>())
                    .collect::<Vec<u8>>(),
            )
            .map_err(|e| format!("Couldn't write to file: {}", e))?;
    }

    Ok(())
}
