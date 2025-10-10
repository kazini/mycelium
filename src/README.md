# Mycelium Network: Source Code

This directory contains the Rust implementation of the Mycelium Network, organized as a Cargo workspace with modular crates.

## Workspace Structure

```
src/
├── mycnet-core/         # Minimal core components
├── mycnet-spores/       # Three-tier spore discovery system
├── mycnet-consensus/    # BFT consensus with trust scoring
├── mycnet-storage/      # Trust-aware distributed storage
├── mycnet-networking/   # Multi-homing and adaptive protocols
├── mycnet-security/     # Cryptography and trust management
└── Cargo.toml          # Workspace configuration
```

## Crate Overview

### mycnet-core
**Minimal core components that run outside containers**
- Bootstrap Agent for network initialization and handoff
- Basic Networking for initial connectivity and peer discovery
- Basic Spore Client for read-only spore operations during bootstrap
- Network Identity and Node Identity management
- Dynamic Component Manager for on-demand component loading

### mycnet-spores
**Three-tier spore discovery system**
- Primary Spore: In-memory Raft consensus (highest authority)
- Seed Spore: External file storage (backup discovery)
- Latent Spore: Gossip protocol (P2P discovery fabric)
- Authority hierarchy: Primary > Seed > Latent

### mycnet-consensus
**Byzantine Fault Tolerant consensus system**
- Commit-reveal protocol prevents result copying
- Trust scoring with slashing mechanisms
- Software-based trust establishment without specialized hardware
- Integration with spore authority hierarchy

### mycnet-storage
**Trust-aware distributed storage system**
- Trust-based storage allocation and replication
- Integration with node hierarchy and trust scores
- Support for different data classifications (Critical, Sensitive, Standard, Public)
- Topology-aligned replication strategies

### mycnet-networking
**Multi-homing and adaptive network protocols**
- QUIC-based communication with adaptive protocols
- Multi-homed connections for automatic failover
- Virtual endpoints for transparent connection management
- Protocol selection based on node types and capabilities

### mycnet-security
**Cryptographic security and trust management**
- Network identity with cryptographic isolation
- Node authentication and credential management
- Trust scoring and access control policies
- Secure channels for encrypted communication

## Development

### Building the Workspace
```bash
# Build all crates
cargo build --workspace

# Build specific crate
cargo build -p mycnet-core

# Run tests
cargo test --workspace

# Check for issues
cargo clippy --workspace
```

### Dependencies
All crates share common dependencies defined in the workspace `Cargo.toml`:
- **tokio**: Async runtime
- **serde**: Serialization
- **ed25519-dalek**: Digital signatures
- **quinn**: QUIC protocol
- **rustls**: TLS implementation
- **uuid**: Unique identifiers
- **chrono**: Date/time handling

### Architecture Alignment
Each crate implements components specified in the [architecture documentation](../.kiro/specs/digital-mycelium-network/architecture/). The implementation follows the self-hosting architecture principle with minimal core components and distributed management services.

## Dynamic Component Loading
The system implements a dynamic component loading architecture:
- **Core components** are always present for essential functionality
- **Specialized components** (VM management, distributed RAM, advanced storage) are loaded on-demand
- **Memory efficiency** through automatic component unloading when not needed
- **Disk management** with automatic cleanup of unused components
- **Network distribution** for efficient component sharing between nodes

## Implementation Status

✅ **Technology Validation Complete**: All dependencies validated and workspace structure established
🚧 **Phase 1 Development**: Currently implementing minimal core components and basic functionality
📋 **Next Steps**: See [tasks.md](../.kiro/specs/mycelium-net/tasks.md) for current development priorities

## Contributing

1. Read the [architecture documentation](../.kiro/specs/mycelium-net/architecture/) to understand the system design
2. Check the [implementation plan](../.kiro/specs/mycelium-net/tasks.md) for current development priorities
3. Follow Rust best practices and maintain consistency with the established code structure
4. Ensure all changes align with the self-hosting architecture principles