FROM alpine:3.16
LABEL maintainer="hello@informal.systems"
ARG CMTVERSION

ENV CMTHOME=/cometbft
RUN apk --no-cache add jq bash && \
    wget https://github.com/freshautomations/sconfig/releases/download/v0.1.0/sconfig_linux_amd64 \
    -O /usr/bin/sconfig && \
    chmod 755 /usr/bin/sconfig && \
    cd /tmp && \
    wget https://github.com/cometbft/cometbft/releases/download/v${CMTVERSION}/cometbft_${CMTVERSION}_linux_amd64.tar.gz \
    -O ./cometbft.tar.gz && \
    tar xf cometbft.tar.gz && \
    mv cometbft /usr/bin/cometbft && \
    chmod 755 /usr/bin/cometbft && \
    rm cometbft.tar.gz && \
    addgroup cometbft && \
    adduser -S -G cometbft cometbft -h "$CMTHOME"

USER cometbft
WORKDIR $CMTHOME

EXPOSE 26656 26657 26660
STOPSIGNAL SIGTERM

COPY entrypoint /usr/bin/entrypoint
ENTRYPOINT ["/usr/bin/entrypoint"]
CMD ["node"]
VOLUME [ "$CMTHOME" ]
