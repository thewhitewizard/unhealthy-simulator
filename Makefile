APP_NAME = unhealthy-simulator
DOCKER_IMAGE = $(APP_NAME):latest

all: build

build:
	cargo build --release

clean:
	cargo clean

image: build
	docker build -t $(DOCKER_IMAGE) .

up:
	docker compose up -d

down:
	docker compose down

ps:
	docker compose ps

simulate:
	curl -X POST http://localhost:3000/simulate
