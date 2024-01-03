FROM rust:1.69 AS builder
WORKDIR /usr/src/class_bot
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/class_bot /usr/local/bin/
CMD ["class_bot"]