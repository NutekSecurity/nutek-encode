pub mod tests {
    use std::{
        fs::{self, File},
        io::Write,
        path::{Path, PathBuf},
        process::Command,
    };
    use tempfile::tempdir;

    fn run_command(args: &[&str]) -> String {
        let process = Command::new("cargo")
            .args(&["run", "--"])
            .args(args)
            .output()
            .expect("failed to execute process");

        String::from_utf8(process.stdout).unwrap()
    }

    fn run_command_stdin(args: &[&str], stdin: &str) -> String {
        let process = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "echo '{}' | cargo run -- {}",
                stdin,
                args.join(" ")
            ))
            .output()
            .expect("failed to execute process");

        String::from_utf8(process.stdout).unwrap()
    }

    struct TestFile {
        dir: tempfile::TempDir,
        file_path: PathBuf,
    }

    fn write_to_file(content: &str) -> TestFile {
        // Create a directory inside of `env::temp_dir()`.
        let dir = tempdir().unwrap();

        let file_path = dir.path().join("test.txt");
        let _ = File::create(&file_path).unwrap();
        fs::write(&file_path, content.as_bytes()).unwrap();
        TestFile { dir, file_path }
    }

    #[test]
    fn test_base64_encode_string() {
        let output = run_command(&["base64", "--string", "hello world"]);
        assert_eq!(output.trim(), "aGVsbG8gd29ybGQ=");
    }

    #[test]
    fn test_base64_decode_string() {
        let output = run_command(&["base64", "--string", "aGVsbG8gd29ybGQ=", "--decode"]);
        assert_eq!(output.trim(), "hello world");
    }

    #[test]
    fn test_base64_encode_stdin() {
        let output = run_command_stdin(&["base64", "-"], "hello world");
        assert_eq!(output.trim(), "aGVsbG8gd29ybGQ=");
    }

    #[test]
    fn test_base64_decode_stdin() {
        let output = run_command_stdin(&["base64", "-", "--decode"], "aGVsbG8gd29ybGQ=");
        assert_eq!(output.trim(), "hello world");
    }

    #[test]
    fn test_base64_encode_file() {
        let file = write_to_file("hello world");
        let output = run_command(&["base64", "--file", file.file_path.to_str().unwrap()]);
        assert_eq!(output.trim(), "aGVsbG8gd29ybGQ=");
    }

    #[test]
    fn test_base64_decode_file() {
        let file = write_to_file("aGVsbG8gd29ybGQ=");
        let output = run_command(&[
            "base64",
            "--file",
            file.file_path.to_str().unwrap(),
            "--decode",
        ]);
        assert_eq!(output.trim(), "hello world");
    }

    #[test]
    fn test_urlsafe_base64_encode_string() {
        let output = run_command(&["url-safe-base64", "--string", "hello world"]);
        assert_eq!(output.trim(), "aGVsbG8gd29ybGQ=");
    }

    #[test]
    fn test_urlsafe_base64_decode_string() {
        let output = run_command(&[
            "url-safe-base64",
            "--string",
            "aGVsbG8gd29ybGQ=",
            "--decode",
        ]);
        assert_eq!(output.trim(), "hello world");
    }

    #[test]
    fn test_urlsafe_base64_encode_stdin() {
        let output = run_command_stdin(&["url-safe-base64", "-"], "hello world");
        assert_eq!(output.trim(), "aGVsbG8gd29ybGQ=");
    }

    #[test]
    fn test_urlsafe_base64_decode_stdin() {
        let output = run_command_stdin(&["url-safe-base64", "-", "--decode"], "aGVsbG8gd29ybGQ=");
        assert_eq!(output.trim(), "hello world");
    }

    #[test]
    fn test_url_encode_string() {
        let output = run_command(&["url", "--string", "hello world"]);
        assert_eq!(output.trim(), "hello%20world");
    }

    #[test]
    fn test_url_decode_string() {
        let output = run_command(&["url", "--string", "hello%20world", "--decode"]);
        assert_eq!(output.trim(), "hello world");
    }

    #[test]
    fn test_url_encode_stdin() {
        let output = run_command_stdin(&["url", "-"], "hello world");
        assert_eq!(output.trim(), "hello%20world");
    }

    #[test]
    fn test_url_decode_stdin() {
        let output = run_command_stdin(&["url", "-", "--decode"], "hello%20world");
        assert_eq!(output.trim(), "hello world");
    }

    #[test]
    fn test_url_encode_file() {
        let file = write_to_file("hello world");
        let output = run_command(&["url", "--file", file.file_path.to_str().unwrap()]);
        assert_eq!(output.trim(), "hello%20world");
    }

    #[test]
    fn test_url_decode_file() {
        let file = write_to_file("hello%20world");
        let output = run_command(&[
            "url",
            "--file",
            file.file_path.to_str().unwrap(),
            "--decode",
        ]);
        assert_eq!(output.trim(), "hello world");
    }

    #[test]
    fn test_hex_encode_string() {
        let output = run_command(&["hex", "--string", "hello world"]);
        assert_eq!(output.trim(), "68656c6c6f20776f726c64");
    }

    #[test]
    fn test_hex_decode_string() {
        let output = run_command(&["hex", "--string", "68656c6c6f20776f726c64", "--decode"]);
        assert_eq!(output.trim(), "hello world");
    }

    #[test]
    fn test_hex_encode_stdin() {
        let output = run_command_stdin(&["hex", "-"], "hello world");
        assert_eq!(output.trim(), "68656c6c6f20776f726c64");
    }

    #[test]
    fn test_hex_decode_stdin() {
        let output = run_command_stdin(&["hex", "-", "--decode"], "68656c6c6f20776f726c64");
        assert_eq!(output.trim(), "hello world");
    }

    #[test]
    fn test_hex_encode_file() {
        let file = write_to_file("hello world");
        let output = run_command(&["hex", "--file", file.file_path.to_str().unwrap()]);
        assert_eq!(output.trim(), "68656c6c6f20776f726c64");
    }

    #[test]
    fn test_hex_decode_file() {
        let file = write_to_file("68656c6c6f20776f726c64");
        let output = run_command(&[
            "hex",
            "--file",
            file.file_path.to_str().unwrap(),
            "--decode",
        ]);
        assert_eq!(output.trim(), "hello world");
    }

    #[test]
    fn test_rot13_encode_string() {
        let output = run_command(&["rot13", "--string", "hello world"]);
        assert_eq!(output.trim(), "uryyb jbeyq");
    }

    #[test]
    fn test_rot13_decode_string() {
        let output = run_command(&["rot13", "--string", "uryyb jbeyq"]);
        assert_eq!(output.trim(), "hello world");
    }

    #[test]
    fn test_rot13_encode_stdin() {
        let output = run_command_stdin(&["rot13", "-"], "hello world");
        assert_eq!(output.trim(), "uryyb jbeyq");
    }

    #[test]
    fn test_rot13_decode_stdin() {
        let output = run_command_stdin(&["rot13", "-"], "uryyb jbeyq");
        assert_eq!(output.trim(), "hello world");
    }

    #[test]
    fn test_rot13_encode_file() {
        let file = write_to_file("hello world");
        let output = run_command(&["rot13", "--file", file.file_path.to_str().unwrap()]);
        assert_eq!(output.trim(), "uryyb jbeyq");
    }

    #[test]
    fn test_rot13_decode_file() {
        let file = write_to_file("uryyb jbeyq");
        let output = run_command(&["rot13", "--file", file.file_path.to_str().unwrap()]);
        assert_eq!(output.trim(), "hello world");
    }

    #[test]
    fn test_binary_encode_string() {
        let output = run_command(&["binary", "--string", "hello world"]);
        assert_eq!(
            output.trim(),
            "0110100001100101011011000110110001101111001000000111011101101111011100100110110001100100"
        );
    }

    #[test]
    fn test_binary_decode_string() {
        let output = run_command(&[
            "binary",
            "--string",
            "0110100001100101011011000110110001101111001000000111011101101111011100100110110001100100",
            "--decode",
        ]);
        assert_eq!(output.trim(), "hello world");
    }

    #[test]
    fn test_binary_encode_stdin() {
        let output = run_command_stdin(&["binary", "-"], "hello world");
        assert_eq!(
            output.trim(),
            "0110100001100101011011000110110001101111001000000111011101101111011100100110110001100100"
        );
    }

    #[test]
    fn test_binary_decode_stdin() {
        let output = run_command_stdin(
            &["binary", "-", "--decode"],
            "0110100001100101011011000110110001101111001000000111011101101111011100100110110001100100",
        );
        assert_eq!(output.trim(), "hello world");
    }

    #[test]
    fn test_binary_encode_file() {
        let file = write_to_file("hello world");
        let output = run_command(&["binary", "--file", file.file_path.to_str().unwrap()]);
        assert_eq!(
            output.trim(),
            "0110100001100101011011000110110001101111001000000111011101101111011100100110110001100100"
        );
    }

    #[test]
    fn test_binary_decode_file() {
        let file = write_to_file(
            "0110100001100101011011000110110001101111001000000111011101101111011100100110110001100100",
        );
        let output = run_command(&[
            "binary",
            "--file",
            file.file_path.to_str().unwrap(),
            "--decode",
        ]);
        assert_eq!(output.trim(), "hello world");
    }

    #[test]
    fn test_octal_encode_string() {
        let output = run_command(&["octal", "--string", "hello world"]);
        assert_eq!(output.trim(), "150145154154157040167157162154144");
    }

    #[test]
    fn test_octal_decode_string() {
        let output = run_command(&[
            "octal",
            "--string",
            "150145154154157040167157162154144",
            "--decode",
        ]);
        assert_eq!(output.trim(), "hello world");
    }

    #[test]
    fn test_octal_encode_stdin() {
        let output = run_command_stdin(&["octal", "-"], "hello world");
        assert_eq!(output.trim(), "150145154154157040167157162154144");
    }

    #[test]
    fn test_octal_decode_stdin() {
        let output = run_command_stdin(
            &["octal", "-", "--decode"],
            "150145154154157040167157162154144",
        );
        assert_eq!(output.trim(), "hello world");
    }

    #[test]
    fn test_octal_encode_file() {
        let file = write_to_file("hello world");
        let output = run_command(&["octal", "--file", file.file_path.to_str().unwrap()]);
        assert_eq!(output.trim(), "150145154154157040167157162154144");
    }

    #[test]
    fn test_octal_decode_file() {
        let file = write_to_file("150145154154157040167157162154144");
        let output = run_command(&[
            "octal",
            "--file",
            file.file_path.to_str().unwrap(),
            "--decode",
        ]);
        assert_eq!(output.trim(), "hello world");
    }

    #[test]
    fn test_decimal_encode_string() {
        let output = run_command(&["decimal", "--string", "hello world"]);
        assert_eq!(output.trim(), "104 101 108 108 111 032 119 111 114 108 100");
    }

    #[test]
    fn test_decimal_decode_string() {
        let output = run_command(&[
            "decimal",
            "--string",
            "104 101 108 108 111 32 119 111 114 108 100",
            "--decode",
        ]);
        assert_eq!(output.trim(), "hello world");
    }

    #[test]
    fn test_decimal_encode_stdin() {
        let output = run_command_stdin(&["decimal", "-"], "hello world");
        assert_eq!(output.trim(), "104 101 108 108 111 032 119 111 114 108 100");
    }

    #[test]
    fn test_decimal_decode_stdin() {
        let output = run_command_stdin(
            &["decimal", "-", "--decode"],
            "104 101 108 108 111 032 119 111 114 108 100",
        );
        assert_eq!(output.trim(), "hello world");
    }

    #[test]
    fn test_decimal_encode_file() {
        let file = write_to_file("hello world");
        let output = run_command(&["decimal", "--file", file.file_path.to_str().unwrap()]);
        assert_eq!(output.trim(), "104 101 108 108 111 032 119 111 114 108 100");
    }

    #[test]
    fn test_decimal_decode_file() {
        let file = write_to_file("104 101 108 108 111 032 119 111 114 108 100");
        let output = run_command(&[
            "decimal",
            "--file",
            file.file_path.to_str().unwrap(),
            "--decode",
        ]);
        assert_eq!(output.trim(), "hello world");
    }

    #[test]
    fn test_html_entities_encode_string() {
        let output = run_command(&["html-entities", "--string", "<br><p>hello world</p>"]);
        assert_eq!(output.trim(), "&lt;br&gt;&lt;p&gt;hello world&lt;/p&gt;");
    }

    #[test]
    fn test_html_entities_decode_string() {
        let output = run_command(&[
            "html-entities",
            "--string",
            "&lt;br&gt;&lt;p&gt;hello world&lt;/p&gt;",
            "--decode",
        ]);
        assert_eq!(output.trim(), "<br><p>hello world</p>");
    }

    #[test]
    fn test_html_entities_encode_stdin() {
        let output = run_command_stdin(&["html-entities", "-"], "<br><p>hello world</p>");
        assert_eq!(output.trim(), "&lt;br&gt;&lt;p&gt;hello world&lt;/p&gt;");
    }

    #[test]
    fn test_html_entities_decode_stdin() {
        let output = run_command_stdin(
            &["html-entities", "-", "--decode"],
            "&lt;br&gt;&lt;p&gt;hello world&lt;/p&gt;",
        );
        assert_eq!(output.trim(), "<br><p>hello world</p>");
    }

    #[test]
    fn test_html_entities_encode_file() {
        let file = write_to_file("<br><p>hello world</p>");
        let output = run_command(&["html-entities", "--file", file.file_path.to_str().unwrap()]);
        assert_eq!(output.trim(), "&lt;br&gt;&lt;p&gt;hello world&lt;/p&gt;");
    }

    #[test]
    fn test_html_entities_decode_file() {
        let file = write_to_file("&lt;br&gt;&lt;p&gt;hello world&lt;/p&gt;");
        let output = run_command(&[
            "html-entities",
            "--file",
            file.file_path.to_str().unwrap(),
            "--decode",
        ]);
        assert_eq!(output.trim(), "<br><p>hello world</p>");
    }

    #[test]
    fn test_html_entities_encode_attribute_string() {
        let output = run_command(&[
            "html-entities",
            "--string",
            "<br><p>hello world</p>",
            "--attribute",
        ]);
        assert_eq!(
            output.trim(),
            "&lt;br&gt;&lt;p&gt;hello&#x20;world&lt;&#x2F;p&gt;"
        );
    }

    #[test]
    fn test_html_entities_decode_attribute_string() {
        let output = run_command(&[
            "html-entities",
            "--string",
            "&lt;br&gt;&lt;p&gt;hello world&lt;/p&gt;",
            "--decode",
            "--attribute",
        ]);
        assert_eq!(output.trim(), "<br><p>hello world</p>");
    }

    #[test]
    fn test_sha1_hash_string() {
        let output = run_command(&["sha1", "--string", "hello world"]);
        assert_eq!(output.trim(), "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed");
    }

    #[test]
    fn test_sha256_hash_string() {
        let output = run_command(&["sha256", "--string", "hello world"]);
        assert_eq!(
            output.trim(),
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }

    #[test]
    fn test_sha512_hash_string() {
        let output = run_command(&["sha512", "--string", "hello world"]);
        assert_eq!(
            output.trim(),
            "309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f"
        );
    }

    #[test]
    fn test_sha384_hash_string() {
        let output = run_command(&["sha384", "--string", "hello world"]);
        assert_eq!(
            output.trim(),
            "fdbd8e75a67f29f701a4e040385e2e23986303ea10239211af907fcbb83578b3e417cb71ce646efd0819dd8c088de1bd"
        );
    }

    #[test]
    fn test_sha224_hash_string() {
        let output = run_command(&["sha224", "--string", "hello world"]);
        assert_eq!(
            output.trim(),
            "2f05477fc24bb4faefd86517156dafdecec45b8ad3cf2522a563582b"
        );
    }

    #[test]
    fn test_sha512_256_hash_string() {
        let output = run_command(&["sha512-256", "--string", "hello world"]);
        assert_eq!(
            output.trim(),
            "0ac561fac838104e3f2e4ad107b4bee3e938bf15f2b15f009ccccd61a913f017"
        );
    }

    #[test]
    fn test_sha512_224_hash_string() {
        let output = run_command(&["sha512-224", "--string", "hello world"]);
        assert_eq!(
            output.trim(),
            "22e0d52336f64a998085078b05a6e37b26f8120f43bf4db4c43a64ee"
        );
    }

    #[test]
    fn test_md5_hash_string() {
        let output = run_command(&["md5", "--string", "hello world"]);
        assert_eq!(output.trim(), "5eb63bbbe01eeed093cb22bb8f5acdc3");
    }

    #[test]
    fn test_md5_hash_stdin() {
        let output = run_command_stdin(&["md5", "-"], "hello world");
        assert_eq!(output.trim(), "5eb63bbbe01eeed093cb22bb8f5acdc3");
    }

    #[test]
    fn test_md5_hash_file() {
        let file = write_to_file("hello world");
        let output = run_command(&["md5", "--file", file.file_path.to_str().unwrap()]);
        assert_eq!(output.trim(), "5eb63bbbe01eeed093cb22bb8f5acdc3");
    }

    #[test]
    fn test_integer_to_hex_string() {
        let output = run_command(&["integer-to-hex", "--string", "10"]);
        assert_eq!(output.trim(), "0a");
    }

    #[test]
    fn test_integer_to_hex_stdin() {
        let output = run_command_stdin(&["integer-to-hex", "-"], "10");
        assert_eq!(output.trim(), "0a");
    }

    #[test]
    fn test_integer_to_hex_file() {
        let file = write_to_file("10");
        let output = run_command(&["integer-to-hex", "--file", file.file_path.to_str().unwrap()]);
        assert_eq!(output.trim(), "0a");
    }

    #[test]
    fn test_integer_to_octal_string() {
        let output = run_command(&["integer-to-octal", "--string", "10"]);
        assert_eq!(output.trim(), "012");
    }

    #[test]
    fn test_integer_to_octal_stdin() {
        let output = run_command_stdin(&["integer-to-octal", "-"], "10");
        assert_eq!(output.trim(), "012");
    }

    #[test]
    fn test_integer_to_octal_file() {
        let file = write_to_file("10");
        let output = run_command(&[
            "integer-to-octal",
            "--file",
            file.file_path.to_str().unwrap(),
        ]);
        assert_eq!(output.trim(), "012");
    }

    #[test]
    fn test_integer_to_binary_string() {
        let output = run_command(&["integer-to-binary", "--string", "10"]);
        assert_eq!(output.trim(), "00001010");
    }

    #[test]
    fn test_integer_to_binary_stdin() {
        let output = run_command_stdin(&["integer-to-binary", "-"], "10");
        assert_eq!(output.trim(), "00001010");
    }

    #[test]
    fn test_integer_to_binary_file() {
        let file = write_to_file("10");
        let output = run_command(&[
            "integer-to-binary",
            "--file",
            file.file_path.to_str().unwrap(),
        ]);
        assert_eq!(output.trim(), "00001010");
    }

    #[test]
    fn test_hex_to_integer_string() {
        let output = run_command(&["hex-to-integer", "--string", "0a"]);
        assert_eq!(output.trim(), "10");
    }

    #[test]
    fn test_hex_to_integer_stdin() {
        let output = run_command_stdin(&["hex-to-integer", "-"], "0a");
        assert_eq!(output.trim(), "10");
    }

    #[test]
    fn test_hex_to_integer_file() {
        let file = write_to_file("0a");
        let output = run_command(&["hex-to-integer", "--file", file.file_path.to_str().unwrap()]);
        assert_eq!(output.trim(), "10");
    }

    #[test]
    fn test_hex_to_binary_string() {
        let output = run_command(&["hex-to-binary", "--string", "0a"]);
        assert_eq!(output.trim(), "00001010");
    }

    #[test]
    fn test_hex_to_binary_stdin() {
        let output = run_command_stdin(&["hex-to-binary", "-"], "0a");
        assert_eq!(output.trim(), "00001010");
    }

    #[test]
    fn test_hex_to_binary_file() {
        let file = write_to_file("0a");
        let output = run_command(&["hex-to-binary", "--file", file.file_path.to_str().unwrap()]);
        assert_eq!(output.trim(), "00001010");
    }

    #[test]
    fn test_hex_to_octal_string() {
        let output = run_command(&["hex-to-octal", "--string", "0a"]);
        assert_eq!(output.trim(), "012");
    }

    #[test]
    fn test_hex_to_octal_stdin() {
        let output = run_command_stdin(&["hex-to-octal", "-"], "0a");
        assert_eq!(output.trim(), "012");
    }

    #[test]
    fn test_hex_to_octal_file() {
        let file = write_to_file("0a");
        let output = run_command(&["hex-to-octal", "--file", file.file_path.to_str().unwrap()]);
        assert_eq!(output.trim(), "012");
    }

    #[test]
    fn test_octal_to_integer_string() {
        let output = run_command(&["octal-to-integer", "--string", "012"]);
        assert_eq!(output.trim(), "10");
    }

    #[test]
    fn test_octal_to_integer_stdin() {
        let output = run_command_stdin(&["octal-to-integer", "-"], "012");
        assert_eq!(output.trim(), "10");
    }

    #[test]
    fn test_octal_to_integer_file() {
        let file = write_to_file("012");
        let output = run_command(&[
            "octal-to-integer",
            "--file",
            file.file_path.to_str().unwrap(),
        ]);
        assert_eq!(output.trim(), "10");
    }

    #[test]
    fn test_octal_to_hex_string() {
        let output = run_command(&["octal-to-hex", "--string", "012"]);
        assert_eq!(output.trim(), "0a");
    }

    #[test]
    fn test_octal_to_hex_stdin() {
        let output = run_command_stdin(&["octal-to-hex", "-"], "012");
        assert_eq!(output.trim(), "0a");
    }

    #[test]
    fn test_octal_to_hex_file() {
        let file = write_to_file("012");
        let output = run_command(&["octal-to-hex", "--file", file.file_path.to_str().unwrap()]);
        assert_eq!(output.trim(), "0a");
    }

    #[test]
    fn test_octal_to_binary_string() {
        let output = run_command(&["octal-to-binary", "--string", "012"]);
        assert_eq!(output.trim(), "00001010");
    }

    #[test]
    fn test_octal_to_binary_stdin() {
        let output = run_command_stdin(&["octal-to-binary", "-"], "012");
        assert_eq!(output.trim(), "00001010");
    }

    #[test]
    fn test_octal_to_binary_file() {
        let file = write_to_file("012");
        let output = run_command(&[
            "octal-to-binary",
            "--file",
            file.file_path.to_str().unwrap(),
        ]);
        assert_eq!(output.trim(), "00001010");
    }

    #[test]
    fn test_binary_to_hex_string() {
        let output = run_command(&["binary-to-hex", "--string", "00001010"]);
        assert_eq!(output.trim(), "0a");
    }

    #[test]
    fn test_binary_to_hex_stdin() {
        let output = run_command_stdin(&["binary-to-hex", "-"], "00001010");
        assert_eq!(output.trim(), "0a");
    }

    #[test]
    fn test_binary_to_hex_file() {
        let file = write_to_file("00001010");
        let output = run_command(&["binary-to-hex", "--file", file.file_path.to_str().unwrap()]);
        assert_eq!(output.trim(), "0a");
    }

    #[test]
    fn test_binary_to_octal_string() {
        let output = run_command(&["binary-to-octal", "--string", "00001010"]);
        assert_eq!(output.trim(), "012");
    }

    #[test]
    fn test_binary_to_octal_stdin() {
        let output = run_command_stdin(&["binary-to-octal", "-"], "00001010");
        assert_eq!(output.trim(), "012");
    }

    #[test]
    fn test_binary_to_octal_file() {
        let file = write_to_file("00001010");
        let output = run_command(&[
            "binary-to-octal",
            "--file",
            file.file_path.to_str().unwrap(),
        ]);
        assert_eq!(output.trim(), "012");
    }

    #[test]
    fn test_binary_to_integer_string() {
        let output = run_command(&["binary-to-integer", "--string", "00001010"]);
        assert_eq!(output.trim(), "10");
    }

    #[test]
    fn test_binary_to_integer_stdin() {
        let output = run_command_stdin(&["binary-to-integer", "-"], "00001010");
        assert_eq!(output.trim(), "10");
    }

    #[test]
    fn test_binary_to_integer_file() {
        let file = write_to_file("00001010");
        let output = run_command(&[
            "binary-to-integer",
            "--file",
            file.file_path.to_str().unwrap(),
        ]);
        assert_eq!(output.trim(), "10");
    }

    // Remove the file at /tmp/test.txt if it exists.
    fn cleanup_output_file() {
        let output_file = Path::new("/tmp/test.txt");
        if output_file.exists() {
            let _ = fs::remove_file(output_file);
        }
    }

    // Test writing the output to a file.
    #[test]
    fn test_output_option_writes_to_file() {
        let file = write_to_file("");

        let _output = run_command(&[
            "rot13",
            "--string",
            "hello",
            "--output",
            file.file_path.to_str().unwrap(),
        ]);

        let file_content =
            fs::read_to_string(file.file_path.as_path()).expect("Failed to read output file");
        assert_eq!(file_content, "uryyb");

        cleanup_output_file();
    }

    // Test overwriting an existing file.
    #[test]
    fn test_output_option_overwrites_existing_file() {
        let file = write_to_file("test");

        // Create the file with initial content.
        fs::write(file.file_path.as_path(), "initial content")
            .expect("Failed to write initial content");

        let _output = run_command(&[
            "rot13",
            "--string",
            "hello",
            "--output",
            file.file_path.to_str().unwrap(),
        ]);

        let file_content =
            fs::read_to_string(file.file_path.as_path()).expect("Failed to read output file");
        assert_eq!(file_content, "uryyb");

        cleanup_output_file();
    }

    // Test handling errors when creating or writing to the output file.
    // #[test]
    // fn test_output_option_write_error() {
    //     // Use a directory as the output file to trigger an error.
    //     let output_file = "/tmp";

    //     let output = run_command(&["rot13", "--string", "hello", "--output", output_file]);

    //     assert!(
    //         output.contains("directory"),
    //         "Expected error message regarding file creation failure, got: {}",
    //         output
    //     );
    // }
}
