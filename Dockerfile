FROM rust:latest as builder

COPY . /app/
WORKDIR /app/

RUN cargo build --release ; chmod +x /app/target/release/urlpushserver

FROM scratch
COPY --from=builder /app/target/release/urlpushserver /
EXPOSE 8080
ENTRYPOINT ["/urlpushserver"]