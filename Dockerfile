FROM rust:1.78.0-slim as base
WORKDIR /usr/src/journalia-backend
RUN apt-get update -y && apt-get install -y \
    libpq-dev \
    netcat-traditional \
    pkg-config \
    libssl-dev
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch
RUN cargo install cargo-chef

FROM base as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM base
COPY --from=planner /usr/src/journalia-backend/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json
COPY . .
RUN cargo build
