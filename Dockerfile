FROM nginx:latest
COPY blog .
COPY image-service .
COPY dist/ /etc/nginx/html/
COPY nginx/ /etc/nginx/
RUN apt-get update && apt-get -qq install libpq-dev ca-certificates

CMD envsubst '${PORT}' < /etc/nginx/app.template > /etc/nginx/conf.d/default.conf && \
    nginx -g 'daemon on;' && \
    RUST_LOG=info ./image-service & \
    ROCKET_DATABASES='{blog={url='$DATABASE_URL'}}' ./blog
