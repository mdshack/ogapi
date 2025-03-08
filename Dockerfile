FROM rust:1-alpine3.21 AS build

WORKDIR /app

RUN apk add libc-dev libressl-dev

COPY . .

RUN cargo build --release 


FROM alpine:3.21 AS runtime

WORKDIR /app

COPY --from=build /app/target/release/ogapi /app/ogapi

EXPOSE 3000

CMD ["/app/ogapi"]