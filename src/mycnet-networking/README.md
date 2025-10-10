# mycnet-networking

Multi-homing and adaptive network protocols for the Mycelium Network, providing resilient communication with automatic failover and protocol adaptation.

## Overview

The networking system provides:
- **Multi-Homing**: Connections to multiple nodes for automatic failover
- **Adaptive Protocols**: Protocol selection based on node types and capabilities
- **Virtual Endpoints**: Transparent connection management with load balancing
- **QUIC Integration**: Modern protocol with built-in encryption and multiplexing
- **Connection Resilience**: Automatic recovery and health monitoring

## Architecture

### Communication Protocols

```rust
pub enum CommunicationProtocol {
    HighPerformance {     // Sclerotia-to-Sclerotia
        encryption_level: EncryptionLevel,
        compression_level: CompressionLevel,
    },
    Adaptive {            // Capability-based adaptation
        base_protocol: BaseProtocol,
        adaptation_strategy: AdaptationStrategy,
    },
    Standard {            // Most inter-node communication
        negotiation_enabled: bool,
        encryption_level: EncryptionLevel,
    },
    Lightweight {         // Client connections
        encryption_level: EncryptionLevel,
        reliability_level: ReliabilityLevel,
    },
}
```

### Multi-Homing Strategy

```rust
pub enum ConnectionSelectionStrategy {
    RoundRobin,          // Distribute load evenly
    LatencyBased,        // Select lowest latency
    LoadBased,           // Select lowest load
    OperationSpecific,   // Select based on operation type
}
```

## Components

### MultiHomingManager
Manages connections to multiple nodes for resilience.

```rust
let mut manager = MultiHomingManager::new(ConnectionSelectionStrategy::LatencyBased);
let virtual_endpoint = manager.establish_multi_homed_connections(target_nodes).await?;
```

### VirtualEndpoint
Provides transparent multi-homing with automatic failover.

```rust
let response = virtual_endpoint.route_request(&request_data).await?;
```

### ConnectionHealthMonitor
Monitors connection health and triggers failover when needed.

### NodeConnection
Represents a connection to a network node with protocol information.

## Protocol Selection

### Node Type Based Selection
- **Sclerotia ↔ Sclerotia**: High-performance protocol with low compression
- **Sclerotia ↔ Rhizomorph**: Adaptive protocol based on Rhizomorph capabilities
- **Rhizomorph ↔ Rhizomorph**: Standard protocol with peer negotiation
- **Any ↔ Hyphae**: Lightweight protocol optimized for client connections

### Adaptive Strategies
- **CapabilityBased**: Adapt based on node capabilities
- **LatencyBased**: Optimize for network latency
- **BandwidthBased**: Optimize for available bandwidth

## Multi-Homing Features

### Connection Resilience
- **Automatic Failover**: Seamless switching between connections
- **Health Monitoring**: Continuous connection health assessment
- **Recovery Mechanisms**: Automatic reconnection and recovery
- **Load Balancing**: Distribute traffic across healthy connections

### Virtual Endpoints
- **Transparent Routing**: Applications see single endpoint
- **Connection Pooling**: Efficient connection reuse
- **Failover Policies**: Configurable failover behavior
- **Performance Optimization**: Intelligent connection selection

## QUIC Integration

### Benefits of QUIC
- **Built-in Encryption**: TLS 1.3 encryption by default
- **Multiplexing**: Multiple streams without head-of-line blocking
- **Connection Migration**: Connections survive network changes
- **Reduced Latency**: Faster connection establishment

### Implementation
```rust
use quinn::{Endpoint, ServerConfig, ClientConfig};

// QUIC provides modern networking with:
// - Built-in security and multiplexing
// - Excellent for multi-homing scenarios
// - Lower latency than traditional TCP+TLS
```

## Usage

```rust
use mycnet_networking::{MultiHomingManager, ConnectionSelectionStrategy, VirtualEndpoint};

// Create multi-homing manager
let mut manager = MultiHomingManager::new(ConnectionSelectionStrategy::LatencyBased);

// Establish multi-homed connections
let target_nodes = vec![node1_id, node2_id, node3_id];
let virtual_endpoint = manager.establish_multi_homed_connections(target_nodes).await?;

// Use virtual endpoint transparently
let request_data = b"hello world";
let response = virtual_endpoint.route_request(request_data).await?;
```

## Dependencies

- **quinn**: QUIC protocol implementation
- **rustls**: TLS implementation for security
- **webpki-roots**: Certificate validation
- **tokio-util**: Async utilities
- **bincode/postcard**: Efficient serialization

## Testing

```bash
cargo test -p mycnet-networking
```

## Related Documentation

- [Network Communication Protocols](../../.kiro/specs/mycelium-net/architecture/networking/network-protocols.md)
- [Inter-Network Communication](../../.kiro/specs/mycelium-net/architecture/networking/inter-network-communication.md)
- [Node Hierarchy](../../.kiro/specs/mycelium-net/architecture/container-models/overview.md)