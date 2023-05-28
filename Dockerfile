FROM rust:latest as builder

COPY . /app/
WORKDIR /app/

RUN cargo build --release

FROM scratch
COPY --from=builder /app/target/release/url_push_server /
EXPOSE 8080
CMD ["/url_push_server"]