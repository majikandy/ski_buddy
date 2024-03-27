.PHONY: all up down clean build

# Default target
all: up
	@echo "Opening web interface..."
	@sleep 5  # Wait for servers to start
	@open http://localhost:5173

# Start all services
up:
	@echo "Starting services..."
	@docker-compose up -d

# Stop all services
down:
	@echo "Stopping services..."
	@docker-compose down

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@docker-compose down -v
	@rm -f ski.db
	@rm -rf web/node_modules web/dist

# Build Docker images
build:
	@echo "Building Docker images..."
	@docker-compose build

# Format code
fmt:
	@echo "Formatting code..."
	@cargo fmt
	@cd web && npm run format

# Run tests
test:
	@echo "Running tests..."
	@cargo test
	@cd web && npm run test 

