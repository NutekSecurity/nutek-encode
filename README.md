# Nutek Encode

Easily encode and decode text, files and buffers into various types of output.

## Why Nutek Encode?

I was inspired by a Perl script - [hURL](https://github.com/fnord0/hURL). I used it a lot in the past, but it was not maintained anymore. I wanted to create a tool that would be easy to use and maintain. I also wanted to learn Rust, so I decided to create this tool.

## Installation

From stable:

```bash
cargo install nutek-encode
```

From source:

```bash
cargo install --git https://github.com/NutekSecurity/nutek-encode
```

or download the binary from the [releases](https://github.com/nuteksecurity/nutek-encode/releases/latest) page.

just maybe go to the [docker hub](https://hub.docker.com/r/neosb/nutek-encode) page and pull the image.

## Usage

![Nutek Encode showcase](examples/demo.gif)

```bash
nutek-encode --help
```

### It is also possible to use the tool as a library

```rust
use nutek_encode_lib::encoder;

fn main() {
    let encoded = encoder::encode_base64("hello").unwrap();
    assert_eq!(encoded, "aGVsbG8=");
}
```

For more inspiration, check out the [documentation](https://docs.rs/nutek-encode).

## Development

If you find a bug or have an idea for a new feature, please open an issue.

```bash
git clone https://github.com/NutekSecurity/nutek-encode.git
cd nutek-encode
cargo build
```

### Tests

```bash
cargo test
```

## Benchmarks

```bash
cargo bench
```

## License

© 2025 Nutek Security and contributors.

Nutek Encode is licensed under the MIT license. Please see the [LICENSE](LICENSE.md) file for more information.
