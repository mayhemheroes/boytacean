FROM rustlang/rust:nightly as builder

RUN apt update && apt upgrade -y && \
    apt install -y clang-11 llvm-11-tools && \
    ln -s /usr/bin/llvm-config-11 /usr/bin/llvm-config

RUN rustup component add rust-src
RUN cargo install -f cargo-fuzz

ADD . /boytacean
WORKDIR /boytacean
RUN RUST_BACKTRACE=1 cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /boytacean/fuzz/target/x86_64-unknown-linux-gnu/release/fuzz_boytacean /