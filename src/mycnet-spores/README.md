# mycnet-spores

Three-tier spore discovery system for the Mycelium Network, providing resilient network discovery and coordination without single points of failure.

## Overview

The spore system implements a hierarchical discovery architecture with three tiers:
- **Primary Spore**: High-speed coordination (Raft consensus)
- **Seed Spore**: Backup discovery (external file storage)  
- **Latent Spore**: P2P discovery fabric (gossip protocol)

Authority hierarchy ensures consistency: **Primary > Seed > Latent**

## Architecture

### Three-Tier Hierarchy

```rust
pub enum SporeType {
    Primary,  // In-memory Raft consensus (highest authority)
    Seed,     // External file storage (backup discovery)
    Latent,   // Gossip protocol (P2P discovery fabric)
}
```

### Spore Data Structure
Spores contain network discovery information:
- Network identity and compatibility information
- Active node registry with capabilities and trust scores
- Service registry with discovery endpoints
- Activity tracking and offline node data
- Cryptographic signatures for integrity

### Authority Resolution
Conflicts are resolved by preferring data from higher-authority sources:
1. **Primary Spore**: Authoritative (result of node consensus)
2. **Seed Spore**: Backup mechanism for network coordination
3. **Latent Spore**: P2P discovery with validation against higher tiers

## Components

### SporeSystem
Main coordinator for all spore types.

```rust
let mut spore_system = SporeSystem::new();
spore_system.initialize(network_identity).await?;
```

### PrimarySpore
Raft-based consensus spore for high-speed coordination between Sclerotia nodes.

### SeedSpore
File-based spore for backup discovery and split-brain resolution.

### LatentSpore
Gossip-based spore maintained by Rhizomorphs for P2P discovery fabric.

## Validation and Security

### Tampering Detection
- Cryptographic signatures and integrity checks
- Connection confirmation through independent verification
- Authority validation where higher-tier spores validate lower-tier data

### Network Isolation
- Cryptographic network identity prevents cross-network contamination
- Validation-first merging strategy
- Fallback mechanisms when higher-authority spores unavailable

## Operational Modes

- **Network Formation**: Bootstrap new networks using Seed Spores
- **Normal Operation**: Primary Spore coordination with Latent Spore backup
- **Partition Recovery**: Seed and Latent Spores enable partition healing
- **Split-Brain Resolution**: Authority hierarchy and activity tracking resolve conflicts

## Usage

```rust
use mycnet_spores::{SporeSystem, SporeType, NetworkIdentity};

// Initialize spore system
let mut spore_system = SporeSystem::new();
let network_identity = NetworkIdentity {
    network_id: Uuid::new_v4(),
    network_name: "my-network".to_string(),
    genesis_timestamp: chrono::Utc::now(),
};

spore_system.initialize(network_identity).await?;
```

## Dependencies

- **sled**: Embedded database for spore persistence
- **quinn**: QUIC networking for spore exchange
- **ed25519-dalek**: Cryptographic validation
- **blake3**: Integrity checking
- **bincode**: Efficient serialization

## Testing

```bash
cargo test -p mycnet-spores
```

## Related Documentation

- [Spore System Architecture](../../.kiro/specs/mycelium-net/architecture/core-systems/spore-system.md)
- [Bootstrap and Split-Brain Resolution](../../.kiro/specs/mycelium-net/architecture/core-systems/bootstrap-split-brain.md)
- [Network Protocols](../../.kiro/specs/mycelium-net/architecture/networking/network-protocols.md)