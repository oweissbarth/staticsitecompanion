FROM rust:latest as build

WORKDIR /usr/src/staticsitecompanion
COPY . .

RUN cargo install --path .

FROM debian:bullseye-slim AS runtime

# Install SSL
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/staticsitecompanion
COPY --from=build /usr/src/staticsitecompanion/target/release/staticsitecompanion staticsitecompanion

CMD ["staticsitecompanion"]