FROM alpine:3.21
RUN apk add --no-cache --update tini
COPY bandwagon-exporter /usr/bin/bandwagon-exporter
EXPOSE 9103/tcp
ENTRYPOINT ["/sbin/tini", "--", "/usr/bin/bandwagon-exporter"]
CMD [ "--help" ]
