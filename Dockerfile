FROM lukemathwalker/cargo-chef:latest-rust-1.63.0 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder

COPY  --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo build --release --bin staticsitecompanion

FROM debian:bullseye-slim AS runtime

# Install SSL
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/staticsitecompanion staticsitecompanion
COPY config.json .
CMD ["./staticsitecompanion"]