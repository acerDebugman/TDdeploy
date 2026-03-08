# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

taosX is a zero-code data ingestion pipeline for TDengine, written in Rust. It provides offline data import/export, real-time replication, and external data source integration from various databases, message queues, and industrial protocols.

**Key Components:**
- `taosx`: Main data integration service (src/main.rs)
- `taos-explorer`: Web-based UI for database management (explorer/)
- `taosx-agent`: Agent for distributed deployments (taosx-agent/)
- `taosx-core`: Core library with shared functionality (taosx-core/)
- `crates/`: Modular source/sink implementations for different data sources

## Build System

The project uses `cargo-make` as the primary build orchestrator. All commands go through `make` or `cargo make`:

```bash
# Build commands
cargo make build-all              # Build taosx + taos-explorer
cargo make build-all-with-agent   # Build all including taosx-agent
cargo make build -p taosx         # Build specific package

# Build profiles
BUILD_PROFILE=release cargo make build-all      # Default
BUILD_PROFILE=production cargo make build-all   # Optimized for production

# External plugins (require Java/Go)
cargo make plugins                # Build InfluxDB, OpenTSDB, OPC plugins

# Skip UI build if needed
NO_BUILD_UI=true cargo make build-all
```

**Prerequisites:**
- Rust 1.90.0+ (edition 2024)
- Node.js v16 (for UI, managed via nvm)
- Java SDK 11+ with Maven (for InfluxDB/OpenTSDB plugins)
- Go 1.20+ (for OPC-UA/DA plugins)
- TDengine v3.0+ (required for most functionality)

## Testing

The project uses `cargo-nextest` for Rust tests and `pytest` for E2E tests.

```bash
# Quick pre-commit checks (no external dependencies)
cargo make pre-commit             # Runs fmt, clippy, core tests

# Run all tests
make test                         # All unit tests

# List all test cases
cargo nextest list

# Run specific test
cargo nextest run --workspace <test-name>

# Test specific data source
cargo make test-datasource-kafka
cargo make test-datasource-mysql
cargo make test-datasource-postgres

# E2E tests (requires Python poetry)
cd tests/e2e
poetry install
poetry run pytest -m sanity
poetry run pytest -sv opcua_test.py::test_sanity
```

**Test Organization:**
- `tests/integration/`: Rust integration tests organized by data source
- `tests/e2e/`: Python E2E scenario tests using pytest
- Tests use feature flags: `test-kafka`, `test-mysql`, `test-oracle`, `test-postgres`, `test-mssql`, `test-mongodb`, `test-opcua`, `test-opcda`, `test-pi`, `test-historian`
- At least 4 cores and 16GB RAM recommended for full test suite

## Code Quality

```bash
# Format code
cargo fmt --all

# Check formatting
cargo make fmt

# Run clippy
cargo make clippy

# Code coverage
cargo install cargo-llvm-cov
cargo llvm-cov --html --open nextest run --workspace
```

## Architecture

### Modular Source/Sink System

The project is organized around pluggable data sources and sinks in `crates/`:

**Sources:**
- Message queues: `source-{kafka,mqtt,pulsar,sparkplugb}`
- Relational databases: `source-{mysql,oracle,postgres,mssql}`
- NoSQL: `source-mongodb`
- Industrial protocols: `source-{opc,pi,historian,kinghistorian}`
- File formats: `source-{csv,parquet,orc}`
- Time-series: `source-{influxdb,opentsdb}`

**Sinks:**
- Output destinations: `sink-{kafka,mqtt,parquet}`
- TDengine TMQ consumers: `tmq-to-td`, `tmq-to-local`
- TDengine writers: `local-to-taos`, `legacy-to-taos`
- TDengine readers: `taos-to-local`

### Core Components

- **taosx-core**: Shared types, connectors, transformation engine
- **taosx-ipc**: Inter-process communication
- **taosx-metrics**: Metrics collection and monitoring
- **taosx-task**: Task management
- **ha-core**: High availability core
- **archive**: Data archival functionality
- **futures-ext**: Async utilities
- **taoslog**: Logging infrastructure

### Main Service Structure

- `src/main.rs`: Entry point with CLI parsing and service initialization
- `src/serve/`: HTTP API server, WebSocket handlers, task management
- `src/replica/`: Replication logic
- `src/privileges/`: Permission management

### Explorer (Web UI)

- `explorer/`: Full-stack web application
- `explorer/server/`: Rust backend (Actix-web)
- `explorer/src/`: Vue.js frontend
- Build: `cd explorer && yarn install && yarn build`
- Access at: http://localhost:6060

## Workspace Structure

This is a Cargo workspace with 50+ member crates. Key workspace dependencies:
- Arrow/Parquet: v56 (data processing)
- Tokio: v1.48 (async runtime)
- Actix-web: v4.9 (HTTP server)
- Tonic: v0.13 (gRPC)
- Memory allocator: `mimalloc` (default) or `jemallocator` (optional)
- TLS: `rustls` (default) or `native-tls` (optional)

## Running Services

```bash
# Local installation
cargo make install-locally

# Start services (after installation)
sudo systemctl start taosx
sudo systemctl start taos-explorer

# Or use helper script
./start_services.sh --agent_name=my_agent

# Without installation
./target/release/taosx --help
./target/release/taos-explorer --help
```

## Development Workflow

1. **Format first**: Run `cargo fmt --all` before committing
2. **Pre-commit checks**: Run `cargo make pre-commit` to validate changes
3. **Test incrementally**: Use specific test tasks rather than running all tests
4. **Check dependencies**: Ensure required data sources are running before testing
5. **Coverage**: Check coverage when adding new features

## Important Notes

- **Rust Edition**: 2024 edition, requires Rust 1.90.0+
- **TDengine Required**: Most functionality requires TDengine v3.0+ to be installed
- **Data Sources**: Tests requiring external data sources need those services running and configured in `tests/e2e/config/env.yaml`
- **Build Profile**: Default is `release`. Use `BUILD_PROFILE=dev` for debug builds
- **Memory**: Tests can be memory-intensive; 16GB RAM recommended
- **UI Build**: Explorer UI is built automatically before taosx build (can skip with `NO_BUILD_UI=true`)

## Packaging & Release

```bash
# Package for distribution
cd packaging
python3 release.py -o taosx        # taosx + explorer + plugins
python3 release.py -ba 1           # taosx-agent + plugins
```

## CI/CD

GitHub Actions workflows in `.github/workflows/`:
- `pr-ci.yaml`: PR validation (lint, format, core tests)
- `3.0-qa-ci.yaml`: Full test suite with coverage

## Documentation

- [Development Guide](docs/dev/README.md)
- [Test Quick Start](docs/dev/TEST_QUICKSTART.md)
- [Test Architecture](docs/dev/TEST_REFACTORING_PLAN.md)
- [Contributing Guidelines](CONTRIBUTING.md)
- [Coverage Usage](docs/dev/COVERAGE_USAGE.md)

## Code Style Guidelines

### Rust Code Style

1. **严格 Lint 规则** (Cargo.toml 中定义):
   ```toml
   [lints.rust]
   unsafe_code = "forbid"
   warnings = "deny"

   [lints.clippy]
   all = { level = "deny", priority = -1 }
   pedantic = { level = "warn", priority = -1 }
   nursery = { level = "warn", priority = -1 }
   unwrap_used = "deny"
   expect_used = "deny"
   panic = "deny"
   ```

2. **错误处理**:
   - 禁止使用裸 `unwrap()`, `expect()`, `panic!()`
   - 使用 `#[must_use]` 标记重要返回值

3. **文档注释**:
   - 所有 public API 必须有 `///` 文档注释
   - 复杂函数需要 `# Errors` 和 `# Panics` 说明
   - 使用 `//!` 为模块添加文档

4. **模块组织**:
   - 遵循 DDD 分层: domain -> application -> infrastructure -> interfaces
   - 每层通过 `mod.rs` 暴露公共接口
   - 依赖方向: 上层 -> 下层 (domain 是最底层)

5. **命名规范**:
   - 结构体/枚举: PascalCase
   - 函数/变量: snake_case
   - 常量: SCREAMING_SNAKE_CASE
   - 类型别名: PascalCase

### TypeScript/Vue Code Style
1. **严格 TypeScript 模式**
2. **Composition API**: 使用 `<script setup>` 语法
3. **类型定义**: 优先使用 `type` 而非 `interface`
4. **Props/Emit**: 必须显式定义类型
5. **运行时校验**: 使用 zod 进行数据校验

## 经验教训沉淀
每次遇到问题或者完成重要改动后，要在[PROGRESS.MD](./PROGRESS.MD)中记录：
- 遇到了什么问题
- 如何解决的
- 以后如何避免

