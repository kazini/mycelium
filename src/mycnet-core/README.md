# mycnet-core

Minimal core components for the Mycelium Network that run outside of containers and provide the foundation for the self-hosting architecture.

## Overview

The core crate contains essential components that:
- Bootstrap new nodes and join existing networks
- Provide basic networking for initial connectivity
- Handle network identity and cryptographic node identity
- Implement the handoff mechanism to distributed management services

## Components

### NetworkIdentity
Cryptographic network identity that uniquely identifies a mycelium network and prevents cross-network communication.

```rust
let network = NetworkIdentity::new_genesis("my-network".to_string());
```

### NodeIdentity
Node identity with Ed25519 keypair for cryptographic authentication.

```rust
let node = NodeIdentity::new(NodeType::Hyphae);
let signature = node.sign_message(b"hello world");
```

### BootstrapAgent
Responsible for network initialization and handoff coordination.

```rust
let mut agent = BootstrapAgent::new(network_identity, node_identity);
agent.initialize_and_join().await?;
```

### BasicSporeClient
Read-only spore client for bootstrap operations.

### BasicNetworking
Initial connectivity and peer discovery before handoff to distributed services.

### DynamicComponentManager
On-demand loading and management of specialized components:
- Downloads components from network when needed for specific tasks
- Manages component lifecycle (loading, unloading, cleanup)
- Handles component caching and version management
- Integrates with spore system for component discovery

## Architecture Principles

### Minimal Core Philosophy
- Keep components lightweight and rarely updated
- Focus on essential bootstrap functionality only
- Hand off complex operations to distributed services
- Maintain backward compatibility for stability

### Self-Hosting Handoff
The core components bootstrap the network and then transfer control to sophisticated management services running within the network itself. This enables:
- Rare updates to node-local components
- Network self-management using distributed capabilities
- Separation of concerns between bootstrap and operations

## Dependencies

- **ed25519-dalek**: Cryptographic signatures
- **blake3**: Hashing for network membership validation
- **uuid**: Unique identifiers
- **chrono**: Timestamp handling
- **getrandom**: Secure random number generation

## Usage

```rust
use mycnet_core::{NetworkIdentity, NodeIdentity, NodeType, BootstrapAgent, DynamicComponentManager};

// Create network identity (genesis node)
let network = NetworkIdentity::new_genesis("my-network".to_string());

// Create node identity
let node = NodeIdentity::new(NodeType::Rhizomorph { promotion_eligible: true });

// Bootstrap and join network
let mut agent = BootstrapAgent::new(network, node);
agent.initialize_and_join().await?;

// Dynamic component loading example
let mut component_manager = DynamicComponentManager::new();
let vm_manager = component_manager.load_component(
    ComponentId::VirtualMachineManager,
    TaskContext::VmHosting(vm_spec)
).await?;
```

## Testing

```bash
cargo test -p mycnet-core
```

## Related Documentation

- [Self-Hosting Architecture](../../.kiro/specs/mycelium-net/architecture/self-hosting/)
- [Network Identity](../../.kiro/specs/mycelium-net/architecture/security/network-identity.md)
- [Bootstrap Process](../../.kiro/specs/mycelium-net/architecture/core-systems/bootstrap-split-brain.md)