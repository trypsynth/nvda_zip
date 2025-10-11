FROM docker.io/library/alpine:3.22.2 AS runtime-build
RUN mkdir -p /runtime/etc/apk && \
    cp -a /etc/apk/repositories /etc/apk/keys /runtime/etc/apk && \
    apk --root /runtime --initdb --no-cache add alpine-baselayout-data alpine-release musl libgcc

FROM docker.io/library/rust:1.90.0-alpine3.22 AS build
RUN apk add --no-cache build-base
ENV RUSTFLAGS="-Ctarget-feature=-crt-static"
RUN cargo install --locked cargo-auditable@0.6.6
WORKDIR /app
ADD . .
RUN cargo auditable build --release

FROM scratch
COPY --from=runtime-build /runtime /
COPY --from=build /app/target/release/nvda_zip /usr/local/bin/
USER nobody
CMD ["nvda_zip"]
