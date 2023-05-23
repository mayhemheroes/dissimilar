# Use the base builder image for Rust projects as the builder stage
FROM ghcr.io/nottirb/fuzz-rust:latest as builder

# Add source to the build stage
ADD . /dissimilar
WORKDIR /dissimilar

# Build the fuzz targets
RUN cd ./fuzz && cargo +nightly fuzz build

# Use Ubuntu 20.04 as the final image
FROM ubuntu:20.04

# Copy the fuzz targets from the builder stage
COPY --from=builder dissimilar/fuzz/target/x86_64-unknown-linux-gnu/release/dissimilar_diff /
COPY --from=builder dissimilar/fuzz/target/x86_64-unknown-linux-gnu/release/fuzz_diff /
COPY --from=builder dissimilar/fuzz/target/x86_64-unknown-linux-gnu/release/fuzz_chunks /