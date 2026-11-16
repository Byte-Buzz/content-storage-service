FROM rust:1.95-slim-trixie AS build
WORKDIR /src

RUN apt-get update \
    && apt-get install -y --no-install-recommends protobuf-compiler libprotobuf-dev \
    && rm -rf /var/lib/apt/lists/*

ENV SQLX_OFFLINE=true

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/src/target \
    cargo build --release --locked --workspace \
    && mkdir /out && cp target/release/css-api target/release/css-migrate /out/

FROM gcr.io/distroless/cc-debian13:nonroot AS api
WORKDIR /app

COPY --from=build /out/ /app/

EXPOSE 50051 8080
ENTRYPOINT ["/app/css-api"]
