FROM rust:1.63.0-alpine3.16 as builder
RUN apk add musl-dev
ARG CARGO_TERM_COLOR=always
WORKDIR /usr/local/src/chrclone
COPY Cargo.toml Cargo.lock ./
COPY src ./src/
RUN cargo build --release

FROM docker.io/rclone/rclone:1.59.1

COPY ./docker-entrypoint.sh /docker-entrypoint.sh
COPY --from=builder /usr/local/src/chrclone/target/release/chrclone /usr/local/bin/chrclone

ARG RCLONE_CONFIG_BASE64
RUN echo "Writing rclone config from RCLONE_CONFIG_BASE64" \
    && [ -n "$RCLONE_CONFIG_BASE64" ] \
    && mkdir -vp /config/rclone \
    && echo "$RCLONE_CONFIG_BASE64" | base64 -d > /config/rclone/rclone.conf

ENTRYPOINT ["/docker-entrypoint.sh"]
CMD ["chrclone"]
