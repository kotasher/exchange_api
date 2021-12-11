FROM alpine:latest
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_ENV="staging"
ENV ROCKET_PORT=8000
EXPOSE 8000

WORKDIR /
COPY target/x86_64-unknown-linux-musl/release/kquotes /usr/bin/kquotes
ENTRYPOINT ["kquotes"]
