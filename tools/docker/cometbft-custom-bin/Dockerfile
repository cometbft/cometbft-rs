FROM alpine:3.16
LABEL maintainer="hello@informal.systems"

ENV CMTHOME=/cometbft
RUN apk --no-cache add jq bash && \
    wget https://github.com/freshautomations/sconfig/releases/download/v0.1.0/sconfig_linux_amd64 \
    -O /usr/bin/sconfig && \
    chmod 755 /usr/bin/sconfig && \
    addgroup cometbft && \
    adduser -S -G cometbft cometbft -h "$CMTHOME"
USER cometbft
WORKDIR $CMTHOME

EXPOSE 26656 26657 26660
STOPSIGNAL SIGTERM

ARG COMETBFT=cometbft
COPY $COMETBFT /usr/bin/cometbft

COPY entrypoint /usr/bin/entrypoint
ENTRYPOINT ["/usr/bin/entrypoint"]
CMD ["node"]
VOLUME [ "$CMTHOME" ]
