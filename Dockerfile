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
RUN apt install -y openssl && apt install -y default-libmysqlclient-dev
# && apt install -y libmariadb3
# && apt install -y libmariadbclient-dev

CMD ["./car_dealership"]
