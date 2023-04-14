FROM rust:1-alpine3.16 as builder
RUN apk add --no-cache cargo
ENV HOME=/root
WORKDIR /app/
COPY . /app/
RUN cargo build --release --target=x86_64-unknown-linux-musl --color never
RUN ls /app/target/x86_64-unknown-linux-musl/release/

FROM alpine:3.16
LABEL org.opencontainers.image.source="https://github.com/ngerakines/pr-has-issues-action"
LABEL org.opencontainers.image.description="A GitHub action that checks for issues references in pull requests."
LABEL org.opencontainers.image.authors="Nick Gerakines <nick.gerakines@gmail.com>"
LABEL org.opencontainers.image.licenses="MIT"
LABEL "com.github.actions.icon"="edit"
LABEL "com.github.actions.color"="red"
RUN apk add --no-cache ca-certificates
ENV RUST_LOG="warning"
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/pr-has-issues-action /usr/local/bin/pr-has-issues-action
ENTRYPOINT ["/usr/local/bin/pr-has-issues-action"]
