FROM docker.io/library/rust:1-alpine AS build
WORKDIR /app
COPY . .
RUN apk add --no-cache musl-dev postgresql-dev
RUN cargo build --release

FROM docker.io/library/alpine:3.16.1
RUN apk add --no-cache ca-certificates
COPY --from=build /app/target/release/conduit /usr/bin/realworld
ENTRYPOINT ["/usr/bin/realworld"]
