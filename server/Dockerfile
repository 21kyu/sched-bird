FROM rust:latest as builder

RUN rustup target add aarch64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev clang llvm pkg-config libssl-dev
ENV CC_aarch64_unknown_linux_musl=clang
ENV AR_aarch64_unknown_linux_musl=llvm-ar
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-Clink-self-contained=yes -Clinker=rust-lld"

WORKDIR /app

RUN cargo new sched-bird
RUN mkdir -p sched-bird/src/bin/hydrate && mkdir -p sched-bird/src/bin/app
RUN cp sched-bird/src/main.rs sched-bird/src/bin/hydrate/ && cp sched-bird/src/main.rs sched-bird/src/bin/app/
COPY Cargo.toml Cargo.lock ./sched-bird/

WORKDIR /app/sched-bird

RUN cargo build --features=ssr --bin app --release --target aarch64-unknown-linux-musl
COPY src ./src
COPY dist ./dist
RUN touch src/main.rs
RUN cargo build --features=ssr --bin app --release --target aarch64-unknown-linux-musl

# CMD ["/app/sched-bird/target/aarch64-unknown-linux-musl/release/sched-bird"]

FROM scratch

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /app/sched-bird/target/aarch64-unknown-linux-musl/release/app /sched-bird
COPY --from=builder /app/sched-bird/dist /dist

CMD ["/sched-bird"]
