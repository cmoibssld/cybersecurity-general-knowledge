FROM alpine:latest

RUN <<EOF
apk update
apk add --no-cache \
  nginx
EOF

COPY nginx.conf /etc/nginx/nginx.conf
COPY html/index.html /etc/pages/index.html

EXPOSE 80

CMD ["nginx"]
