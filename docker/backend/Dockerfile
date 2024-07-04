from rust:1.79.0 as builder

WORKDIR /app

RUN apt update && apt install lld clang -y

COPY ../../backend/ .
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM rust:1.79.0 as runtime

WORKDIR /app

COPY --from=builder /app/target/release/carbonite carbonite

COPY ../../backend/config/ config
ENV APP_ENVIRONMENT production

ENTRYPOINT [".carbonite"]
