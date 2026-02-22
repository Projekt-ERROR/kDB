FROM rust:1.93.1-alpine AS builder

RUN apk add --no-cache musl-dev pkgconfig

WORKDIR /app

ENV SQLX_OFFLINE=true

COPY Cargo.toml ./
COPY Cargo.lock ./
COPY .sqlx ./.sqlx
COPY src ./src

RUN cargo build --release

FROM alpine:3.19

RUN apk add --no-cache ca-certificates

WORKDIR /app

COPY --from=builder /app/target/release/kdb-backend .

EXPOSE 5000

CMD ["./kdb-backend"]
