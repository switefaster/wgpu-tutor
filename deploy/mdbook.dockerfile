FROM rust:slim-buster AS builder
RUN apt-get update && \
    apt-get install --no-install-recommends -y \
    musl-tools patch
ENV ARC="x86_64-unknown-linux-musl"
RUN rustup target add "${ARC}"
RUN cargo install mdbook mdbook-katex --target "${ARC}"

FROM alpine:latest
SHELL ["/bin/ash", "-eo", "pipefail", "-c"]
COPY --from=builder /usr/local/cargo/bin/mdbook* /usr/local/bin/

WORKDIR /book
EXPOSE 3000 3001
COPY . .

CMD ["mdbook", "serve", "-n", "0.0.0.0"]
