FROM rustlang/rust:nightly AS backendBuilder
COPY backend .
RUN cargo build --release

FROM node:latest AS frontendBuilder
COPY frontend .
RUN npm install && npm run build

FROM nginx:latest
COPY --from=backendBuilder target/release/blog .
COPY --from=frontendBuilder dist/ /etc/nginx/html/
COPY nginx/ /etc/nginx/
RUN apt-get update && apt-get -qq install libpq-dev

CMD envsubst '${PORT}' < /etc/nginx/app.template > /etc/nginx/conf.d/default.conf && \
    nginx -g 'daemon on;' && \
    ROCKET_DATABASES='{blog={url='$DATABASE_URL'}}' ./blog
