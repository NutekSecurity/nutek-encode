use clap::Parser;
use clap_stdin::MaybeStdin;
mod cli;
use cli::{Args, Commands};
use nutek_encode_lib::encoder::*;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Base64 {
            file,
            decode,
            output,
            string,
            stdin,
        } => {
            handle_base64(file, decode, output, string, stdin);
        }
        Commands::UrlSafeBase64 {
            file,
            decode,
            output,
            string,
            stdin,
        } => {
            handle_urlsafe_base64(file, decode, output, string, stdin);
        }
        Commands::Url {
            file,
            decode,
            output,
            string,
            stdin,
        } => {
            handle_url(file, decode, output, string, stdin);
        }
        Commands::Hex {
            file,
            decode,
            output,
            string,
            stdin,
        } => {
            handle_hex(file, decode, output, string, stdin);
        }
        Commands::Rot13 {
            file,
            output,
            string,
            stdin,
        } => {
            handle_rot13(file, output, string, stdin);
        }
        Commands::Binary {
            file,
            decode,
            output,
            string,
            stdin,
        } => {
            handle_binary(file, decode, output, string, stdin);
        }
        Commands::Octal {
            file,
            decode,
            output,
            string,
            stdin,
        } => {
            handle_octal(file, decode, output, string, stdin);
        }
        Commands::Decimal {
            file,
            decode,
            output,
            string,
            stdin,
        } => {
            handle_decimal(file, decode, output, string, stdin);
        }
        Commands::HtmlEntities {
            file,
            decode,
            output,
            string,
            attribute,
            stdin,
        } => {
            handle_html_entities(file, decode, output, string, attribute, stdin);
        }
        Commands::Sha1 {
            string,
            stdin,
            output,
            file,
        } => {
            handle_simple_encoding(file, output, string, stdin, encode_sha1);
        }
        Commands::Sha256 {
            string,
            stdin,
            output,
            file,
        } => {
            handle_simple_encoding(file, output, string, stdin, encode_sha256);
        }
        Commands::Sha512 {
            string,
            stdin,
            output,
            file,
        } => {
            handle_simple_encoding(file, output, string, stdin, encode_sha512);
        }
        Commands::Sha384 {
            string,
            stdin,
            output,
            file,
        } => {
            handle_simple_encoding(file, output, string, stdin, encode_sha384);
        }
        Commands::Sha224 {
            string,
            stdin,
            output,
            file,
        } => {
            handle_simple_encoding(file, output, string, stdin, encode_sha224);
        }
        Commands::Sha512_256 {
            string,
            stdin,
            output,
            file,
        } => {
            handle_simple_encoding(file, output, string, stdin, encode_sha512_256);
        }
        Commands::Sha512_224 {
            string,
            stdin,
            output,
            file,
        } => {
            handle_simple_encoding(file, output, string, stdin, encode_sha512_224);
        }
        Commands::Md5 {
            string,
            stdin,
            output,
            file,
        } => {
            handle_simple_encoding(file, output, string, stdin, encode_md5);
        }
        Commands::IntegerToHex {
            file,
            output,
            string,
            stdin,
        } => {
            handle_simple_encoding(file, output, string, stdin, encode_integer_to_hex);
        }
        Commands::IntegerToOctal {
            file,
            output,
            string,
            stdin,
        } => {
            handle_simple_encoding(file, output, string, stdin, encode_integer_to_octal);
        }
        Commands::IntegerToBinary {
            file,
            output,
            string,
            stdin,
        } => {
            handle_simple_encoding(file, output, string, stdin, encode_integer_to_binary);
        }
        Commands::HexToInteger {
            file,
            output,
            string,
            stdin,
        } => {
            handle_simple_encoding(file, output, string, stdin, encode_hex_to_integer);
        }
        Commands::HexToBinary {
            file,
            output,
            string,
            stdin,
        } => {
            handle_simple_encoding(file, output, string, stdin, encode_hex_to_binary);
        }
        Commands::HexToOctal {
            file,
            output,
            string,
            stdin,
        } => {
            handle_simple_encoding(file, output, string, stdin, encode_hex_to_octal);
        }
        Commands::OctalToInteger {
            file,
            output,
            string,
            stdin,
        } => {
            handle_simple_encoding(file, output, string, stdin, encode_octal_to_integer);
        }
        Commands::OctalToHex {
            file,
            output,
            string,
            stdin,
        } => {
            handle_simple_encoding(file, output, string, stdin, encode_octal_to_hex);
        }
        Commands::OctalToBinary {
            file,
            output,
            string,
            stdin,
        } => {
            handle_simple_encoding(file, output, string, stdin, encode_octal_to_binary);
        }
        Commands::BinaryToHex {
            file,
            output,
            string,
            stdin,
        } => {
            handle_simple_encoding(file, output, string, stdin, encode_binary_to_hex);
        }
        Commands::BinaryToOctal {
            file,
            output,
            string,
            stdin,
        } => {
            handle_simple_encoding(file, output, string, stdin, encode_binary_to_octal);
        }
        Commands::BinaryToInteger {
            file,
            output,
            string,
            stdin,
        } => {
            handle_simple_encoding(file, output, string, stdin, encode_binary_to_integer);
        }
    }
}

/// Reads input data based on the provided sources: --file, --string, or stdin.
fn get_input_data(
    file: Option<PathBuf>,
    string: Option<String>,
    stdin: MaybeStdin<String>,
) -> String {
    // Prefer the --string argument if present and nonempty.
    if let Some(input_string) = string {
        if !input_string.is_empty() {
            return input_string;
        }
    }
    // Next, if --file is provided, read its content.
    if let Some(file_path) = file {
        return fs::read_to_string(file_path).unwrap_or_else(|e| {
            eprintln!("Failed to read file: {:?}", e);
            std::process::exit(1);
        });
    }
    // Finally, check stdin.
    let stdin_data = stdin.into_inner();
    if stdin_data.trim() == "-" {
        return String::new();
    }
    if !stdin_data.is_empty() {
        return stdin_data;
    }
    String::new()
}

/// Handles the base64 and decoding logic.
fn handle_base64(
    file: Option<PathBuf>,
    decode: bool,
    output: Option<PathBuf>,
    string: Option<String>,
    stdin: MaybeStdin<String>,
) {
    handle_encoding(
        file,
        decode,
        output,
        string,
        stdin,
        encode_base64,
        decode_base64,
    );
}

/// Handles the URL-safe base64 encoding/decoding.
fn handle_urlsafe_base64(
    file: Option<PathBuf>,
    decode: bool,
    output: Option<PathBuf>,
    string: Option<String>,
    stdin: MaybeStdin<String>,
) {
    handle_encoding(
        file,
        decode,
        output,
        string,
        stdin,
        encode_urlsafe_base64,
        decode_urlsafe_base64,
    );
}

/// Handles the URL encoding/decoding.
fn handle_url(
    file: Option<PathBuf>,
    decode: bool,
    output: Option<PathBuf>,
    string: Option<String>,
    stdin: MaybeStdin<String>,
) {
    handle_encoding(file, decode, output, string, stdin, encode_url, decode_url);
}

/// Handles hexadecimal encoding/decoding.
fn handle_hex(
    file: Option<PathBuf>,
    decode: bool,
    output: Option<PathBuf>,
    string: Option<String>,
    stdin: MaybeStdin<String>,
) {
    handle_encoding(file, decode, output, string, stdin, encode_hex, decode_hex);
}

/// Handles ROT13 encoding.
fn handle_rot13(
    file: Option<PathBuf>,
    output: Option<PathBuf>,
    string: Option<String>,
    stdin: MaybeStdin<String>,
) {
    let input_data = get_input_data(file, string, stdin);
    if input_data.is_empty() {
        eprintln!("No input provided. Use --file, --string, or stdin.");
        return;
    }
    let result = encode_rot13(&input_data).unwrap();

    if let Some(output_path) = output {
        let mut file = fs::File::create(output_path).unwrap_or_else(|e| {
            eprintln!("Failed to create output file: {:?}", e);
            std::process::exit(1);
        });
        file.write_all(result.as_bytes()).unwrap_or_else(|e| {
            eprintln!("Failed to write to output file: {:?}", e);
            std::process::exit(1);
        });
    } else {
        println!("{}", result);
    }
}

/// Handles binary encoding/decoding.
fn handle_binary(
    file: Option<PathBuf>,
    decode: bool,
    output: Option<PathBuf>,
    string: Option<String>,
    stdin: MaybeStdin<String>,
) {
    handle_encoding(
        file,
        decode,
        output,
        string,
        stdin,
        encode_binary,
        decode_binary,
    );
}

/// Handles octal encoding/decoding.
fn handle_octal(
    file: Option<PathBuf>,
    decode: bool,
    output: Option<PathBuf>,
    string: Option<String>,
    stdin: MaybeStdin<String>,
) {
    handle_encoding(
        file,
        decode,
        output,
        string,
        stdin,
        encode_octal,
        decode_octal,
    );
}

/// Handles decimal encoding/decoding.
fn handle_decimal(
    file: Option<PathBuf>,
    decode: bool,
    output: Option<PathBuf>,
    string: Option<String>,
    stdin: MaybeStdin<String>,
) {
    handle_encoding(
        file,
        decode,
        output,
        string,
        stdin,
        encode_decimal,
        decode_decimal,
    );
}

/// Handles HTML entities encoding/decoding.
fn handle_html_entities(
    file: Option<PathBuf>,
    decode: bool,
    output: Option<PathBuf>,
    string: Option<String>,
    attribute: bool,
    stdin: MaybeStdin<String>,
) {
    let input_data = get_input_data(file, string, stdin);
    if input_data.is_empty() {
        eprintln!("No input provided. Use --file, --string, or stdin.");
        return;
    }
    let result = if decode {
        decode_html_entities(&input_data).unwrap_or_else(|e| {
            eprintln!("Failed to decode HTML entities: {:?}", e);
            std::process::exit(1);
        })
    } else {
        if attribute {
            encode_html_entities_attribute(&input_data).unwrap()
        } else {
            encode_html_entities(&input_data).unwrap()
        }
    };

    if let Some(output_path) = output {
        let mut file = fs::File::create(output_path).unwrap_or_else(|e| {
            eprintln!("Failed to create output file: {:?}", e);
            std::process::exit(1);
        });
        file.write_all(result.as_bytes()).unwrap_or_else(|e| {
            eprintln!("Failed to write to output file: {:?}", e);
            std::process::exit(1);
        });
    } else {
        println!("{}", result);
    }
}

/// General handler for encoding/decoding commands.
fn handle_encoding<F, G>(
    file: Option<PathBuf>,
    decode: bool,
    output: Option<PathBuf>,
    string: Option<String>,
    stdin: MaybeStdin<String>,
    encode_fn: F,
    decode_fn: G,
) where
    F: Fn(&str) -> Result<String, Box<dyn std::error::Error>>,
    G: Fn(&str) -> Result<String, Box<dyn std::error::Error>>,
{
    let input_data = get_input_data(file, string, stdin);
    if input_data.is_empty() {
        eprintln!("No input provided. Use --file, --string, or stdin.");
        return;
    }
    let result = if decode {
        decode_fn(&input_data).unwrap_or_else(|e| {
            eprintln!("Failed to decode: {:?}", e);
            std::process::exit(1);
        })
    } else {
        encode_fn(&input_data).unwrap_or_else(|e| {
            eprintln!("Failed to encode: {:?}", e);
            std::process::exit(1);
        })
    };

    if let Some(output_path) = output {
        let mut file = fs::File::create(output_path).unwrap_or_else(|e| {
            eprintln!("Failed to create output file: {:?}", e);
            std::process::exit(1);
        });
        file.write_all(result.as_bytes()).unwrap_or_else(|e| {
            eprintln!("Failed to write to output file: {:?}", e);
            std::process::exit(1);
        });
    } else {
        println!("{}", result);
    }
}

/// Handles commands that only require a simple encoding function.
fn handle_simple_encoding<F>(
    file: Option<PathBuf>,
    output: Option<PathBuf>,
    string: Option<String>,
    stdin: MaybeStdin<String>,
    encode_fn: F,
) where
    F: Fn(&str) -> Result<String, Box<dyn std::error::Error>>,
{
    let input_data = get_input_data(file, string, stdin);
    if input_data.is_empty() {
        eprintln!("No input provided. Use --file, --string, or stdin.");
        return;
    }
    let result = encode_fn(&input_data).unwrap_or_else(|e| {
        eprintln!("Failed to encode: {:?}", e);
        std::process::exit(1);
    });

    if let Some(output_path) = output {
        let mut file = fs::File::create(output_path).unwrap_or_else(|e| {
            eprintln!("Failed to create output file: {:?}", e);
            std::process::exit(1);
        });
        file.write_all(result.as_bytes()).unwrap_or_else(|e| {
            eprintln!("Failed to write to output file: {:?}", e);
            std::process::exit(1);
        });
    } else {
        println!("{}", result);
    }
}
