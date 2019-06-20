FROM openfaas/classic-watchdog:0.13.4 as watchdog

FROM rust:1.35-slim as builder

# Allows you to add additional packages via build-arg
ARG ADDITIONAL_PACKAGE

COPY --from=watchdog /fwatchdog /usr/bin/fwatchdog
RUN chmod +x /usr/bin/fwatchdog

WORKDIR /home/app

RUN apt-get update -qy \
    && apt-get install -qy ca-certificates ${ADDITIONAL_PACKAGE} --no-install-recommends

COPY function ./function
COPY main ./main

RUN cargo build --release --manifest-path ./main/Cargo.toml

FROM debian:stretch-slim

RUN apt-get update -qy \
    && apt-get install -qy ca-certificates

# Add non root user
RUN addgroup --system app && adduser --system --group app
RUN mkdir -p /home/app

WORKDIR /home/app

COPY --from=builder /usr/bin/fwatchdog                  .
COPY --from=builder /home/app/main/target/release/main  .

RUN chown -R app /home/app

USER app

ENV fprocess="./main"

EXPOSE 8080

HEALTHCHECK --interval=3s CMD [ -e /tmp/.lock ] || exit 1

CMD ["./fwatchdog"]
