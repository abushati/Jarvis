FROM rust:1.64.0

WORKDIR /usr/src/web
COPY . .

RUN cargo install --path .

CMD ['web']