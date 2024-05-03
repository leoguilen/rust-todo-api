FROM rust:slim-bullseye AS build

WORKDIR /build

COPY Cargo.lock Cargo.toml ./
COPY src src

RUN cargo build --bins --locked --release

FROM gcr.io/distroless/cc-debian12 AS final
COPY --from=build /build/target/release/todo-api /
ENV HTTP_PORT=80
ENV RUST_LOG=info
EXPOSE 80
CMD ["/todo-api"]