use std::{fs, env, path::Path};

const BYTE_TERMINATOR: &str = "👉👈";
const VALUES: [(u8, char); 5] = [
    (200, '🫂'),
    (50, '💖'),
    (10, '✨'),
    (5, '🥺'),
    (1, ',')
];
const NULL_CHARACTER: &str = "❤️";

fn encode_byte(mut byte: u8) -> String {
    let mut output = String::new();
    if byte == 0 {
        output.push_str(NULL_CHARACTER);
    }
    else {
        for (value, character) in VALUES {
            if byte == 0 { break; }
            output += &character.to_string().repeat((byte / value) as usize);
            byte %= value;
        }
    }
    output
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("lookup.rs");

    let mut lookup = Vec::<String>::with_capacity(u8::MAX as usize);
    for i in 0..=u8::MAX {
        lookup.push(encode_byte(i));
    }

    fs::write(
        dest_path,
        format!(
            "
                pub const BYTE_TERMINATOR: &str = \"{BYTE_TERMINATOR}\";
                pub const NULL_CHARACTER: &str = \"{NULL_CHARACTER}\";
                pub static ENCODING_TABLE: [&str; {}] = [{}];
                fn hash_chunk(input: &str) -> Result<u8, ()> {{
                    match input {{
                        {},
                        _ => Err(())
                    }}
                }}
                pub const REGEX_STRING: &str = \"(?:(?:(?:{NULL_CHARACTER})|{})+{})+\";
                fn char_to_value(input: char) -> Result<u8, ()> {{
                    match input {{
                        {},
                        _ => Err(())
                    }}
                }}
            ",
            lookup.len(),
            lookup.iter().map(|x| format!("\"{x}\"")).collect::<Vec<String>>().join(", "),
            lookup.iter().enumerate().map(|(i, x)| format!("\"{x}\" => Ok({i})")).collect::<Vec<String>>().join(",\n"),
            VALUES.iter().map(|(_, character)| format!("(?:{character})")).collect::<Vec<String>>().join("|"),
            BYTE_TERMINATOR,
            VALUES.iter().map(|(value, character)| format!("'{character}' => Ok({value})")).collect::<Vec<String>>().join(",\n")
        )
    ).unwrap();
}