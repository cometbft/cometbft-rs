FROM alpine:3.16
LABEL maintainer="hello@informal.systems"
ARG CMTVERSION

ENV CMTHOME=/cometbft
#GLIBC for Alpine from: https://github.com/sgerrand/alpine-pkg-glibc
RUN wget https://alpine-pkgs.sgerrand.com/sgerrand.rsa.pub -O /etc/apk/keys/sgerrand.rsa.pub && \
     wget https://github.com/sgerrand/alpine-pkg-glibc/releases/download/2.32-r0/glibc-2.32-r0.apk \
     https://github.com/sgerrand/alpine-pkg-glibc/releases/download/2.32-r0/glibc-bin-2.32-r0.apk \
     https://github.com/sgerrand/alpine-pkg-glibc/releases/download/2.32-r0/glibc-i18n-2.32-r0.apk && \
     apk add --no-cache glibc-2.32-r0.apk glibc-bin-2.32-r0.apk glibc-i18n-2.32-r0.apk && \
     rm glibc-2.32-r0.apk glibc-bin-2.32-r0.apk glibc-i18n-2.32-r0.apk && \
     /usr/glibc-compat/bin/localedef -i en_US -f UTF-8 en_US.UTF-8 && \
     apk --no-cache add jq bash file && \
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

EXPOSE 26656 26657 26658 26660
STOPSIGNAL SIGTERM

COPY entrypoint /usr/bin/entrypoint
ENTRYPOINT ["/usr/bin/entrypoint"]
VOLUME [ "$CMTHOME", "/abci" ]
