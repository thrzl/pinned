# Using the `rust-musl-builder` as base image, instead of 
# the official Rust toolchain
FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /pinned

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /pinned/recipe.json recipe.json
# Notice that we are specifying the --target flag!
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin pinned

FROM alpine AS runtime
RUN apk add libressl-dev
RUN addgroup -S myuser && adduser -S myuser -G myuser
COPY --from=builder /pinned/target/x86_64-unknown-linux-musl/release/pinned /usr/local/bin/
USER myuser
CMD ["/usr/local/bin/pinned"]