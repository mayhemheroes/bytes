# Use the base builder image for Rust projects as the builder stage
FROM ghcr.io/nottirb/fuzz-rust:latest as builder

# Add source to the build stage
ADD . /bytes
WORKDIR /bytes

# Build the fuzz targets
RUN cd ./fuzz && cargo +nightly hfuzz build

# Use Ubuntu 20.04 as the final image
FROM ubuntu:20.04

# Copy the fuzz targets from the builder stage
COPY --from=builder bytes/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/fuzz_bytebuf_de /
COPY --from=builder bytes/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/fuzz_bytebuf /
COPY --from=builder bytes/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/fuzz_bytecache /
COPY --from=builder bytes/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/fuzz_bytes /