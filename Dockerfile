FROM ekidd/rust-musl-builder:nightly-2021-02-13 as builder

WORKDIR /home/rust/src
ADD ./src ./src
ADD ./Cargo.toml ./Cargo.toml
ADD ./Cargo.lock ./Cargo.lock
ARG TARGET_ARCHITECTURE=x86_64-unknown-linux-musl
RUN cargo build --target=$TARGET_ARCHITECTURE --release
RUN cp /home/rust/src/target/${TARGET_ARCHITECTURE}/release/fund-watch-bot /home/rust/src/target/fund-watch-bot

FROM alpine:latest

ARG APP=/usr/src/app

ENV TZ=Etc/UTC \
    APP_USER=fund-watch-bot

RUN addgroup -S $APP_USER \
    && adduser -S -g $APP_USER $APP_USER

RUN apk update \
    && apk add --no-cache ca-certificates tzdata esh bash \
    && rm -rf /var/cache/apk/*

COPY --from=builder /home/rust/src/target/fund-watch-bot ${APP}/fund-watch-bot

RUN mkdir -p ${APP}/bin && mv ${APP}/fund-watch-bot ${APP}/bin/fund-watch-bot
ADD ./docker ${APP}

RUN mkdir -p ${APP}/data && chown -R $APP_USER:$APP_USER ${APP}/data

RUN mkdir -p ${APP}/config && chown -R $APP_USER:$APP_USER ${APP}/config

USER $APP_USER
WORKDIR ${APP}

CMD ["/usr/src/app/start.sh"]