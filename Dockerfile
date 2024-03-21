FROM rust:1.63 as build

# Install git and other dependencies, if necessary
RUN apt-get update && apt-get install -y git && rm -rf /var/lib/apt/lists/*

# Clone and build your project directly
RUN git clone https://github.com/medinardaniel/rust-web-service.git /usr/src/myapp && \
    cd /usr/src/myapp && \
    cargo build --release

FROM debian:buster-slim as runtime
COPY --from=build /usr/src/myapp/target/release/rust-web-service /usr/local/bin/rust-web-service
CMD ["./rust-web-service"]
