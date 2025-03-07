FROM rust:1.85.0-alpine3.21 AS build
RUN apk add --no-cache build-base openssl-dev
ENV RUSTFLAGS="-Ctarget-feature=-crt-static"
RUN cargo install --locked cargo-auditable@0.6.6
WORKDIR /app
ADD . .
RUN cargo auditable build --release

FROM alpine:3.21.3
RUN apk add --no-cache libgcc
RUN adduser -D app
USER app
COPY --from=build /app/target/release/nvda_zip /usr/local/bin/
CMD ["nvda_zip"]
