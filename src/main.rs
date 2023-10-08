// https://github.com/bottom-software-foundation/spec

use std::{io::{self, BufRead}, fmt::Display, time::{Instant, Duration}};
use bottom_rust::{encode, decode, Mode, encode_ignoring_whitespace};
use clap::{Parser, ValueEnum};
use anyhow::{Context, Result};

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Method {
    Encode,
    Decode
}
impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Encode => "encode",
            Self::Decode => "decode",
        })
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// The string to encode/decode
    message: Option<String>,

    /// Whether to encode or decode the input
    #[arg(short, long, default_value_t = Method::Encode)]
    method: Method,

    /// `strict` is faster but won't decode partially encoded strings or strings with unsorted sections
    /// `lenient` allows for partially encoded strings and unsorted sections
    /// 
    /// Only affects decoding
    #[arg(long, default_value_t = Mode::Strict, verbatim_doc_comment)]
    mode: Mode,

    /// Skip encoding whitespace
    #[arg(short, long, default_value_t = false)]
    skip_whitespace: bool,

    /// Trim the input string before encoding/decoding
    #[arg(long, default_value_t = true)]
    trim: bool
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut message = args.message.unwrap_or_else(|| {
        io::stdin().lock().lines().fold("".to_string(), |acc, line| {
            acc + &line.unwrap() + "\n"
        })
    });

    if args.trim {
        message = message.trim().to_string();
    }

    let mut elapsed: Option<Duration> = None;
    let start = Instant::now();
    if !message.is_empty() {
        match args.method {
            Method::Encode => {
                let encoded = if args.skip_whitespace { encode_ignoring_whitespace(&message) } else { encode(&message) };

                elapsed = Some(start.elapsed());

                print!("{encoded}");
            },
            Method::Decode => {
                let decoded = decode(&message, args.mode)
                    .with_context(|| "Could not decode input")?;

                elapsed = Some(start.elapsed());

                print!("{decoded}");
            }
        }
    }
    if let Some(elapsed) = elapsed {
        eprintln!(
            "Done! {} took {:?}.",
            match args.method {
                Method::Decode => "Decoding",
                Method::Encode => "Encoding"
            },
            elapsed
        );
    }

    Ok(())
}