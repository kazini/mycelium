# Digital Mycelium Network: Development Environment and Tooling

## Overview

This document defines the development environment, tooling, and processes for building the Digital Mycelium Network. The environment is designed to support the multi-crate Rust workspace architecture while providing comprehensive testing, quality assurance, and development productivity tools.

## Project Structure

### Workspace Organization

```
mycelium-network/
├── Cargo.toml                 # Workspace root configuration
├── Cargo.lock                 # Dependency lock file
├── README.md                  # Project overview and quick start
├── LICENSE                    # Project license
├── .gitignore                 # Git ignore patterns
├── .github/                   # GitHub workflows and templates
│   ├── workflows/
│   │   ├── ci.yml            # Continuous integration
│   │   ├── security.yml      # Security audits
│   │   └── release.yml       # Release automation
│   └── ISSUE_TEMPLATE/       # Issue templates
├── docs/                      # Documentation
│   ├── architecture/         # Architecture documentation
│   ├── api/                  # API documentation
│   ├── guides/               # User and developer guides
│   └── rfcs/                 # Request for Comments
├── crates/                    # Core library crates
│   ├── mycelium-core/        # Minimal core components
│   ├── mycelium-spores/      # Spore discovery system
│   ├── mycelium-consensus/   # BFT consensus implementation
│   ├── mycelium-storage/     # Distributed storage integration
│   ├── mycelium-networking/  # Multi-homing and protocols
│   ├── mycelium-security/    # Cryptography and identity
│   └── mycelium-k3s/        # K3s integration components
├── services/                  # Distributed service binaries
│   ├── bootstrap-agent/      # Minimal core bootstrap
│   ├── network-manager/      # Distributed network management
│   └── update-manager/       # Decentralized update system
├── tools/                     # Development and operational tools
│   ├── mycelium-cli/        # Command-line interface
│   ├── network-installer/    # Network installation tool
│   └── dev-tools/           # Development utilities
├── tests/                     # Integration and end-to-end tests
│   ├── integration/         # Integration test suites
│   ├── chaos/               # Chaos engineering tests
│   └── performance/         # Performance benchmarks
├── scripts/                   # Build and development scripts
│   ├── setup-dev-env.sh     # Development environment setup
│   ├── run-tests.sh         # Test execution scripts
│   └── build-release.sh     # Release build scripts
└── config/                    # Configuration templates and examples
    ├── development/         # Development configurations
    ├── testing/            # Testing configurations
    └── production/         # Production configuration templates
```

## Development Environment Setup

### Prerequisites

#### System Requirements
- **Operating System**: Linux (Ubuntu 20.04+, Fedora 35+, Arch Linux)
- **CPU**: x86_64 or ARM64 (Apple Silicon supported)
- **Memory**: 8GB RAM minimum, 16GB recommended
- **Storage**: 50GB available space for development
- **Network**: Reliable internet connection for dependencies

#### Required Software
```bash
# Rust toolchain (latest stable)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup component add clippy rustfmt rust-src

# Additional Rust tools
cargo install cargo-audit cargo-outdated cargo-tree cargo-watch
cargo install cargo-nextest cargo-llvm-cov cargo-deny

# Container runtime (Docker or Podman)
# Ubuntu/Debian:
sudo apt-get install docker.io docker-compose
# Fedora:
sudo dnf install docker docker-compose
# Arch:
sudo pacman -S docker docker-compose

# Kubernetes tools
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl"
sudo install -o root -g root -m 0755 kubectl /usr/local/bin/kubectl

# K3s (for testing)
curl -sfL https://get.k3s.io | sh -

# Development tools
sudo apt-get install git build-essential pkg-config libssl-dev
```

### Development Environment Configuration

#### Rust Configuration
```toml
# .cargo/config.toml
[build]
rustflags = ["-D", "warnings"]

[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[alias]
# Development aliases
dev-test = "nextest run"
dev-check = "check --all-targets --all-features"
dev-clippy = "clippy --all-targets --all-features -- -D warnings"
dev-fmt = "fmt --all"
dev-audit = "audit"

# Coverage and benchmarks
coverage = "llvm-cov nextest --html"
bench = "bench --all-features"
```

#### IDE Configuration

##### Visual Studio Code
```json
// .vscode/settings.json
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.checkOnSave.allTargets": true,
    "rust-analyzer.cargo.allFeatures": true,
    "rust-analyzer.procMacro.enable": true,
    "rust-analyzer.inlayHints.enable": true,
    "rust-analyzer.completion.addCallParentheses": false,
    "files.watcherExclude": {
        "**/target/**": true
    },
    "search.exclude": {
        "**/target": true,
        "**/Cargo.lock": true
    }
}

// .vscode/extensions.json
{
    "recommendations": [
        "rust-lang.rust-analyzer",
        "vadimcn.vscode-lldb",
        "serayuzgur.crates",
        "tamasfe.even-better-toml",
        "ms-vscode.test-adapter-converter"
    ]
}
```

##### IntelliJ IDEA / CLion
```xml
<!-- .idea/runConfigurations/Cargo_Test.xml -->
<component name="ProjectRunConfigurationManager">
  <configuration default="false" name="Cargo Test" type="CargoCommandRunConfiguration">
    <option name="command" value="nextest run" />
    <option name="workingDirectory" value="$PROJECT_DIR$" />
    <option name="emulateTerminal" value="true" />
    <method v="2">
      <option name="CARGO.BUILD_TASK_PROVIDER" enabled="true" />
    </method>
  </configuration>
</component>
```

## Workspace Configuration

### Root Cargo.toml
```toml
[workspace]
members = [
    "crates/*",
    "services/*",
    "tools/*",
]
exclude = [
    "target",
    "tests/integration",
]

resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
rust-version = "1.70"
authors = ["Digital Mycelium Network Contributors"]
license = "MIT OR Apache-2.0"
repository = "https://github.com/mycelium-network/mycelium-network"
homepage = "https://mycelium-network.org"
documentation = "https://docs.mycelium-network.org"
keywords = ["distributed", "networking", "kubernetes", "consensus", "storage"]
categories = ["network-programming", "distributed-systems"]

[workspace.dependencies]
# Async runtime and utilities
tokio = { version = "1.0", features = ["full"] }
tokio-util = "0.7"
futures = "0.3"

# Serialization and data formats
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"
bincode = "1.3"

# Networking and protocols
quinn = "0.10"
rustls = "0.21"
webpki-roots = "0.25"
hyper = { version = "0.14", features = ["full"] }
tonic = "0.9"
prost = "0.11"

# Cryptography and security
ed25519-dalek = "2.0"
ring = "0.16"
rand = "0.8"
sha2 = "0.10"
blake3 = "1.4"

# Storage and databases
etcd-rs = "1.0"
rocksdb = "0.21"
sled = "0.34"

# Kubernetes integration
k8s-openapi = { version = "0.19", features = ["v1_27"] }
kube = { version = "0.85", features = ["runtime", "derive"] }
kube-runtime = "0.85"

# CLI and configuration
clap = { version = "4.0", features = ["derive"] }
config = "0.13"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Testing utilities
proptest = "1.2"
tempfile = "3.7"
wiremock = "0.5"

# Error handling and utilities
thiserror = "1.0"
anyhow = "1.0"
uuid = { version = "1.4", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }

[profile.dev]
opt-level = 0
debug = true
split-debuginfo = "unpacked"
debug-assertions = true
overflow-checks = true
lto = false
panic = "unwind"
incremental = true
codegen-units = 256

[profile.release]
opt-level = 3
debug = false
split-debuginfo = "packed"
debug-assertions = false
overflow-checks = false
lto = "thin"
panic = "abort"
incremental = false
codegen-units = 1

[profile.test]
opt-level = 1
debug = true
debug-assertions = true
overflow-checks = true
incremental = true

[profile.bench]
opt-level = 3
debug = false
debug-assertions = false
overflow-checks = false
lto = "thin"
incremental = false
codegen-units = 1
```

## Testing Framework

### Testing Strategy

#### Unit Testing
```rust
// Example unit test structure
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_basic_functionality() {
        // Standard unit tests
    }
    
    proptest! {
        #[test]
        fn test_property_based(input in any::<String>()) {
            // Property-based testing
        }
    }
}
```

#### Integration Testing
```toml
# tests/integration/Cargo.toml
[package]
name = "integration-tests"
version = "0.1.0"
edition = "2021"

[dependencies]
mycelium-core = { path = "../../crates/mycelium-core" }
mycelium-spores = { path = "../../crates/mycelium-spores" }
tokio-test = "0.4"
testcontainers = "0.14"
```

#### Chaos Engineering
```rust
// tests/chaos/network_partition.rs
use chaos_engineering::*;

#[tokio::test]
async fn test_network_partition_recovery() {
    let cluster = TestCluster::new(5).await;
    
    // Simulate network partition
    cluster.partition_nodes(vec![0, 1], vec![2, 3, 4]).await;
    
    // Verify system continues operating
    assert!(cluster.verify_consensus_in_partition(&[2, 3, 4]).await);
    
    // Heal partition
    cluster.heal_partition().await;
    
    // Verify full recovery
    assert!(cluster.verify_full_consensus().await);
}
```

### Test Configuration

#### Nextest Configuration
```toml
# .config/nextest.toml
[profile.default]
retries = 2
test-threads = "num-cpus"
slow-timeout = { period = "60s", terminate-after = 2 }

[profile.ci]
retries = 3
test-threads = 1
slow-timeout = { period = "120s", terminate-after = 1 }
fail-fast = false

[profile.integration]
test-threads = 1
retries = 1
slow-timeout = { period = "300s", terminate-after = 1 }
```

## Continuous Integration

### GitHub Actions Workflow

#### Main CI Pipeline
```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1

jobs:
  check:
    name: Check
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo check --all-targets --all-features

  test:
    name: Test Suite
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - uses: taiki-e/install-action@nextest
      - run: cargo nextest run --all-features

  fmt:
    name: Rustfmt
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt
      - run: cargo fmt --all -- --check

  clippy:
    name: Clippy
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy
      - uses: Swatinem/rust-cache@v2
      - run: cargo clippy --all-targets --all-features -- -D warnings

  coverage:
    name: Coverage
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - uses: taiki-e/install-action@cargo-llvm-cov
      - uses: taiki-e/install-action@nextest
      - run: cargo llvm-cov nextest --all-features --lcov --output-path lcov.info
      - uses: codecov/codecov-action@v3
        with:
          files: lcov.info

  integration:
    name: Integration Tests
    runs-on: ubuntu-latest
    services:
      etcd:
        image: quay.io/coreos/etcd:v3.5.9
        ports:
          - 2379:2379
        env:
          ETCD_LISTEN_CLIENT_URLS: http://0.0.0.0:2379
          ETCD_ADVERTISE_CLIENT_URLS: http://localhost:2379
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo test --test integration --all-features
```

#### Security Audit
```yaml
# .github/workflows/security.yml
name: Security Audit

on:
  schedule:
    - cron: '0 0 * * *'
  push:
    paths:
      - '**/Cargo.toml'
      - '**/Cargo.lock'

jobs:
  audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - uses: actions-rs/audit-check@v1
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
```

## Development Scripts

### Setup Script
```bash
#!/bin/bash
# scripts/setup-dev-env.sh

set -euo pipefail

echo "Setting up Digital Mycelium Network development environment..."

# Check prerequisites
command -v rustc >/dev/null 2>&1 || { echo "Rust not installed. Please install Rust first."; exit 1; }
command -v docker >/dev/null 2>&1 || { echo "Docker not installed. Please install Docker first."; exit 1; }

# Install Rust tools
echo "Installing Rust development tools..."
cargo install cargo-nextest cargo-llvm-cov cargo-audit cargo-deny cargo-outdated

# Setup git hooks
echo "Setting up git hooks..."
mkdir -p .git/hooks
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
set -e
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo nextest run --all-features
EOF
chmod +x .git/hooks/pre-commit

# Create development configuration
echo "Creating development configuration..."
mkdir -p config/development
cat > config/development/network.yaml << 'EOF'
network:
  name: "dev-network"
  bootstrap_nodes: []
  
node:
  type: "development"
  resources:
    cpu_limit: "2"
    memory_limit: "4Gi"
    
logging:
  level: "debug"
  format: "json"
EOF

echo "Development environment setup complete!"
echo "Run 'cargo dev-check' to verify everything is working."
```

### Test Runner Script
```bash
#!/bin/bash
# scripts/run-tests.sh

set -euo pipefail

TEST_TYPE=${1:-"all"}

case $TEST_TYPE in
  "unit")
    echo "Running unit tests..."
    cargo nextest run --lib --bins
    ;;
  "integration")
    echo "Running integration tests..."
    cargo test --test integration
    ;;
  "chaos")
    echo "Running chaos engineering tests..."
    cargo test --test chaos --features chaos-testing
    ;;
  "coverage")
    echo "Running tests with coverage..."
    cargo llvm-cov nextest --all-features --html
    echo "Coverage report generated in target/llvm-cov/html/"
    ;;
  "all")
    echo "Running all tests..."
    cargo nextest run --all-features
    cargo test --test integration
    ;;
  *)
    echo "Usage: $0 [unit|integration|chaos|coverage|all]"
    exit 1
    ;;
esac
```

## Quality Assurance

### Code Quality Tools

#### Clippy Configuration
```toml
# clippy.toml
avoid-breaking-exported-api = false
msrv = "1.70"

# Allowed lints for specific cases
allow = [
    "clippy::module_name_repetitions",
    "clippy::must_use_candidate",
]

# Denied lints for code quality
deny = [
    "clippy::all",
    "clippy::pedantic",
    "clippy::cargo",
    "clippy::nursery",
]
```

#### Rustfmt Configuration
```toml
# rustfmt.toml
edition = "2021"
max_width = 100
hard_tabs = false
tab_spaces = 4
newline_style = "Unix"
use_small_heuristics = "Default"
reorder_imports = true
reorder_modules = true
remove_nested_parens = true
merge_derives = true
use_try_shorthand = true
use_field_init_shorthand = true
force_explicit_abi = true
empty_item_single_line = true
struct_lit_single_line = true
fn_single_line = false
where_single_line = false
imports_layout = "Mixed"
merge_imports = false
```

#### Deny Configuration
```toml
# deny.toml
[licenses]
unlicensed = "deny"
allow = [
    "MIT",
    "Apache-2.0",
    "Apache-2.0 WITH LLVM-exception",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "ISC",
    "Unicode-DFS-2016",
]
deny = [
    "GPL-2.0",
    "GPL-3.0",
    "AGPL-1.0",
    "AGPL-3.0",
]

[bans]
multiple-versions = "warn"
wildcards = "allow"
highlight = "all"

[advisories]
vulnerability = "deny"
unmaintained = "warn"
yanked = "warn"
notice = "warn"
```

## Documentation

### Documentation Generation
```bash
# Generate API documentation
cargo doc --all-features --no-deps --open

# Generate book-style documentation
mdbook build docs/
mdbook serve docs/ --open
```

### Documentation Structure
```
docs/
├── book.toml              # mdBook configuration
├── src/                   # Documentation source
│   ├── SUMMARY.md        # Table of contents
│   ├── introduction.md   # Project introduction
│   ├── architecture/     # Architecture documentation
│   ├── development/      # Development guides
│   ├── deployment/       # Deployment guides
│   └── api/             # API documentation
└── theme/                # Custom documentation theme
```

## Development Workflow

### Branch Strategy
- **main**: Production-ready code
- **develop**: Integration branch for features
- **feature/***: Feature development branches
- **hotfix/***: Critical bug fixes
- **release/***: Release preparation branches

### Commit Convention
```
type(scope): description

[optional body]

[optional footer]
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

### Pull Request Process
1. Create feature branch from `develop`
2. Implement changes with tests
3. Ensure all CI checks pass
4. Request code review
5. Address review feedback
6. Merge to `develop` after approval

## Performance Monitoring

### Benchmarking
```rust
// benches/consensus_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mycelium_consensus::*;

fn consensus_benchmark(c: &mut Criterion) {
    c.bench_function("bft_consensus_round", |b| {
        b.iter(|| {
            // Benchmark BFT consensus round
            black_box(run_consensus_round())
        })
    });
}

criterion_group!(benches, consensus_benchmark);
criterion_main!(benches);
```

### Profiling
```bash
# CPU profiling with perf
cargo build --release
perf record --call-graph=dwarf ./target/release/mycelium-network
perf report

# Memory profiling with valgrind
cargo build
valgrind --tool=massif ./target/debug/mycelium-network
```

## Conclusion

This development environment provides a comprehensive foundation for building the Digital Mycelium Network with:

- **Structured Workspace**: Clear organization of components and responsibilities
- **Quality Assurance**: Comprehensive testing, linting, and security auditing
- **Automation**: CI/CD pipelines for continuous integration and deployment
- **Documentation**: Integrated documentation generation and maintenance
- **Performance**: Benchmarking and profiling tools for optimization

The environment supports the project's goals of reliability, security, and maintainability while providing developers with productive tooling and clear processes.