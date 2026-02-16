FROM rust:alpine AS build

WORKDIR /pipeline

COPY src src
COPY Cargo.* .

RUN cargo build --release

FROM alpine:latest
WORKDIR /app

COPY --from=build /pipeline/target/release/pipeline /app/pipeline

EXPOSE 80
ENTRYPOINT [ "/app/pipeline" ]