// nutek-encode/benches/my_benchmark/main.rs
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use nutek_encode_lib::encoder::*;

fn benchmark_encode_urlsafe_base64(c: &mut Criterion) {
    let data = "hello world";
    c.bench_function("encode_urlsafe_base64", |b| {
        b.iter(|| encode_urlsafe_base64(black_box(data)))
    });
}

fn benchmark_decode_urlsafe_base64(c: &mut Criterion) {
    let data = "aGVsbG8gd29ybGQ=";
    c.bench_function("decode_urlsafe_base64", |b| {
        b.iter(|| decode_urlsafe_base64(black_box(data)))
    });
}

fn benchmark_encode_base64(c: &mut Criterion) {
    let data = "hello world";
    c.bench_function("encode_base64", |b| {
        b.iter(|| encode_base64(black_box(data)))
    });
}

fn benchmark_decode_base64(c: &mut Criterion) {
    let data = "aGVsbG8gd29ybGQ=";
    c.bench_function("decode_base64", |b| {
        b.iter(|| decode_base64(black_box(data)))
    });
}

fn benchmark_encode_url(c: &mut Criterion) {
    let data = "hello world";
    c.bench_function("encode_url", |b| b.iter(|| encode_url(black_box(data))));
}

fn benchmark_decode_url(c: &mut Criterion) {
    let data = "hello%20world";
    c.bench_function("decode_url", |b| b.iter(|| decode_url(black_box(data))));
}

fn benchmark_encode_binary(c: &mut Criterion) {
    let data = "hello world";
    c.bench_function("encode_binary", |b| {
        b.iter(|| encode_binary(black_box(data)))
    });
}

fn benchmark_decode_binary(c: &mut Criterion) {
    let data =
        "0110100001100101011011000110110001101111001000000111011101101111011100100110110001100100";
    c.bench_function("decode_binary", |b| {
        b.iter(|| decode_binary(black_box(data)))
    });
}

fn benchmark_encode_hex(c: &mut Criterion) {
    let data = "hello world";
    c.bench_function("encode_hex", |b| b.iter(|| encode_hex(black_box(data))));
}

fn benchmark_decode_hex(c: &mut Criterion) {
    let data = "68656c6c6f20776f726c64";
    c.bench_function("decode_hex", |b| b.iter(|| decode_hex(black_box(data))));
}

fn benchmark_encode_rot13(c: &mut Criterion) {
    let data = "hello world";
    c.bench_function("encode_rot13", |b| b.iter(|| encode_rot13(black_box(data))));
}

fn benchmark_encode_octal(c: &mut Criterion) {
    let data = "hello world";
    c.bench_function("encode_octal", |b| b.iter(|| encode_octal(black_box(data))));
}

fn benchmark_decode_octal(c: &mut Criterion) {
    let data = "150145154154157040167157162154144";
    c.bench_function("decode_octal", |b| b.iter(|| decode_octal(black_box(data))));
}

fn benchmark_encode_decimal(c: &mut Criterion) {
    let data = "hello world";
    c.bench_function("encode_decimal", |b| {
        b.iter(|| encode_decimal(black_box(data)))
    });
}

fn benchmark_decode_decimal(c: &mut Criterion) {
    let data = "104 101 108 108 111 32 119 111 114 108 100";
    c.bench_function("decode_decimal", |b| {
        b.iter(|| decode_decimal(black_box(data)))
    });
}

fn benchmark_encode_html_entities(c: &mut Criterion) {
    let data = "<br><p>hello world</p>";
    c.bench_function("encode_html_entities", |b| {
        b.iter(|| encode_html_entities(black_box(data)))
    });
}

fn benchmark_decode_html_entities(c: &mut Criterion) {
    let data = "&lt;br&gt;&lt;p&gt;hello world&lt;/p&gt;";
    c.bench_function("decode_html_entities", |b| {
        b.iter(|| decode_html_entities(black_box(data)))
    });
}

fn benchmark_encode_html_entities_attribute(c: &mut Criterion) {
    let data = "<br><p>hello world</p>";
    c.bench_function("encode_html_entities_attribute", |b| {
        b.iter(|| encode_html_entities_attribute(black_box(data)))
    });
}

fn benchmark_decode_html_entities_attribute(c: &mut Criterion) {
    let data = "&lt;br&gt;&lt;p&gt;hello&#x20;world&lt;&#x2F;p&gt;";
    c.bench_function("decode_html_entities_attribute", |b| {
        b.iter(|| decode_html_entities_attribute(black_box(data)))
    });
}

fn benchmark_encode_integer_to_hex(c: &mut Criterion) {
    let data = "10";
    c.bench_function("encode_integer_to_hex", |b| {
        b.iter(|| encode_integer_to_hex(black_box(data)))
    });
}

fn benchmark_encode_integer_to_octal(c: &mut Criterion) {
    let data = "10";
    c.bench_function("encode_integer_to_octal", |b| {
        b.iter(|| encode_integer_to_octal(black_box(data)))
    });
}

fn benchmark_encode_integer_to_binary(c: &mut Criterion) {
    let data = "10";
    c.bench_function("encode_integer_to_binary", |b| {
        b.iter(|| encode_integer_to_binary(black_box(data)))
    });
}

fn benchmark_encode_hex_to_integer(c: &mut Criterion) {
    let data = "0a";
    c.bench_function("encode_hex_to_integer", |b| {
        b.iter(|| encode_hex_to_integer(black_box(data)))
    });
}

fn benchmark_encode_hex_to_binary(c: &mut Criterion) {
    let data = "0a";
    c.bench_function("encode_hex_to_binary", |b| {
        b.iter(|| encode_hex_to_binary(black_box(data)))
    });
}

fn benchmark_encode_hex_to_octal(c: &mut Criterion) {
    let data = "0a";
    c.bench_function("encode_hex_to_octal", |b| {
        b.iter(|| encode_hex_to_octal(black_box(data)))
    });
}

fn benchmark_encode_octal_to_integer(c: &mut Criterion) {
    let data = "012";
    c.bench_function("encode_octal_to_integer", |b| {
        b.iter(|| encode_octal_to_integer(black_box(data)))
    });
}

fn benchmark_encode_octal_to_hex(c: &mut Criterion) {
    let data = "012";
    c.bench_function("encode_octal_to_hex", |b| {
        b.iter(|| encode_octal_to_hex(black_box(data)))
    });
}

fn benchmark_encode_octal_to_binary(c: &mut Criterion) {
    let data = "012";
    c.bench_function("encode_octal_to_binary", |b| {
        b.iter(|| encode_octal_to_binary(black_box(data)))
    });
}

fn benchmark_encode_binary_to_hex(c: &mut Criterion) {
    let data = "00001010";
    c.bench_function("encode_binary_to_hex", |b| {
        b.iter(|| encode_binary_to_hex(black_box(data)))
    });
}

fn benchmark_encode_binary_to_octal(c: &mut Criterion) {
    let data = "00001010";
    c.bench_function("encode_binary_to_octal", |b| {
        b.iter(|| encode_binary_to_octal(black_box(data)))
    });
}

fn benchmark_encode_binary_to_integer(c: &mut Criterion) {
    let data = "00001010";
    c.bench_function("encode_binary_to_integer", |b| {
        b.iter(|| encode_binary_to_integer(black_box(data)))
    });
}

fn benchmark_encode_sha1(c: &mut Criterion) {
    let data = "hello world";
    c.bench_function("encode_sha1", |b| b.iter(|| encode_sha1(black_box(data))));
}

fn benchmark_encode_sha256(c: &mut Criterion) {
    let data = "hello world";
    c.bench_function("encode_sha256", |b| {
        b.iter(|| encode_sha256(black_box(data)))
    });
}

fn benchmark_encode_sha512(c: &mut Criterion) {
    let data = "hello world";
    c.bench_function("encode_sha512", |b| {
        b.iter(|| encode_sha512(black_box(data)))
    });
}

fn benchmark_encode_sha384(c: &mut Criterion) {
    let data = "hello world";
    c.bench_function("encode_sha384", |b| {
        b.iter(|| encode_sha384(black_box(data)))
    });
}

fn benchmark_encode_sha224(c: &mut Criterion) {
    let data = "hello world";
    c.bench_function("encode_sha224", |b| {
        b.iter(|| encode_sha224(black_box(data)))
    });
}

fn benchmark_encode_sha512_256(c: &mut Criterion) {
    let data = "hello world";
    c.bench_function("encode_sha512_256", |b| {
        b.iter(|| encode_sha512_256(black_box(data)))
    });
}

fn benchmark_encode_sha512_224(c: &mut Criterion) {
    let data = "hello world";
    c.bench_function("encode_sha512_224", |b| {
        b.iter(|| encode_sha512_224(black_box(data)))
    });
}

fn benchmark_encode_md5(c: &mut Criterion) {
    let data = "hello world";
    c.bench_function("encode_md5", |b| b.iter(|| encode_md5(black_box(data))));
}

criterion_group!(
    benches,
    benchmark_encode_urlsafe_base64,
    benchmark_decode_urlsafe_base64,
    benchmark_encode_base64,
    benchmark_decode_base64,
    benchmark_encode_url,
    benchmark_decode_url,
    benchmark_encode_binary,
    benchmark_decode_binary,
    benchmark_encode_hex,
    benchmark_decode_hex,
    benchmark_encode_rot13,
    benchmark_encode_octal,
    benchmark_decode_octal,
    benchmark_encode_decimal,
    benchmark_decode_decimal,
    benchmark_encode_html_entities,
    benchmark_decode_html_entities,
    benchmark_encode_html_entities_attribute,
    benchmark_decode_html_entities_attribute,
    benchmark_encode_integer_to_hex,
    benchmark_encode_integer_to_octal,
    benchmark_encode_integer_to_binary,
    benchmark_encode_hex_to_integer,
    benchmark_encode_hex_to_binary,
    benchmark_encode_hex_to_octal,
    benchmark_encode_octal_to_integer,
    benchmark_encode_octal_to_hex,
    benchmark_encode_octal_to_binary,
    benchmark_encode_binary_to_hex,
    benchmark_encode_binary_to_octal,
    benchmark_encode_binary_to_integer,
    benchmark_encode_sha1,
    benchmark_encode_sha256,
    benchmark_encode_sha512,
    benchmark_encode_sha384,
    benchmark_encode_sha224,
    benchmark_encode_sha512_256,
    benchmark_encode_sha512_224,
    benchmark_encode_md5,
);
criterion_main!(benches);
