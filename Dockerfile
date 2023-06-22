FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application and reduce size
COPY . .
RUN cargo build --release --bin veteshnik && strip /app/target/release/veteshnik

# We do not need the Rust toolchain to run the binary!
FROM debian:buster-slim AS runtime
WORKDIR app
COPY --from=builder /app/target/release/veteshnik /usr/local/bin
EXPOSE 8000
ENTRYPOINT ["/usr/local/bin/veteshnik"]
