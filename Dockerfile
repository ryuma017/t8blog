FROM lukemathwalker/cargo-chef:latest-rust-1.62.0 as chef
WORKDIR /app
RUN apt update && apt install lld clang -y

# Planner stage
FROM chef as planner
COPY . .
## Compute a lock-like file for the project
RUN cargo chef prepare --recipe-path recipe.json

# Builder stage
FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
## Build the project dependencies
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE true
## Build the project
RUN cargo build --release --bin t8blog

# Runtime stage
FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/list/*
COPY --from=builder /app/target/release/t8blog t8blog
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT [ "./t8blog" ]
