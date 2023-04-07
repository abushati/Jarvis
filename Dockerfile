FROM rust:1-alpine3.17

ENV RUSTFLAGS="-C target-feature=-crt-static"

RUN apk add --no-cache musl-dev
RUN apk add --update openssl openssl-dev


WORKDIR /app
COPY . /app

RUN cargo build 
RUN strip target/debug/webserver
RUN strip target/debug/server
RUN strip target/debug/diskmanager

FROM alpine:3.17

RUN apk add --no-cache libgcc

COPY --from=0 /app/target/debug/webserver .
COPY --from=0 /app/target/debug/server .
COPY --from=0 /app/target/debug/diskmanager .
ENV redis=localhost
# EXPOSE 8080

# ENTRYPOINT ["./server"]
# CMD ['ls']
