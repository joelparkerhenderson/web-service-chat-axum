FROM alpine
RUN apk --update add openssl
COPY  target/x86_64-unknown-linux-musl/release/web-service-chat-axum /opt/web-service-chat-axum
WORKDIR /opt
CMD ["/opt/web-service-chat-axum"]
EXPOSE 8080
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:8080/ || exit 1
LABEL org.opencontainers.image.authors="Joel Parker Henderson <joel@joelparkerhenderson.com>"
LABEL org.opencontainers.image.description="Web service chat axum example"
LABEL org.opencontainers.image.documentation="https://github.com/joelparkrhenderson/web-service-chat-axum"
LABEL org.opencontainers.image.licenses="MIT"
LABEL org.opencontainers.image.source="https://github.com/joelparkrhenderson/web-service-chat-axum"
LABEL org.opencontainers.image.title="web-service-chat-axum"
LABEL org.opencontainers.image.url="https://github.com/joelparkrhenderson/web-service-chat-axum"
LABEL org.opencontainers.image.version="1.4.0"
