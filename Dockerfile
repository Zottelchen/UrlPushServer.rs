FROM rust:latest as builder

ARG TARGETPLATFORM

COPY . /app/
WORKDIR /app/

RUN --mount=type=cache,target=/usr/local/cargo/registry,id=${TARGETPLATFORM} --mount=type=cache,target=/root/target,id=${TARGETPLATFORM} \
    cargo build --release ; \
    chmod +x /app/target/release/urlpushserver

FROM scratch
COPY --from=builder /app/target/release/urlpushserver /
EXPOSE 8080
ENTRYPOINT ["/urlpushserver"]
