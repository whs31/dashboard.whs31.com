[working-directory("web")]
build-frontend:
	npm install
	npm run build

build-backend:
	cargo build --release

build: build-frontend build-backend

dev: build-frontend build-backend
	cargo run --release