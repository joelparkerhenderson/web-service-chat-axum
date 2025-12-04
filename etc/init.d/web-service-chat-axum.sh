#!/bin/sh

### BEGIN INIT INFO
# Provides:          web-service-chat-axum
# Required-Start:    $local_fs $network
# Required-Stop:     $local_fs
# Default-Start:     2 3 4 5
# Default-Stop:      0 1 6
# Short-Description: Web service chat axum
# Description:       Web service example that provides a chat function that is implemented via Rust Axum
### END INIT INFO

case "$1" in
  start)
    echo "Start web-service-chat-axum"
    PORT=10005 /opt/web-service-chat-axum/target/release/web-service-chat-axum
    ;;
  stop)
    echo "Stop web-service-chat-axum"
    pgrep '[w]eb-service-chat-axum' | xargs kill
    ;;
  *)
    echo "Usage: /etc/init.d/web-service-chat-axum {start|stop}"
    exit 1
    ;;
esac

exit 0
