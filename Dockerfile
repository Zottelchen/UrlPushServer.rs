FROM clux/muslrust:stable as builder

ARG TARGETPLATFORM

COPY . /app/
WORKDIR /app/

RUN --mount=type=cache,target=/usr/local/cargo/registry,id=${TARGETPLATFORM} --mount=type=cache,target=/root/target,id=${TARGETPLATFORM} \
    cargo build --release ; \
    chmod +x /app/target/x86_64-unknown-linux-musl/release/urlpushserver

FROM gcr.io/distroless/static
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/urlpushserver /
EXPOSE 8080
ENTRYPOINT ["/urlpushserver"]
