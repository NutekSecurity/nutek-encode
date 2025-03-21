# Use the official Rust image as the base image
FROM rust:alpine AS builder

WORKDIR /myapp

RUN apk update && \
    apk add build-base musl-dev

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./
COPY ./src/ ./src/


# Build the application
RUN cargo build --release

# Use a minimal base image for the final stage
FROM alpine:latest

# Copy the compiled binary from the builder stage
COPY --from=builder /myapp/target/release/nutek-encode /usr/bin/nutek-encode

ENTRYPOINT ["nutek-encode"]

# Set the startup command to run the binary
CMD ["nutek-encode"]
