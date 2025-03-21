/// This module provides various encoding and decoding functions.
pub mod encoder {
    use base64::{Engine as _, engine::general_purpose::URL_SAFE, prelude::BASE64_STANDARD};
    use hex;
    use htmlescape::{DecodeErrKind, decode_html, encode_attribute, encode_minimal};
    use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
    use sha1::{Digest as Digest1, Sha1};
    use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};
    use std::{error::Error, str};

    /// Encodes a string into URL-safe Base64 format.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let encoded = encoder::encode_urlsafe_base64("hello").unwrap();
    /// assert_eq!(encoded, "aGVsbG8=");
    /// ```
    pub fn encode_urlsafe_base64(data: &str) -> Result<String, Box<dyn Error>> {
        Ok(URL_SAFE.encode(data.as_bytes()))
    }

    /// Decodes a URL-safe Base64 encoded string.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let decoded = encoder::decode_urlsafe_base64("aGVsbG8=").unwrap();
    /// assert_eq!(decoded, "hello");
    /// ```
    pub fn decode_urlsafe_base64(data: &str) -> Result<String, Box<dyn Error>> {
        let decoded_bytes = URL_SAFE.decode(data.as_bytes())?;
        let decoded_str = String::from_utf8(decoded_bytes)?;
        Ok(decoded_str)
    }

    /// Encodes a string into standard Base64 format.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let encoded = encoder::encode_base64("hello").unwrap();
    /// assert_eq!(encoded, "aGVsbG8=");
    /// ```
    pub fn encode_base64(data: &str) -> Result<String, Box<dyn Error>> {
        Ok(BASE64_STANDARD.encode(data.as_bytes()))
    }

    /// Decodes a standard Base64 encoded string.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let decoded = encoder::decode_base64("aGVsbG8=").unwrap();
    /// assert_eq!(decoded, "hello");
    /// ```
    pub fn decode_base64(data: &str) -> Result<String, Box<dyn Error>> {
        let decoding_result = BASE64_STANDARD.decode(data)?;
        let decoded_str = std::str::from_utf8(&decoding_result)?;
        Ok(decoded_str.to_string())
    }

    /// Encodes a string into URL percent-encoded format.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let encoded = encoder::encode_url("hello world!").unwrap();
    /// assert_eq!(encoded, "hello%20world%21");
    /// ```
    pub fn encode_url(data: &str) -> Result<String, Box<dyn Error>> {
        Ok(utf8_percent_encode(data, NON_ALPHANUMERIC).to_string())
    }

    /// Decodes a URL percent-encoded string.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let decoded = encoder::decode_url("hello%20world%21").unwrap();
    /// assert_eq!(decoded, "hello world!");
    /// ```
    pub fn decode_url(data: &str) -> Result<String, Box<dyn Error>> {
        let decoded_str = percent_encoding::percent_decode_str(data).decode_utf8()?;
        Ok(decoded_str.to_string())
    }

    /// Encodes a string into binary format.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let encoded = encoder::encode_binary("hello").unwrap();
    /// assert_eq!(encoded, "0110100001100101011011000110110001101111");
    /// ```
    pub fn encode_binary(data: &str) -> Result<String, Box<dyn Error>> {
        Ok(data.chars().map(|c| format!("{:08b}", c as u8)).collect())
    }

    /// Decodes a binary encoded string.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let decoded = encoder::decode_binary("0110100001100101011011000110110001101111").unwrap();
    /// assert_eq!(decoded, "hello");
    /// ```
    pub fn decode_binary(data: &str) -> Result<String, Box<dyn Error>> {
        if data.len() % 8 != 0 {
            return Err("Invalid binary data length".into());
        }
        let decoded_str: Result<String, Box<dyn Error>> = data
            .chars()
            .collect::<Vec<_>>()
            .chunks(8)
            .map(|chunk| {
                let s: String = chunk.iter().collect();
                let char_result = char::from_u32(u32::from_str_radix(&s, 2)?);
                char_result.ok_or_else(|| "Invalid binary data".into())
            })
            .collect();
        decoded_str
    }

    /// Encodes a string into hexadecimal format.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let encoded = encoder::encode_hex("hello").unwrap();
    /// assert_eq!(encoded, "68656c6c6f");
    /// ```
    pub fn encode_hex(data: &str) -> Result<String, Box<dyn Error>> {
        Ok(data.chars().map(|c| format!("{:02x}", c as u8)).collect())
    }

    /// Decodes a hexadecimal encoded string.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let decoded = encoder::decode_hex("68656c6c6f").unwrap();
    /// assert_eq!(decoded, "hello");
    /// ```
    pub fn decode_hex(data: &str) -> Result<String, Box<dyn Error>> {
        if data.len() % 2 != 0 {
            return Err("Invalid hex data length".into());
        }
        let decoded_str: Result<String, Box<dyn Error>> = data
            .chars()
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|chunk| {
                let s: String = chunk.iter().collect();
                let char_result = char::from_u32(u32::from_str_radix(&s, 16)?);
                char_result.ok_or_else(|| "Invalid hex data".into())
            })
            .collect();
        decoded_str
    }

    /// Encodes a string using ROT13.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let encoded = encoder::encode_rot13("hello").unwrap();
    /// assert_eq!(encoded, "uryyb");
    /// ```
    pub fn encode_rot13(data: &str) -> Result<String, Box<dyn Error>> {
        Ok(data
            .chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    let offset = (c as u8).wrapping_sub(first);
                    let rotated = (offset + 13) % 26;
                    (first + rotated) as char
                } else {
                    c
                }
            })
            .collect())
    }

    /// Encodes a string into octal format.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let encoded = encoder::encode_octal("hello").unwrap();
    /// assert_eq!(encoded, "150145154154157");
    /// ```
    pub fn encode_octal(data: &str) -> Result<String, Box<dyn Error>> {
        Ok(data.chars().map(|c| format!("{:03o}", c as u8)).collect())
    }

    /// Decodes an octal encoded string.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let decoded = encoder::decode_octal("150145154154157").unwrap();
    /// assert_eq!(decoded, "hello");
    /// ```
    pub fn decode_octal(data: &str) -> Result<String, Box<dyn Error>> {
        // Check that the overall length is a multiple of 3.
        if data.len() % 3 != 0 {
            return Err("Invalid octal data length".into());
        }

        // Validate that every character is in the octal range '0'..='7'.
        for (i, ch) in data.chars().enumerate() {
            if ch < '0' || ch > '7' {
                return Err(format!("Invalid octal digit '{}' at position {}", ch, i).into());
            }
        }

        let mut result = String::new();

        // Process the string in chunks of 3 characters.
        for chunk in data.as_bytes().chunks(3) {
            // Convert the chunk (which is a &[u8]) into a &str.
            let chunk_str = str::from_utf8(chunk)?;
            // Convert the 3-digit octal string to a u32.
            let val = u32::from_str_radix(chunk_str, 8)?;
            // Convert the u32 to a char. If the codepoint is not valid, return an error.
            match char::from_u32(val) {
                Some(ch) => result.push(ch),
                None => return Err(format!("Invalid octal codepoint: {}", chunk_str).into()),
            }
        }
        Ok(result)
    }

    /// Encodes a string into decimal format.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let encoded = encoder::encode_decimal("hello").unwrap();
    /// assert_eq!(encoded, "104 101 108 108 111");
    /// ```
    pub fn encode_decimal(data: &str) -> Result<String, Box<dyn Error>> {
        let decimal_encoded: String = data.chars().map(|c| format!("{:03} ", c as u8)).collect();
        Ok(decimal_encoded.trim().to_string())
    }

    /// Decodes a decimal encoded string.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let decoded = encoder::decode_decimal("104 101 108 108 111").unwrap();
    /// assert_eq!(decoded, "hello");
    /// ```
    pub fn decode_decimal(data: &str) -> Result<String, Box<dyn Error>> {
        let decoded_str: Result<String, Box<dyn Error>> = data
            .split_whitespace()
            .map(|s| {
                let char_result = char::from_u32(s.parse()?);
                char_result.ok_or_else(|| "Invalid decimal data".into())
            })
            .collect();
        decoded_str
    }

    /// Encodes a string into HTML entities.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let encoded = encoder::encode_html_entities("hello & world").unwrap();
    /// assert_eq!(encoded, "hello &amp; world");
    /// ```
    pub fn encode_html_entities(data: &str) -> Result<String, Box<dyn Error>> {
        Ok(encode_minimal(data))
    }

    /// Decodes a string with HTML entities.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let decoded = encoder::decode_html_entities("hello &amp; world").unwrap();
    /// assert_eq!(decoded, "hello & world");
    /// ```
    pub fn decode_html_entities(data: &str) -> Result<String, DecodeErrKind> {
        decode_html(data).map_err(|e| e.kind)
    }

    /// Encodes a string into HTML attribute entities.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let encoded = encoder::encode_html_entities_attribute("hello & world").unwrap();
    /// assert_eq!(encoded, "hello&#x20;&amp;&#x20;world");
    /// ```
    pub fn encode_html_entities_attribute(data: &str) -> Result<String, Box<dyn Error>> {
        Ok(encode_attribute(data))
    }

    /// Decodes a string with HTML attribute entities.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let decoded = encoder::decode_html_entities_attribute("hello &amp; world").unwrap();
    /// assert_eq!(decoded, "hello & world");
    /// ```
    pub fn decode_html_entities_attribute(data: &str) -> Result<String, DecodeErrKind> {
        decode_html_entities(data)
    }

    /// Encodes an integer string to hexadecimal format.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let encoded = encoder::encode_integer_to_hex("255").unwrap();
    /// assert_eq!(encoded, "ff");
    /// ```
    pub fn encode_integer_to_hex(data: &str) -> Result<String, Box<dyn Error>> {
        let parsed_int = data.parse::<u32>()?;
        Ok(format!("{:02x}", parsed_int))
    }

    /// Encodes an integer string to octal format.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let encoded = encoder::encode_integer_to_octal("255").unwrap();
    /// assert_eq!(encoded, "377");
    /// ```
    pub fn encode_integer_to_octal(data: &str) -> Result<String, Box<dyn Error>> {
        let parsed_int = data.parse::<u32>()?;
        Ok(format!("{:03o}", parsed_int))
    }

    /// Encodes an integer string to binary format.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let encoded = encoder::encode_integer_to_binary("255").unwrap();
    /// assert_eq!(encoded, "11111111");
    /// ```
    pub fn encode_integer_to_binary(data: &str) -> Result<String, Box<dyn Error>> {
        let parsed_int = data.parse::<u32>()?;
        Ok(format!("{:08b}", parsed_int))
    }

    /// Encodes a hexadecimal string to integer format.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let encoded = encoder::encode_hex_to_integer("ff").unwrap();
    /// assert_eq!(encoded, "255");
    /// ```
    pub fn encode_hex_to_integer(data: &str) -> Result<String, Box<dyn Error>> {
        let parsed_int = u32::from_str_radix(data, 16)?;
        Ok(parsed_int.to_string())
    }

    /// Encodes a hexadecimal string to binary format.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let encoded = encoder::encode_hex_to_binary("ff").unwrap();
    /// assert_eq!(encoded, "11111111");
    /// ```
    pub fn encode_hex_to_binary(data: &str) -> Result<String, Box<dyn Error>> {
        let parsed_int = u32::from_str_radix(data, 16)?;
        Ok(format!("{:08b}", parsed_int))
    }

    /// Encodes a hexadecimal string to octal format.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let encoded = encoder::encode_hex_to_octal("ff").unwrap();
    /// assert_eq!(encoded, "377");
    /// ```
    pub fn encode_hex_to_octal(data: &str) -> Result<String, Box<dyn Error>> {
        let parsed_int = u32::from_str_radix(data, 16)?;
        Ok(format!("{:03o}", parsed_int))
    }

    /// Encodes an octal string to integer format.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let encoded = encoder::encode_octal_to_integer("377").unwrap();
    /// assert_eq!(encoded, "255");
    /// ```
    pub fn encode_octal_to_integer(data: &str) -> Result<String, Box<dyn Error>> {
        let parsed_int = u32::from_str_radix(data, 8)?;
        Ok(parsed_int.to_string())
    }

    /// Encodes an octal string to hexadecimal format.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let encoded = encoder::encode_octal_to_hex("377").unwrap();
    /// assert_eq!(encoded, "ff");
    /// ```
    pub fn encode_octal_to_hex(data: &str) -> Result<String, Box<dyn Error>> {
        let parsed_int = u32::from_str_radix(data, 8)?;
        Ok(format!("{:02x}", parsed_int))
    }

    /// Encodes an octal string to binary format.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let encoded = encoder::encode_octal_to_binary("377").unwrap();
    /// assert_eq!(encoded, "11111111");
    /// ```
    pub fn encode_octal_to_binary(data: &str) -> Result<String, Box<dyn Error>> {
        let parsed_int = u32::from_str_radix(data, 8)?;
        Ok(format!("{:08b}", parsed_int))
    }

    /// Encodes a binary string to hexadecimal format.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let encoded = encoder::encode_binary_to_hex("11111111").unwrap();
    /// assert_eq!(encoded, "ff");
    /// ```
    pub fn encode_binary_to_hex(data: &str) -> Result<String, Box<dyn Error>> {
        let parsed_int = u32::from_str_radix(data, 2)?;
        Ok(format!("{:02x}", parsed_int))
    }

    /// Encodes a binary string to octal format.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let encoded = encoder::encode_binary_to_octal("11111111").unwrap();
    /// assert_eq!(encoded, "377");
    /// ```
    pub fn encode_binary_to_octal(data: &str) -> Result<String, Box<dyn Error>> {
        let parsed_int = u32::from_str_radix(data, 2)?;
        Ok(format!("{:03o}", parsed_int))
    }

    /// Encodes a binary string to integer format.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let encoded = encoder::encode_binary_to_integer("11111111").unwrap();
    /// assert_eq!(encoded, "255");
    /// ```
    pub fn encode_binary_to_integer(data: &str) -> Result<String, Box<dyn Error>> {
        let parsed_int = u32::from_str_radix(data, 2)?;
        Ok(parsed_int.to_string())
    }

    /// Encodes a string using SHA-1.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let encoded = encoder::encode_sha1("hello").unwrap();
    /// assert_eq!(encoded, "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d");
    /// ```
    pub fn encode_sha1(data: &str) -> Result<String, Box<dyn Error>> {
        let mut hasher = Sha1::new();
        hasher.update(data);
        let result = hasher.finalize();
        Ok(hex::encode(result))
    }

    /// Encodes a string using SHA-256.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let encoded = encoder::encode_sha256("hello").unwrap();
    /// assert_eq!(encoded, "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
    /// ```
    pub fn encode_sha256(data: &str) -> Result<String, Box<dyn Error>> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        Ok(hex::encode(result))
    }

    /// Encodes a string using SHA-512.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let encoded = encoder::encode_sha512("hello").unwrap();
    /// assert!(encoded.starts_with("9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca72323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043"));
    /// ```
    pub fn encode_sha512(data: &str) -> Result<String, Box<dyn Error>> {
        let mut hasher = Sha512::new();
        hasher.update(data);
        let result = hasher.finalize();
        Ok(hex::encode(result))
    }

    /// Encodes a string using SHA-384.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let encoded = encoder::encode_sha384("hello").unwrap();
    /// assert!(encoded.starts_with("59e1748777448c69de6b800d7a33bbfb9ff1b463e44354c3553bcdb9c666fa90125a3c79f90397bdf5f6a13de828684f"));
    /// ```
    pub fn encode_sha384(data: &str) -> Result<String, Box<dyn Error>> {
        let mut hasher = Sha384::new();
        hasher.update(data);
        let result = hasher.finalize();
        Ok(hex::encode(result))
    }

    /// Encodes a string using SHA-224.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let encoded = encoder::encode_sha224("hello").unwrap();
    /// assert!(encoded.starts_with("ea09ae9cc6768c50fcee903ed054556e5bfc8347907f12598aa24193"));
    /// ```
    pub fn encode_sha224(data: &str) -> Result<String, Box<dyn Error>> {
        let mut hasher = Sha224::new();
        hasher.update(data);
        let result = hasher.finalize();
        Ok(hex::encode(result))
    }

    /// Encodes a string using SHA-512/256.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let encoded = encoder::encode_sha512_256("hello").unwrap();
    /// assert!(encoded.starts_with("e30d87cfa2a75db545eac4d61baf970366a8357c7f72fa95b52d0accb698f13a"));
    /// ```
    pub fn encode_sha512_256(data: &str) -> Result<String, Box<dyn Error>> {
        let mut hasher = Sha512_256::new();
        hasher.update(data);
        let result = hasher.finalize();
        Ok(hex::encode(result))
    }

    /// Encodes a string using SHA-512/224.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let encoded = encoder::encode_sha512_224("hello").unwrap();
    /// assert!(encoded.starts_with("fe8509ed1fb7dcefc27e6ac1a80eddbec4cb3d2c6fe565244374061c"));
    /// ```
    pub fn encode_sha512_224(data: &str) -> Result<String, Box<dyn Error>> {
        let mut hasher = Sha512_224::new();
        hasher.update(data);
        let result = hasher.finalize();
        Ok(hex::encode(result))
    }

    /// Encodes a string using MD5.
    ///
    /// # Examples
    ///
    /// ```
    /// use nutek_encode_lib::encoder;
    /// let encoded = encoder::encode_md5("hello").unwrap();
    /// assert_eq!(encoded, "5d41402abc4b2a76b9719d911017c592");
    /// ```
    pub fn encode_md5(data: &str) -> Result<String, Box<dyn Error>> {
        let result = md5::compute(data);
        Ok(format!("{:x}", result))
    }
}

#[cfg(test)]
mod tests_encoder {
    use super::*;
    use encoder::*;

    #[test]
    fn it_encodes_base64() {
        let data = "hello world";
        let encoded = encode_base64(data).unwrap();
        assert_eq!(encoded, "aGVsbG8gd29ybGQ=");
    }

    #[test]
    fn it_decodes_base64() {
        let data = "aGVsbG8gd29ybGQ=";
        let decoded = decode_base64(data).unwrap();
        assert_eq!(decoded, "hello world");

        // Malformed input
        let malformed_data = "aGVsbG8gd29ybGQ";
        let result = decode_base64(malformed_data);
        assert!(result.is_err());
    }

    #[test]
    fn it_encodes_urlsafe_base64() {
        let data = "hello world";
        let encoded = encode_urlsafe_base64(data).unwrap();
        assert_eq!(encoded, "aGVsbG8gd29ybGQ=");
    }

    #[test]
    fn it_decodes_urlsafe_base64() {
        let data = "aGVsbG8gd29ybGQ=";
        let decoded = decode_urlsafe_base64(data).unwrap();
        assert_eq!(decoded, "hello world");

        // Malformed input
        let malformed_data = "aGVsbG8gd29ybGQ";
        let result = decode_urlsafe_base64(malformed_data);
        assert!(result.is_err());
    }

    #[test]
    fn it_encodes_url() {
        let data = "hello world";
        let encoded = encode_url(data).unwrap();
        assert_eq!(encoded, "hello%20world");
    }

    #[test]
    fn it_decodes_url() {
        let data = "hello%20world";
        let decoded = decode_url(data).unwrap();
        assert_eq!(decoded, "hello world");
    }

    #[test]
    fn it_encodes_hex() {
        let data = "hello world";
        let encoded = encode_hex(data).unwrap();
        assert_eq!(encoded, "68656c6c6f20776f726c64");
    }

    #[test]
    fn it_decodes_hex() {
        let data = "68656c6c6f20776f726c64";
        let decoded = decode_hex(data).unwrap();
        assert_eq!(decoded, "hello world");

        // Malformed input
        let malformed_data = "68656c6c6f20776f726c6";
        let result = decode_hex(malformed_data);
        assert!(result.is_err());
    }

    #[test]
    fn it_encodes_rot13() {
        let data = "hello world";
        let encoded = encode_rot13(data).unwrap();
        assert_eq!(encoded, "uryyb jbeyq");
    }

    #[test]
    fn it_decodes_rot13() {
        let data = "uryyb jbeyq";
        let decoded = encode_rot13(data).unwrap();
        assert_eq!(decoded, "hello world");
    }

    #[test]
    fn it_encodes_binary() {
        let data = "hello world";
        let encoded = encode_binary(data).unwrap();
        assert_eq!(
            encoded,
            "0110100001100101011011000110110001101111001000000111011101101111011100100110110001100100"
        );
    }

    #[test]
    fn it_decodes_binary() {
        let data = "0110100001100101011011000110110001101111001000000111011101101111011100100110110001100100";
        let decoded = decode_binary(data).unwrap();
        assert_eq!(decoded, "hello world");

        // Malformed input
        let malformed_data = "011010000110010101101100011011000110111100100000011101110110111101110010011011000110010";
        let result = decode_binary(malformed_data);
        assert!(result.is_err());
    }

    #[test]
    fn it_encodes_octal() {
        let data = "hello world";
        let encoded = encode_octal(data).unwrap();
        assert_eq!(encoded, "150145154154157040167157162154144");
    }

    #[test]
    fn it_decodes_octal() {
        let data = "150145154154157040167157162154144";
        let decoded = decode_octal(data).unwrap();
        assert_eq!(decoded, "hello world");

        // Malformed input
        let malformed_data = "15014515415415704016715716215414";
        let result = decode_octal(malformed_data);
        assert!(result.is_err());
    }

    #[test]
    fn it_encodes_decimal() {
        let data = "hello world";
        let encoded = encode_decimal(data).unwrap();
        assert_eq!(encoded, "104 101 108 108 111 032 119 111 114 108 100");
    }

    #[test]
    fn it_decodes_decimal() {
        let data = "104 101 108 108 111 32 119 111 114 108 100";
        let decoded = decode_decimal(data).unwrap();
        assert_eq!(decoded, "hello world");

        // Malformed input
        let malformed_data = "104 101 108 108 111 032 119 111 114 108 abc";
        let result = decode_decimal(malformed_data);
        assert!(result.is_err());

        // Malformed input
        let malformed_data = "104 101 108 108 111 032 119 111 114 108 9999999999";
        let result = decode_decimal(malformed_data);
        assert!(result.is_err());

        // Malformed input
        let malformed_data = "104 101 108 108 111 032 119 111 114 108 -1";
        let result = decode_decimal(malformed_data);
        assert!(result.is_err());

        // Malformed input
        let malformed_data = "104 101 108 108 111 032 119 111 114 108 65.5";
        let result = decode_decimal(malformed_data);
        assert!(result.is_err());

        // Malformed input
        let malformed_data = "104 101 108 108 111 032 119 111 114 108 256 abc";
        let result = decode_decimal(malformed_data);
        assert!(result.is_err());
    }

    #[test]
    fn it_encodes_html_entities_minimal() {
        let data = "<br><p>hello world</p>";
        let encoded = encode_html_entities(data).unwrap();
        assert_eq!(encoded, "&lt;br&gt;&lt;p&gt;hello world&lt;/p&gt;");
    }

    #[test]
    fn it_decodes_html_entities() {
        let data = "&lt;br&gt;&lt;p&gt;hello world&lt;/p&gt;";
        let decoded = decode_html_entities(data).unwrap();
        assert_eq!(decoded, "<br><p>hello world</p>");

        // Malformed input
        let malformed_data = "&thisentitydoesnotexist;";
        let result = decode_html_entities(malformed_data);
        assert!(result.is_err());

        // Malformed input
        let malformed_data = "&#xfoo;";
        let result = decode_html_entities(malformed_data);
        assert!(result.is_err());

        // Malformed input
        let malformed_data = "&#xffffff;";
        let result = decode_html_entities(malformed_data);
        assert!(result.is_err());
    }

    #[test]
    fn it_encodes_html_entities_attribute() {
        let data = "<br><p>hello world</p>";
        let encoded = encode_html_entities_attribute(data).unwrap();
        assert_eq!(
            encoded,
            "&lt;br&gt;&lt;p&gt;hello&#x20;world&lt;&#x2F;p&gt;"
        );
    }

    #[test]
    fn it_decodes_html_entities_attribute() {
        let data = "&lt;br&gt;&lt;p&gt;hello&#x20;world&lt;&#x2F;p&gt;";
        let decoded = decode_html_entities_attribute(data).unwrap();
        assert_eq!(decoded, "<br><p>hello world</p>");

        // Malformed input
        let malformed_data = "&thisentitydoesnotexist;";
        let result = decode_html_entities_attribute(malformed_data);
        assert!(result.is_err());

        // Malformed input
        let malformed_data = "&#xfoo;";
        let result = decode_html_entities_attribute(malformed_data);
        assert!(result.is_err());

        // Malformed input
        let malformed_data = "&#xffffff;";
        let result = decode_html_entities_attribute(malformed_data);
        assert!(result.is_err());
    }

    #[test]
    fn it_encodes_integer_to_hex() {
        let data = "10";
        let encoded = encode_integer_to_hex(data).unwrap();
        assert_eq!(encoded, "0a");

        // Malformed input
        let malformed_data = "ten";
        let result = encode_integer_to_hex(malformed_data);
        assert!(result.is_err());
    }

    #[test]
    fn it_encodes_integer_to_octal() {
        let data = "10";
        let encoded = encode_integer_to_octal(data).unwrap();
        assert_eq!(encoded, "012");

        // Malformed input
        let malformed_data = "ten";
        let result = encode_integer_to_octal(malformed_data);
        assert!(result.is_err());
    }

    #[test]
    fn it_encodes_integer_to_binary() {
        let data = "10";
        let encoded = encode_integer_to_binary(data).unwrap();
        assert_eq!(encoded, "00001010");

        // Malformed input
        let malformed_data = "ten";
        let result = encode_integer_to_binary(malformed_data);
        assert!(result.is_err());
    }

    #[test]
    fn it_encodes_hex_to_integer() {
        let data = "0a";
        let decoded = encode_hex_to_integer(data).unwrap();
        assert_eq!(decoded, "10");

        // Malformed input
        let malformed_data = "0g";
        let result = encode_hex_to_integer(malformed_data);
        assert!(result.is_err());
    }

    #[test]
    fn it_encodes_hex_to_binary() {
        let data = "0a";
        let encoded = encode_hex_to_binary(data).unwrap();
        assert_eq!(encoded, "00001010");

        // Malformed input
        let malformed_data = "0g";
        let result = encode_hex_to_binary(malformed_data);
        assert!(result.is_err());
    }

    #[test]
    fn it_encodes_hex_to_octal() {
        let data = "0a";
        let encoded = encode_hex_to_octal(data).unwrap();
        assert_eq!(encoded, "012");

        // Malformed input
        let malformed_data = "0g";
        let result = encode_hex_to_octal(malformed_data);
        assert!(result.is_err());
    }

    #[test]
    fn it_encodes_octal_to_integer() {
        let data = "012";
        let decoded = encode_octal_to_integer(data).unwrap();
        assert_eq!(decoded, "10");

        // Malformed input
        let malformed_data = "018";
        let result = encode_octal_to_integer(malformed_data);
        assert!(result.is_err());
    }

    #[test]
    fn it_encodes_octal_to_hex() {
        let data = "012";
        let encoded = encode_octal_to_hex(data).unwrap();
        assert_eq!(encoded, "0a");

        // Malformed input
        let malformed_data = "018";
        let result = encode_octal_to_hex(malformed_data);
        assert!(result.is_err());
    }

    #[test]
    fn it_encodes_octal_to_binary() {
        let data = "012";
        let encoded = encode_octal_to_binary(data).unwrap();
        assert_eq!(encoded, "00001010");

        // Malformed input
        let malformed_data = "018";
        let result = encode_octal_to_binary(malformed_data);
        assert!(result.is_err());
    }

    #[test]
    fn it_encodes_binary_to_hex() {
        let data = "00001010";
        let encoded = encode_binary_to_hex(data).unwrap();
        assert_eq!(encoded, "0a");

        // Malformed input
        let malformed_data = "00001210";
        let result = encode_binary_to_hex(malformed_data);
        assert!(result.is_err());
    }

    #[test]
    fn it_encodes_binary_to_octal() {
        let data = "00001010";
        let encoded = encode_binary_to_octal(data).unwrap();
        assert_eq!(encoded, "012");

        // Malformed input
        let malformed_data = "00001210";
        let result = encode_binary_to_octal(malformed_data);
        assert!(result.is_err());
    }

    #[test]
    fn it_encodes_binary_to_integer() {
        let data = "00001010";
        let decoded = encode_binary_to_integer(data).unwrap();
        assert_eq!(decoded, "10");

        // Malformed input
        let malformed_data = "00001210";
        let result = encode_binary_to_integer(malformed_data);
        assert!(result.is_err());
    }

    #[test]
    fn it_encodes_sha1() {
        let data = "hello world";
        let encoded = encode_sha1(data).unwrap();
        assert_eq!(encoded, "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed");
    }

    #[test]
    fn it_encodes_sha256() {
        let data = "hello world";
        let encoded = encode_sha256(data).unwrap();
        assert_eq!(
            encoded,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }

    #[test]
    fn it_encodes_sha512() {
        let data = "hello world";
        let encoded = encode_sha512(data).unwrap();
        assert_eq!(
            encoded,
            "309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f"
        );
    }

    #[test]
    fn it_encodes_sha384() {
        let data = "hello world";
        let encoded = encode_sha384(data).unwrap();
        assert_eq!(
            encoded,
            "fdbd8e75a67f29f701a4e040385e2e23986303ea10239211af907fcbb83578b3e417cb71ce646efd0819dd8c088de1bd"
        );
    }

    #[test]
    fn it_encodes_sha224() {
        let data = "hello world";
        let encoded = encode_sha224(data).unwrap();
        assert_eq!(
            encoded,
            "2f05477fc24bb4faefd86517156dafdecec45b8ad3cf2522a563582b"
        );
    }

    #[test]
    fn it_encodes_sha512_256() {
        let data = "hello world";
        let encoded = encode_sha512_256(data).unwrap();
        assert_eq!(
            encoded,
            "0ac561fac838104e3f2e4ad107b4bee3e938bf15f2b15f009ccccd61a913f017"
        );
    }

    #[test]
    fn it_encodes_sha512_224() {
        let data = "hello world";
        let encoded = encode_sha512_224(data).unwrap();
        assert_eq!(
            encoded,
            "22e0d52336f64a998085078b05a6e37b26f8120f43bf4db4c43a64ee"
        );
    }

    #[test]
    fn it_encodes_md5() {
        let data = "hello world";
        let encoded = encode_md5(data).unwrap();
        assert_eq!(encoded, "5eb63bbbe01eeed093cb22bb8f5acdc3");
    }
}
