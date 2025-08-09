# Rust Backend Framework

A production-ready, asynchronous Rust backend scaffold built with `axum` and `tokio`, including pre-configured connectors for MySQL, MongoDB, Redis, RabbitMQ, and Kafka.

## 🚀 Features

- **Fully async** with `tokio` runtime
- **HTTP API** using `axum` framework
- **Structured JSON logging** with `tracing`
- **Configuration management** via `config` crate + environment variables
- **Multiple database connectors**:
  - MySQL (`sqlx`)
  - MongoDB (`mongodb`)
  - Redis (`redis`)
  - RabbitMQ (`lapin`)
  - Kafka (`rdkafka`)
- **API versioning** and health check endpoints
- **Alpine-based multi-stage Dockerfile** for small production images
- **Docker Compose** setup for local development
- **Comprehensive testing** with integration tests

## 📁 Project Structure

```text
.
├── config/
│   ├── default.toml           # Default configuration
│   └── production.toml        # Production overrides
├── data/                      # Docker volumes (gitignored)
├── src/
│   ├── api/
│   │   ├── health_check.rs    # Health check endpoint
│   │   ├── mod.rs            # API router setup
│   │   └── v1/               # API version 1
│   │       ├── mod.rs
│   │       └── user_handler.rs # User CRUD operations
│   ├── database/
│   │   ├── mod.rs
│   │   ├── mongo_db.rs       # MongoDB connection
│   │   └── my_sql.rs         # MySQL connection
│   ├── messaging/
│   │   ├── mod.rs
│   │   ├── kafka.rs          # Kafka producer setup
│   │   └── rabbit_mq.rs      # RabbitMQ connection
│   ├── models/
│   │   ├── mod.rs
│   │   └── user.rs           # User data models
│   ├── services/             # Business logic layer
│   │   └── mod.rs
│   ├── app_state.rs          # Shared application state
│   ├── config.rs             # Configuration loading
│   ├── error.rs              # Error handling
│   ├── main.rs               # Application entry point
│   └── telemetry.rs          # Logging setup
├── tests/
│   └── health_check.rs       # Integration tests
├── Cargo.toml                # Rust dependencies
├── Dockerfile                # Multi-stage build
├── docker-compose.yml        # Local development stack
└── README.md                 # This file
```

## 🛠 Prerequisites

- **Docker** and **Docker Compose** installed
- **Rust** (if developing locally) - nightly toolchain recommended
- **Git** for version control

### Installing Rust (if not using Docker)

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install nightly toolchain (recommended for this project)
rustup install nightly
rustup default nightly

# Install required tools
brew install cmake  # macOS
# or
apt-get install cmake  # Linux
```

## 🚀 Quick Start

### 1. Clone and Setup

```bash
git clone <your-repo-url>
cd rust-backend

# Copy environment template
cp .env.example .env
# Edit .env if needed (optional for local development)
```

### 2. Run with Docker Compose (Recommended)

```bash
# Start all services (databases + application)
docker-compose up -d

# View logs
docker-compose logs -f app

# Check service status
docker-compose ps
```

The application will be available at `http://localhost:8080`

### 3. Alternative: Step-by-step startup

If you encounter issues, start services individually:

```bash
# 1. Start dependencies first
docker-compose up -d mysql mongo redis rabbitmq kafka

# 2. Wait for services to be ready (10-15 seconds)
sleep 15

# 3. Check all services are running
docker-compose ps

# 4. Start the application
docker-compose up app
```

## 🔧 Development Setup

### Local Development (without Docker)

```bash
# Install dependencies and run locally
cargo run

# Run tests
cargo test

# Run with specific log level
RUST_LOG=debug cargo run

# Format code
cargo fmt

# Run linter
cargo clippy
```

### Building Docker Image Manually

```bash
# Build with host network (avoids APK fetch issues)
docker build --network=host -t rust-backend:alpine .

# Run container directly
docker run --rm -p 8080:8080 \
  --env-file .env \
  --name rust-backend \
  rust-backend:alpine
```

## 📡 API Endpoints

### Health Check
```bash
curl http://localhost:8080/health
```

### User Management
```bash
# Create a user
curl -X POST http://localhost:8080/api/v1/users \
  -H 'Content-Type: application/json' \
  -d '{"email":"user@example.com","name":"Alice"}'

# List users
curl http://localhost:8080/api/v1/users
```

## ⚙️ Configuration

Configuration is loaded from multiple sources in order of precedence:

1. Environment variables (highest priority)
2. `.env` file
3. `config/default.toml` (lowest priority)

### Key Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `APP__SERVER__HOST` | Server bind address | `0.0.0.0` |
| `APP__SERVER__PORT` | Server port | `8080` |
| `APP__MYSQL__URI` | MySQL connection string | `mysql://root:password@mysql:3306/app` |
| `APP__MONGODB__URI` | MongoDB connection string | `mongodb://mongo:27017` |
| `APP__REDIS__URI` | Redis connection string | `redis://redis:6379` |
| `APP__RABBITMQ__URI` | RabbitMQ connection string | `amqp://guest:guest@rabbitmq:5672/%2f` |
| `APP__KAFKA__BROKERS` | Kafka broker list | `kafka:9092` |
| `RUST_LOG` | Log level | `info` |

### Configuration File Example

```toml
# config/default.toml
[server]
host = "0.0.0.0"
port = 8080

[mysql]
uri = "mysql://root:password@localhost:3306/app"

[mongodb]
uri = "mongodb://localhost:27017"

[redis]
uri = "redis://localhost:6379"

[rabbitmq]
uri = "amqp://guest:guest@localhost:5672/%2f"

[kafka]
brokers = "localhost:9092"
```

## 🗄 Database Setup

### MySQL Schema

```sql
-- Connect to MySQL and create the required table
CREATE DATABASE IF NOT EXISTS app;
USE app;

CREATE TABLE IF NOT EXISTS users (
  id CHAR(36) PRIMARY KEY,
  email VARCHAR(255) NOT NULL UNIQUE,
  name VARCHAR(100) NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
```

### MongoDB Collections

The application will automatically create collections as needed. No manual setup required.

## 🐛 Troubleshooting

### Common Issues

#### 1. Container Exits Immediately
**Symptoms**: Container shows "Exited (0)" status, no logs
**Cause**: Dependencies (MySQL, MongoDB, etc.) not ready
**Solution**:
```bash
# Check service status
docker-compose ps

# Start dependencies first
docker-compose up -d mysql mongo redis rabbitmq kafka

# Wait and check logs
sleep 15
docker-compose logs mysql mongo redis rabbitmq kafka

# Then start app
docker-compose up app
```

#### 2. Build Errors - cmake not found
**Symptoms**: `cmake not installed?` during build
**Solution**:
```bash
# macOS
brew install cmake

# Linux
apt-get update && apt-get install cmake

# Or use Docker build with dependencies
docker build --network=host -t rust-backend:alpine .
```

#### 3. Port Already in Use
**Symptoms**: `bind: address already in use`
**Solution**:
```bash
# Check what's using port 8080
lsof -i :8080

# Kill the process or change port
export APP__SERVER__PORT=3000
docker-compose up
```

#### 4. APK Index Fetch Errors
**Symptoms**: `IO ERROR` during Docker build
**Solution**:
```bash
# Build with host network
docker build --network=host -t rust-backend:alpine .
```

#### 5. Database Connection Errors
**Symptoms**: Connection refused errors in logs
**Solution**:
```bash
# Check if databases are healthy
docker-compose ps
docker-compose logs mysql

# Restart unhealthy services
docker-compose restart mysql mongo redis rabbitmq kafka
```

#### 6. Apple Silicon (M1/M2) Issues
**Symptoms**: Architecture mismatch errors
**Solution**: The Dockerfile automatically handles ARM64 architecture. Ensure you're using recent Docker Desktop.

### Debug Commands

```bash
# View all container logs
docker-compose logs

# View specific service logs
docker-compose logs app
docker-compose logs mysql

# Follow logs in real-time
docker-compose logs -f app

# Check container resource usage
docker stats

# Access running container
docker-compose exec app sh

# Check network connectivity between containers
docker-compose exec app ping mysql
```

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test health_check_works

# Run tests in Docker
docker-compose run --rm app cargo test
```

### Integration Tests

The project includes integration tests that:
- Test HTTP endpoints
- Verify database connections
- Check service health

## 🔄 CI/CD

The project is ready for CI/CD with:
- GitHub Actions workflows (if `.github/workflows/` exists)
- Docker multi-stage builds for optimized production images
- Health check endpoints for monitoring

### Example GitHub Actions workflow:

```yaml
name: CI
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: nightly
      - run: cargo test
      - run: cargo clippy
      - run: cargo fmt --check
```

## 🚀 Production Deployment

### Environment Variables for Production

```bash
export APP__SERVER__HOST=0.0.0.0
export APP__SERVER__PORT=8080
export APP__MYSQL__URI=mysql://user:pass@prod-mysql:3306/app
export APP__MONGODB__URI=mongodb://prod-mongo:27017
export RUST_LOG=info
```

### Docker Production Build

```bash
# Build production image
docker build -t rust-backend:prod .

# Run in production
docker run -d \
  --name rust-backend-prod \
  -p 8080:8080 \
  --env-file .env.production \
  --restart unless-stopped \
  rust-backend:prod
```

## 📚 Architecture Notes

### Design Principles
- **Async-first**: All I/O operations are non-blocking
- **Modular**: Clear separation of concerns
- **Configurable**: Environment-based configuration
- **Observable**: Structured logging with tracing
- **Resilient**: Graceful error handling

### Key Dependencies
- `axum` - Web framework
- `tokio` - Async runtime
- `sqlx` - SQL toolkit
- `mongodb` - MongoDB driver
- `redis` - Redis client
- `lapin` - RabbitMQ client
- `rdkafka` - Kafka client
- `tracing` - Logging framework
- `config` - Configuration management

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Run `cargo fmt` and `cargo clippy`
6. Submit a pull request

## 📄 License

[Add your license here]

## 📞 Support

For issues and questions:
1. Check the troubleshooting section above
2. Review container logs: `docker-compose logs app`
3. Open an issue with detailed error information