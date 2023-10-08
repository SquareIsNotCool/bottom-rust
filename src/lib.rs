mod split_keep;

use split_keep::{split_at_whitespace, split_at_encoded_text};
use std::{string::FromUtf8Error, fmt::Display};
use clap::ValueEnum;
use thiserror::Error;
use rayon::prelude::*;

include!(concat!(env!("OUT_DIR"), "/lookup.rs"));

pub fn encode(input: &str) -> String {
    if input.is_empty() {
        String::new()
    }
    else {
        let bytes = input.bytes();
        
        let encoded = bytes.into_iter()
            .map(|byte| ENCODING_TABLE[byte as usize])
            .collect::<Vec<_>>()
            .join(BYTE_TERMINATOR);
    
        format!("{encoded}{BYTE_TERMINATOR}")
    }

}

pub fn encode_ignoring_whitespace(input: &str) -> String {
    split_at_whitespace(input)
        .into_iter()
        .map(|(is_whitespace, content)| {
            if is_whitespace {
                content.to_string()
            }
            else {
                encode(content)
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

#[derive(Error, Debug)]
pub enum BottomDecodeError {
    #[error("Input is not a valid Bottom encoded string")]
    InvalidBottomEncodedString,

    #[error("Input is not a valid UTF-8 encoded string: {0}")]
    InvalidUtf8EncodedString(FromUtf8Error)
}

#[derive(Clone, Copy, Debug, ValueEnum, PartialEq, Eq)]
pub enum Mode {
    Strict,
    Lenient
}
impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Strict => "strict",
            Self::Lenient => "lenient",
        })
    }
}

pub fn decode(input: &str, mode: Mode) -> Result<String, BottomDecodeError> {
    if mode == Mode::Strict {
        decode_section(input, mode)
    }
    else {
        Ok(
            split_at_encoded_text(input)
                .into_par_iter()
                .map(|(is_encoded, content)| {
                    if is_encoded {
                        decode_section(content, mode)
                            .unwrap_or(content.to_string())
                    }
                    else {
                        content.to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

fn decode_section(input: &str, mode: Mode) -> Result<String, BottomDecodeError> {
    if !input.ends_with(BYTE_TERMINATOR) {
        Err(BottomDecodeError::InvalidBottomEncodedString)
    }
    else {
        let bytes = input
            .split(BYTE_TERMINATOR)
            .collect::<Vec<_>>()
            .par_iter()
            .filter(|x| !x.is_empty())
            .map(|x| {
                let mut byte = hash_chunk(x);
                if byte.is_err() && mode != Mode::Strict {
                    byte = decode_unordered_chunk(x);
                }
                byte
            })
            .collect::<Result<Vec<u8>, ()>>();

        if let Ok(bytes) = bytes {
            match String::from_utf8(bytes) {
                Ok(output) => {
                    Ok(output)
                },
                Err(err) => {
                    Err(BottomDecodeError::InvalidUtf8EncodedString(err))
                }
            }
        }
        else {
            Err(BottomDecodeError::InvalidBottomEncodedString)
        }
    }
}

fn decode_unordered_chunk(chunk: &str) -> Result<u8, ()> {
    let mut byte = 0;
    for character in chunk.chars() {
        byte += char_to_value(character)?;
    }
    Ok(byte)
}