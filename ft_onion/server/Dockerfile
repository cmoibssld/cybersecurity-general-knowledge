FROM alpine:latest

RUN <<EOF
apk update
apk add --no-cache \
  nginx
EOF

COPY nginx.conf /etc/nginx/nginx.conf

EXPOSE 80

CMD ["nginx"]
