FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /boytacean
WORKDIR /boytacean/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /boytacean/fuzz/target/x86_64-unknown-linux-gnu/release/fuzz_boytacean /