use clap::{Parser, Subcommand};
use clap_stdin::MaybeStdin;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    version,
    about = r#"
::::    ::: :::    ::: ::::::::::: :::::::::: :::    :::
:+:+:   :+: :+:    :+:     :+:     :+:        :+:   :+:             ♪
:+:+:+  +:+ +:+    +:+     +:+     +:+        +:+  +:+      ^_^  ♪
+#+ +:+ +#+ +#+    +:+     +#+     +#++:++#   +#++:++      ('.')  ♪
+#+  +#+#+# +#+    +#+     +#+     +#+        +#+  +#+     M>^<M
#+#   #+#+# #+#    #+#     #+#     #+#        #+#   #+#   ~(m.m)
###    ####  ########      ###     ########## ###    ### security.com

Nutek Encode

Easily encode and decode text, files and buffers into various types of output.

https://nuteksecurity.com"#,
    long_about = r#"Nutek Encode

Easily encode and decode text, files and buffers into various types of output.

Why Nutek Encode?

I was inspired by a Perl script - [hURL](https://github.com/fnord0/hURL). I used it a lot in the past, but it was not maintained anymore. I wanted to create a tool that would be easy to use and maintain. I also wanted to learn Rust, so I decided to create this tool.

© 2025 Nutek Security and contributors.

Nutek Encode is licensed under the MIT license. Please see the [LICENSE](LICENSE.md) file for more information.
"#,
    author = "Neosb"
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Base64 encode or decode
    Base64 {
        /// File to encode or decode
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Decode the input
        #[arg(short, long, default_value_t = false)]
        decode: bool,
        /// Output file to write the result
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
        /// String to encode or decode
        #[arg(short, long, value_name = "STRING")]
        string: Option<String>,
        /// Standard input
        #[arg(default_value = "")]
        stdin: MaybeStdin<String>,
    },
    /// URL-safe Base64 encode or decode
    UrlSafeBase64 {
        /// File to encode or decode
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Decode the input
        #[arg(short, long, default_value_t = false)]
        decode: bool,
        /// Output file to write the result
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
        /// String to encode or decode
        #[arg(short, long, value_name = "STRING")]
        string: Option<String>,
        /// Standard input
        #[arg(default_value = "")]
        stdin: MaybeStdin<String>,
    },
    /// URL encode or decode
    Url {
        /// File to encode or decode
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Decode the input
        #[arg(short, long, default_value_t = false)]
        decode: bool,
        /// Output file to write the result
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
        /// String to encode or decode
        #[arg(short, long, value_name = "STRING")]
        string: Option<String>,
        /// Standard input
        #[arg(default_value = "")]
        stdin: MaybeStdin<String>,
    },
    /// Hex encode or decode
    Hex {
        /// File to encode or decode
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Decode the input
        #[arg(short, long, default_value_t = false)]
        decode: bool,
        /// Output file to write the result
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
        /// String to encode or decode
        #[arg(short, long, value_name = "STRING")]
        string: Option<String>,
        /// Standard input
        #[arg(default_value = "")]
        stdin: MaybeStdin<String>,
    },
    /// Rot13 encode or decode
    /// Rot13 is symmetric, so encoding again will decode it.
    /// This is a simple letter substitution cipher that replaces a letter with the 13th letter after it in the alphabet.
    Rot13 {
        /// File to encode or decode
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Output file to write the result
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
        /// String to encode or decode
        #[arg(short, long, value_name = "STRING")]
        string: Option<String>,
        /// Standard input
        #[arg(default_value = "")]
        stdin: MaybeStdin<String>,
    },
    /// Binary encode or decode
    Binary {
        /// File to encode or decode
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Decode the input
        #[arg(short, long, default_value_t = false)]
        decode: bool,
        /// Output file to write the result
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
        /// String to encode or decode
        #[arg(short, long, value_name = "STRING")]
        string: Option<String>,
        /// Standard input
        #[arg(default_value = "")]
        stdin: MaybeStdin<String>,
    },
    /// Octal encode or decode
    Octal {
        /// File to encode or decode
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Decode the input
        #[arg(short, long, default_value_t = false)]
        decode: bool,
        /// Output file to write the result
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
        /// String to encode or decode
        #[arg(short, long, value_name = "STRING")]
        string: Option<String>,
        /// Standard input
        #[arg(default_value = "")]
        stdin: MaybeStdin<String>,
    },
    /// Decimal encode or decode
    Decimal {
        /// File to encode or decode
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Decode the input
        #[arg(short, long, default_value_t = false)]
        decode: bool,
        /// Output file to write the result
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
        /// String to encode or decode
        #[arg(short, long, value_name = "STRING")]
        string: Option<String>,
        /// Standard input
        #[arg(default_value = "")]
        stdin: MaybeStdin<String>,
    },
    /// HTML entities encode or decode
    HtmlEntities {
        /// File to encode or decode
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Decode the input
        #[arg(short, long, default_value_t = false)]
        decode: bool,
        /// Output file to write the result
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
        /// String to encode or decode
        #[arg(short, long, value_name = "STRING")]
        string: Option<String>,
        /// Entity-encodes a string using an extensive
        /// set of entities, giving a string suitable
        /// for use in HTML attribute values. All entities
        /// from encode_minimal are used, and further,
        /// all non-alphanumeric ASCII characters are
        /// hex-encoded (&#x__;)
        #[arg(short, long)]
        attribute: bool,
        /// Standard input
        #[arg(default_value = "")]
        stdin: MaybeStdin<String>,
    },
    /// SHA-1 hash
    Sha1 {
        /// File to encode or decode
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Output file to write the result
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
        /// String to hash
        #[arg(short, long, value_name = "STRING")]
        string: Option<String>,
        /// Standard input
        #[arg(default_value = "")]
        stdin: MaybeStdin<String>,
    },
    /// SHA-256 hash
    Sha256 {
        /// File to encode or decode
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Output file to write the result
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
        /// String to hash
        #[arg(short, long, value_name = "STRING")]
        string: Option<String>,
        /// Standard input
        #[arg(default_value = "")]
        stdin: MaybeStdin<String>,
    },
    /// SHA-512 hash
    Sha512 {
        /// File to encode or decode
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Output file to write the result
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
        /// String to hash
        #[arg(short, long, value_name = "STRING")]
        string: Option<String>,
        /// Standard input
        #[arg(default_value = "")]
        stdin: MaybeStdin<String>,
    },
    /// SHA-384 hash
    Sha384 {
        /// File to encode or decode
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Output file to write the result
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
        /// String to hash
        #[arg(short, long, value_name = "STRING")]
        string: Option<String>,
        /// Standard input
        #[arg(default_value = "")]
        stdin: MaybeStdin<String>,
    },
    /// SHA-224 hash
    Sha224 {
        /// File to encode or decode
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Output file to write the result
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
        /// String to hash
        #[arg(short, long, value_name = "STRING")]
        string: Option<String>,
        /// Standard input
        #[arg(default_value = "")]
        stdin: MaybeStdin<String>,
    },
    /// SHA-512/256 hash
    Sha512_256 {
        /// File to encode or decode
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Output file to write the result
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
        /// String to hash
        #[arg(short, long, value_name = "STRING")]
        string: Option<String>,
        /// Standard input
        #[arg(default_value = "")]
        stdin: MaybeStdin<String>,
    },
    /// SHA-512/224 hash
    Sha512_224 {
        /// File to encode or decode
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Output file to write the result
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
        /// String to hash
        #[arg(short, long, value_name = "STRING")]
        string: Option<String>,
        /// Standard input
        #[arg(default_value = "")]
        stdin: MaybeStdin<String>,
    },
    /// MD5 hash
    Md5 {
        /// File to encode or decode
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Output file to write the result
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
        /// String to hash
        #[arg(short, long, value_name = "STRING")]
        string: Option<String>,
        /// Standard input
        #[arg(default_value = "")]
        stdin: MaybeStdin<String>,
    },
    /// Convert integer to hex
    IntegerToHex {
        /// File containing the integer to convert
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Output file to write the result
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
        /// Integer to convert
        #[arg(short, long, value_name = "STRING")]
        string: Option<String>,
        /// Standard input
        #[arg(default_value = "")]
        stdin: MaybeStdin<String>,
    },
    /// Convert integer to octal
    IntegerToOctal {
        /// File containing the integer to convert
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Output file to write the result
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
        /// Integer to convert
        #[arg(short, long, value_name = "STRING")]
        string: Option<String>,
        /// Standard input
        #[arg(default_value = "")]
        stdin: MaybeStdin<String>,
    },
    /// Convert integer to binary
    IntegerToBinary {
        /// File containing the integer to convert
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Output file to write the result
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
        /// Integer to convert
        #[arg(short, long, value_name = "STRING")]
        string: Option<String>,
        /// Standard input
        #[arg(default_value = "")]
        stdin: MaybeStdin<String>,
    },
    /// Convert hex to integer
    HexToInteger {
        /// File containing the hex to convert
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Output file to write the result
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
        /// Hex to convert
        #[arg(short, long, value_name = "STRING")]
        string: Option<String>,
        /// Standard input
        #[arg(default_value = "")]
        stdin: MaybeStdin<String>,
    },
    /// Convert hex to binary
    HexToBinary {
        /// File containing the hex to convert
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Output file to write the result
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
        /// Hex to convert
        #[arg(short, long, value_name = "STRING")]
        string: Option<String>,
        /// Standard input
        #[arg(default_value = "")]
        stdin: MaybeStdin<String>,
    },
    /// Convert hex to octal
    HexToOctal {
        /// File containing the hex to convert
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Output file to write the result
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
        /// Hex to convert
        #[arg(short, long, value_name = "STRING")]
        string: Option<String>,
        /// Standard input
        #[arg(default_value = "")]
        stdin: MaybeStdin<String>,
    },
    /// Convert octal to integer
    OctalToInteger {
        /// File containing the octal to convert
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Output file to write the result
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
        /// Octal to convert
        #[arg(short, long, value_name = "STRING")]
        string: Option<String>,
        /// Standard input
        #[arg(default_value = "")]
        stdin: MaybeStdin<String>,
    },
    /// Convert octal to hex
    OctalToHex {
        /// File containing the octal to convert
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Output file to write the result
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
        /// Octal to convert
        #[arg(short, long, value_name = "STRING")]
        string: Option<String>,
        /// Standard input
        #[arg(default_value = "")]
        stdin: MaybeStdin<String>,
    },
    /// Convert octal to binary
    OctalToBinary {
        /// File containing the octal to convert
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Output file to write the result
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
        /// Octal to convert
        #[arg(short, long, value_name = "STRING")]
        string: Option<String>,
        /// Standard input
        #[arg(default_value = "")]
        stdin: MaybeStdin<String>,
    },
    /// Convert binary to hex
    BinaryToHex {
        /// File containing the binary to convert
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Output file to write the result
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
        /// Binary to convert
        #[arg(short, long, value_name = "STRING")]
        string: Option<String>,
        /// Standard input
        #[arg(default_value = "")]
        stdin: MaybeStdin<String>,
    },
    /// Convert binary to octal
    BinaryToOctal {
        /// File containing the binary to convert
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Output file to write the result
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
        /// Binary to convert
        #[arg(short, long, value_name = "STRING")]
        string: Option<String>,
        /// Standard input
        #[arg(default_value = "")]
        stdin: MaybeStdin<String>,
    },
    /// Convert binary to integer
    BinaryToInteger {
        /// File containing the binary to convert
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Output file to write the result
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
        /// Binary to convert
        #[arg(short, long, value_name = "STRING")]
        string: Option<String>,
        /// Standard input
        #[arg(default_value = "")]
        stdin: MaybeStdin<String>,
    },
}
