FROM rustlang/rust:nightly AS backendBuilder
COPY backend backend
COPY image-service image-service
RUN cd backend && cargo build --release && cd ..
RUN cd image-service && cargo build --release && cd ..

FROM node:latest AS frontendBuilder
COPY frontend .
RUN npm install && npm run build

FROM nginx:latest
COPY --from=backendBuilder /backend/target/release/blog .
COPY --from=backendBuilder /image-service/target/release/image-service .
COPY --from=frontendBuilder dist/ /etc/nginx/html/
COPY nginx/ /etc/nginx/
RUN apt-get update && apt-get -qq install libpq-dev ca-certificates

CMD envsubst '${PORT}' < /etc/nginx/app.template > /etc/nginx/conf.d/default.conf && \
    nginx -g 'daemon on;' && \
    RUST_LOG=info ./image-service & \
    ROCKET_DATABASES='{blog={url='$DATABASE_URL'}}' ./blog
