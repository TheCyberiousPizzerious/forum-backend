FROM rust:latest as builder

LABEL maintaner="whateva@blah.nah"

WORKDIR forum-backend

ADD ./ .

RUN cargo build --release

FROM debian:bookworm

COPY --from=builder /forum-backend/target/release/forum-backend /usr/src

WORKDIR /usr/src

CMD "./forum-backend"
# Alle handlingene som skjer her skjer relativt til hvor den bygges fra.
# Så om du bygger filen med `podman compose up` må alle handlingene skje relativt til docker compose filen