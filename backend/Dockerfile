FROM rust:latest AS chef
ARG CHEF_TAG=0.1.71
RUN cargo install cargo-chef --locked --version "$CHEF_TAG" && rm -rf "$CARGO_HOME/registry/"
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path /tmp/recipe.json && rm -rf /app/* && mv /tmp/recipe.json /app

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --locked --recipe-path recipe.json --bin backend
COPY . .
RUN cargo build --release --frozen --bin backend

FROM debian:12-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends openssl \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists

COPY --from=builder /app/target/release/backend /usr/local/bin/app
COPY --from=builder /app/templates /templates
ENTRYPOINT ["/usr/local/bin/app"]
