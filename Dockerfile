# Build stage
FROM rust:1.69-buster as builder

WORKDIR /app

# pass env vars
ENV DATABASE_URL=$DATABASE_URL

COPY . . 

RUN cargo build --release

# Production stage
FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/car_dealership .
RUN apt-get update
RUN apt-get upgrade -y
# RUN apt install -y openssl
RUN apt install -y gnutls-bin
# RSA Encryption not supported - caching_sha2_password plugin was built with GnuTLS support
# RUN apt install -y default-libmysqlclient-dev
# RUN apt install -y libmariadbclient-dev
RUN apt install -y libmariadb3

CMD ["./car_dealership"]
