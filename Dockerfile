FROM lukemathwalker/cargo-chef:latest-rust-slim-buster AS chef
WORKDIR /pinned


FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder
COPY --from=planner /pinned/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin pinned

FROM alpine:latest AS runtime
WORKDIR /pinned
COPY --from=builder /pinned/target/release/pinned /usr/local/bin/pinned
ENTRYPOINT ["/usr/local/bin/pinned"]
