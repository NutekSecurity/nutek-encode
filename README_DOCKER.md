# Nutek Encode

Easily encode and decode text, files and buffers into various types of output.

## Why Nutek Encode?

I was inspired by a Perl script - [hURL](https://github.com/fnord0/hURL). I used it a lot in the past, but it was not maintained anymore. I wanted to create a tool that would be easy to use and maintain. I also wanted to learn Rust, so I decided to create this tool.

## Pull the image

You can pull the image from the [docker hub](https://hub.docker.com/r/neosb/nutek-encode) page.

```bash
docker pull neosb/nutek-encode
```

## Usage

```bash
docker run --rm neosb/nutek-encode
```

## Examples

### Encode text

```bash
echo "Hello, World!" | docker run -i --rm -i neosb/nutek-encode base64 -
```
