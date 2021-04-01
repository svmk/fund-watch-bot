FROM ekidd/rust-musl-builder:nightly-2021-02-13 as builder

RUN USER=root cargo new --bin fund-watch-bot
WORKDIR ./fund-watch-bot
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/x86_64-unknown-linux-musl/release/deps/rust_docker_web*
RUN cargo build --release


FROM alpine:latest

ARG APP=/usr/src/app

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=fund-watch-bot

RUN addgroup -S $APP_USER \
    && adduser -S -g $APP_USER $APP_USER

RUN apk update \
    && apk add --no-cache ca-certificates tzdata \
    && rm -rf /var/cache/apk/*

COPY --from=builder /home/rust/src/fund-watch-bot/target/x86_64-unknown-linux-musl/release/fund-watch-bot ${APP}/fund-watch-bot

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./fund-watch-bot"]