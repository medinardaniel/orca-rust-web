# Use the official Rust image as a parent image
FROM rust:1.63 as builder

# Create a new empty shell project
RUN USER=root cargo new --bin rust-web-service
WORKDIR /rust-web-service

# Copy your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# This trick will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Now that the dependencies are built, copy your source tree
COPY ./src ./src

# Build for release.
RUN rm ./target/release/deps/rust_web_service*
RUN cargo build --release

# The final stage, copy the binaries and entrypoint from the builder stage
FROM debian:buster-slim
COPY --from=builder /rust-web-service/target/release/rust-web-service .
COPY --from=builder /rust-web-service/Rocket.toml .

# Set the startup command to run your binary
CMD ["./rust-web-service"]