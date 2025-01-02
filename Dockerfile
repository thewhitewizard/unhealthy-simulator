FROM debian:bullseye-slim
RUN apt update && apt -y install curl
WORKDIR /app
COPY target/release/unhealthy-simulator /app/unhealthy-simulator
EXPOSE 3000
CMD ["/app/unhealthy-simulator"]