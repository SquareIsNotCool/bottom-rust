# 🔥bottom-rust🔥

A blazingly fast implementation of the [Bottom Encoding spec](https://github.com/bottom-software-foundation/spec) in the [Rust programming language](https://www.rust-lang.org/).

## Usage

Clone the repo, run `cargo build`, and use the CLI.

```text
A blazingly fast implementation of the Bottom Encoding spec in the Rust programming language

Usage: bottom-rust.exe [OPTIONS] [MESSAGE]

Arguments:
  [MESSAGE]
          The string to encode/decode

Options:
  -m, --method <METHOD>
          Whether to encode or decode the input

          [default: encode]
          [possible values: encode, decode]

      --mode <MODE>
          `strict` is faster but won't decode partially encoded strings or strings with unsorted sections
          `lenient` allows for partially encoded strings and unsorted sections

          Only affects decoding

          [default: strict]

          Possible values:
          - strict:  Faster but won't decode partially encoded strings or strings with unsorted sections. Use this if you care about speed or want to insure input is 100% up to code 
          - lenient: Allows for partially encoded strings and unsorted sections

  -s, --skip-whitespace
          Skip encoding whitespace

      --no-trim
          Trim the input string before encoding/decoding

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Notice

I created this abomination for the hell of it and currently have zero intentions of maintaining it.

If you plan on using this repo for reasons other than "for the hell of it" I highly recommend you use <https://github.com/bottom-software-foundation/bottom-rs> instead.
