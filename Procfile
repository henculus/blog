release: ./target/release/diesel migration run
web: ROCKET_PORT=$PORT ROCKET_DATABASES={blog={url=$DATABASE_URL}} ROCKET_ENV=prod ./target/release/blog