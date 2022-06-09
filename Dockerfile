# Using the `rust-musl-builder` as base image, instead of 
# the official Rust toolchain
FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef

FROM chef AS fleet
USER root
RUN git clone https://github.com/dimensionhq/fleet
RUN cd fleet
RUN cargo install --path .
WORKDIR /pinned


FROM fleet AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM fleet AS builder
COPY --from=planner /pinned/recipe.json recipe.json
# Notice that we are specifying the --target flag!
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN fleet build --release --target x86_64-unknown-linux-musl --bin pinned

FROM alpine AS runtime
RUN apk add libressl-dev
RUN addgroup -S myuser && adduser -S myuser -G myuser
COPY --from=builder /pinned/target/x86_64-unknown-linux-musl/release/pinned /usr/local/bin/
USER myuser
CMD ["/usr/local/bin/pinned"]