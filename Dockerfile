FROM rust:1.85 AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release --locked

FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/nova-gateway-telegram-bot /app/nova-gateway-telegram-bot
USER nonroot
ENV PORT=8080
EXPOSE 8080
ENTRYPOINT ["/app/nova-gateway-telegram-bot"]
