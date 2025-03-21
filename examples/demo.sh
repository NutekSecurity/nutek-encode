#!/bin/bash

# Check if the file exists
if [ -f "Cargo.toml" ]; then
    # Check if the file contains the specified content
    if grep -q 'name = "nutek-encode"' "Cargo.toml"; then
        #echo "File Cargo.toml exists and contains the specified content."
        continue
    else
        echo "File Cargo.toml exists but does not contain the specified content."
        exit 1
    fi
else
    echo "File Cargo.toml does not exist."
    exit 1
fi

cargo build
clear

echo "$ 'Welcome to Nutek Encode Demo\!'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 5

# Base64 encode and decode
echo "$ 'Base64 encode and decode'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 5
echo "$ nutek-encode base64 --string 'hello world'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode base64 --string 'hello world'
sleep 0.5
echo
sleep 5
echo "$ nutek-encode base64 --string 'aGVsbG8gd29ybGQ=' --decode" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode base64 --string 'aGVsbG8gd29ybGQ=' --decode
sleep 0.5
echo
sleep 5

# URL-safe Base64 encode and decode
echo "$ 'URL-safe Base64 encode and decode'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 5
echo "$ nutek-encode url-safe-base64 --string 'hello world'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode url-safe-base64 --string 'hello world'
sleep 0.5
echo
sleep 5
echo "$ nutek-encode url-safe-base64 --string 'aGVsbG8gd29ybGQ=' --decode" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode url-safe-base64 --string 'aGVsbG8gd29ybGQ=' --decode
sleep 0.5
echo
sleep 5

# URL encode and decode
echo "$ 'URL encode and decode'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 5
echo "$ nutek-encode url --string 'hello world'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode url --string 'hello world'
sleep 0.5
echo
sleep 5
echo "$ nutek-encode url --string 'hello%20world' --decode" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode url --string 'hello%20world' --decode
sleep 0.5
echo
sleep 5

# Hex encode and decode
echo "$ 'Hex encode and decode'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 5
echo "$ nutek-encode hex --string 'hello world'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode hex --string 'hello world'
sleep 0.5
echo
sleep 5
echo "$ nutek-encode hex --string '68656c6c6f20776f726c64' --decode" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode hex --string '68656c6c6f20776f726c64' --decode
sleep 0.5
echo
sleep 5

# Rot13 encode and decode
echo "$ 'Rot13 encode and decode'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 5
echo "$ nutek-encode rot13 --string 'hello world'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode rot13 --string 'hello world'
sleep 0.5
echo
sleep 5
echo "$ nutek-encode rot13 --string 'uryyb jbeyq'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode rot13 --string 'uryyb jbeyq'
sleep 0.5
echo
sleep 5

# Binary encode and decode
echo "$ 'Binary encode and decode'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 5
echo "$ nutek-encode binary --string 'hello world'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode binary --string 'hello world'
sleep 0.5
echo
sleep 5
echo "$ nutek-encode binary --string '0110100001100101011011000110110001101111001000000111011101101111011100100110110001100100' --decode" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode binary --string '0110100001100101011011000110110001101111001000000111011101101111011100100110110001100100' --decode
sleep 0.5
echo
sleep 5

# Octal encode and decode
echo "$ 'Octal encode and decode'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 5
echo "$ nutek-encode octal --string 'hello world'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode octal --string 'hello world'
sleep 0.5
echo
sleep 5
echo "$ nutek-encode octal --string '150145154154157040167157162154144' --decode" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode octal --string '150145154154157040167157162154144' --decode
sleep 0.5
echo
sleep 5

# Decimal encode and decode
echo "$ 'Decimal encode and decode'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 5
echo "$ nutek-encode decimal --string 'hello world'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode decimal --string 'hello world'
sleep 0.5
echo
sleep 5
echo "$ nutek-encode decimal --string '104 101 108 108 111 32 119 111 114 108 100' --decode" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode decimal --string '104 101 108 108 111 32 119 111 114 108 100' --decode
sleep 0.5
echo
sleep 5

# HTML entities encode and decode
echo "$ 'HTML entities encode and decode'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 5
echo "$ nutek-encode html-entities --string '<br><p>hello world</p>'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode html-entities --string '<br><p>hello world</p>'
sleep 0.5
echo
sleep 5
echo "$ nutek-encode html-entities --string '&lt;br&gt;&lt;p&gt;hello world&lt;/p&gt;' --decode" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode html-entities --string '&lt;br&gt;&lt;p&gt;hello world&lt;/p&gt;' --decode
sleep 0.5
echo
sleep 5

# SHA-1 hash
echo "$ 'SHA-1 hash'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 5
echo "$ nutek-encode sha1 --string 'hello world'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode sha1 --string 'hello world'
sleep 0.5
echo
sleep 5

# SHA-256 hash
echo "$ 'SHA-256 hash'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 5
echo "$ nutek-encode sha256 --string 'hello world'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode sha256 --string 'hello world'
sleep 0.5
echo
sleep 5

# SHA-512 hash
echo "$ 'SHA-512 hash'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 5
echo "$ nutek-encode sha512 --string 'hello world'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode sha512 --string 'hello world'
sleep 0.5
echo
sleep 5

# SHA-384 hash
echo "$ 'SHA-384 hash'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 5
echo "$ nutek-encode sha384 --string 'hello world'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode sha384 --string 'hello world'
sleep 0.5
echo
sleep 5

# SHA-224 hash
echo "$ 'SHA-224 hash'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 5
echo "$ nutek-encode sha224 --string 'hello world'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode sha224 --string 'hello world'
sleep 0.5
echo
sleep 5

# SHA-512/256 hash
echo "$ 'SHA-512/256 hash'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 5
echo "$ nutek-encode sha512-256 --string 'hello world'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode sha512-256 --string 'hello world'
sleep 0.5
echo
sleep 5

# SHA-512/224 hash
echo "$ 'SHA-512/224 hash'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 5
echo "$ nutek-encode sha512-224 --string 'hello world'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode sha512-224 --string 'hello world'
sleep 0.5
echo
sleep 5

# MD5 hash
echo "$ 'MD5 hash'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 5
echo "$ nutek-encode md5 --string 'hello world'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode md5 --string 'hello world'
sleep 0.5
echo
sleep 5

# Integer to Hex
echo "$ 'Integer to Hex'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 5
echo "$ nutek-encode integer-to-hex --string '10'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode integer-to-hex --string '10'
sleep 0.5
echo
sleep 5

# Integer to Octal
echo "$ 'Integer to Octal'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 5
echo "$ nutek-encode integer-to-octal --string '10'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode integer-to-octal --string '10'
sleep 0.5
echo
sleep 5

# Integer to Binary
echo "$ 'Integer to Binary'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 5
echo "$ nutek-encode integer-to-binary --string '10'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode integer-to-binary --string '10'
sleep 0.5
echo
sleep 5

# Hex to Integer
echo "$ 'Hex to Integer'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 5
echo "$ nutek-encode hex-to-integer --string '0a'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode hex-to-integer --string '0a'
sleep 0.5
echo
sleep 5

# Hex to Binary
echo "$ 'Hex to Binary'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 5
echo "$ nutek-encode hex-to-binary --string '0a'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode hex-to-binary --string '0a'
sleep 0.5
echo
sleep 5

# Hex to Octal
echo "$ 'Hex to Octal'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 5
echo "$ nutek-encode hex-to-octal --string '0a'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode hex-to-octal --string '0a'
sleep 0.5
echo
sleep 5

# Octal to Integer
echo "$ 'Octal to Integer'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 5
echo "$ nutek-encode octal-to-integer --string '012'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode octal-to-integer --string '012'
sleep 0.5
echo
sleep 5

# Octal to Hex
echo "$ 'Octal to Hex'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 5
echo "$ nutek-encode octal-to-hex --string '012'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode octal-to-hex --string '012'
sleep 0.5
echo
sleep 5

# Octal to Binary
echo "$ 'Octal to Binary'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 5
echo "$ nutek-encode octal-to-binary --string '012'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode octal-to-binary --string '012'
sleep 0.5
echo
sleep 5

# Binary to Hex
echo "$ 'Binary to Hex'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 5
echo "$ nutek-encode binary-to-hex --string '00001010'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode binary-to-hex --string '00001010'
sleep 0.5
echo
sleep 5

# Binary to Octal
echo "$ 'Binary to Octal'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 5
echo "$ nutek-encode binary-to-octal --string '00001010'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode binary-to-octal --string '00001010'
sleep 0.5
echo
sleep 5

# Binary to Integer
echo "$ 'Binary to Integer'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 5
echo "$ nutek-encode binary-to-integer --string '00001010'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
target/debug/nutek-encode binary-to-integer --string '00001010'
sleep 0.5
echo
sleep 5

echo "$ 'Thanks for watching\!\n- Neosb'" > examples/tmp.scenario && \
asciinema-scenario examples/tmp.scenario > examples/tmp.cast && \
asciinema play --quiet examples/tmp.cast
sleep 0.5
echo
sleep 14
