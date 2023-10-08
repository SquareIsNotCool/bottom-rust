use crate::REGEX_STRING;

macro_rules! regex {
    ($pattern:expr) => {
        {
            static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
            RE.get_or_init(|| regex::Regex::new($pattern).unwrap())
        }
    };
}

pub const WHITESPACE: [char; 11] = [
    '\u{0009}', // (horizontal tab, `'\t'`)
    '\u{000A}', // (line feed, `'\n'`)
    '\u{000B}', // (vertical tab)
    '\u{000C}', // (form feed)
    '\u{000D}', // (carriage return, `'\r'`)
    '\u{0020}', // (space, `' '`)
    '\u{0085}', // (next line)
    '\u{200E}', // (left-to-right mark)
    '\u{200F}', // (right-to-left mark)
    '\u{2028}', // (line separator)
    '\u{2029}' // (paragraph separator)
];

// https://stackoverflow.com/a/56923739
pub fn split_at_whitespace<'a>(input: &'a str) -> Vec<(bool, &'a str)> {
    let mut result: Vec<(bool, &'a str)> = Vec::new();
    let mut last = 0;
    for (index, matched) in input.match_indices(WHITESPACE) {
        if last != index {
            result.push((false, &input[last..index]));
        }
        result.push((true, matched));
        last = index + 1;
    }
    if last < input.len() {
        result.push((false, &input[last..]));
    }
    result
}

// https://stackoverflow.com/a/56923739
pub fn split_at_encoded_text(input: &str) -> Vec<(bool, &str)> {
    let mut result = Vec::new();
    let mut last = 0;
    for (index, matched) in input.match_indices(regex!(REGEX_STRING)) {
        if last != index {
            result.push((false, &input[last..index]));
        }
        result.push((true, matched));
        last = index + matched.len();
    }
    if last < input.len() {
        result.push((false, &input[last..]));
    }
    result
}