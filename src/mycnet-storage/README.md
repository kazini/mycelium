# mycnet-storage

Trust-aware distributed storage system for the Mycelium Network that integrates with the node hierarchy and trust scores to provide secure, resilient storage.

## Overview

The storage system provides:
- **Trust-Aware Allocation**: Storage placement based on node trust scores
- **Data Classification**: Different trust requirements for different data types
- **Topology-Aligned Replication**: Replication strategies that consider network topology
- **Service Isolation**: Isolated storage namespaces for different services
- **Kubernetes Integration**: CSI driver for seamless K3s integration

## Architecture

### Trust-Based Storage Allocation

```rust
pub enum DataClassification {
    Critical,   // Requires high-trust nodes only (0.9+ trust)
    Sensitive,  // Requires medium to high trust (0.7+ trust)
    Standard,   // Can use any trusted nodes (0.5+ trust)
    Public,     // Minimal trust requirements (0.1+ trust)
}
```

### Replication Strategies

```rust
pub enum ReplicationStrategy {
    HierarchyAware,        // Distribute across node hierarchy levels
    GeographicDistribution, // Distribute geographically
    TrustDiversification,  // Distribute across trust score ranges
    PerformanceOptimized,  // Optimize for read/write performance
}
```

## Components

### TrustAwareStorageManager
Main storage coordinator that evaluates trust requirements and allocates storage.

```rust
let mut storage_manager = TrustAwareStorageManager::new();
let allocation = storage_manager.allocate_storage(storage_request).await?;
```

### TrustEvaluator
Evaluates node trustworthiness for storage operations.

### ReplicationManager
Manages distributed replication with topology awareness.

### StoragePool
Organizes storage by trust levels and capabilities.

## Storage Types

### Network Storage
Infrastructure storage for network services:
- **Infrastructure**: Critical network infrastructure storage
- **Consensus**: Consensus and state storage with strong consistency
- **Discovery**: Spore storage with replication strategies
- **Configuration**: Configuration and policy storage with versioning

### Service Storage
Application-specific storage with isolation:
- **Isolated Namespaces**: Each service gets isolated storage namespace
- **Access Control**: Trust-based access policies
- **Cross-Service Coordination**: Controlled sharing between services
- **Lifecycle Management**: Storage lifecycle tied to service lifecycle

## Trust Integration

### Storage Trust Evaluation
```rust
struct StorageTrustFactors {
    uptime_percentage: f32,
    data_integrity_score: f32,
    io_performance_score: f32,
    response_time_score: f32,
    encryption_compliance: bool,
    access_control_score: f32,
    consensus_participation: f32,
    network_contribution: f32,
}
```

### Trust-Based Replication
- **High-Trust Data**: Replicated only to nodes with high trust scores
- **Geographic Distribution**: Considers both trust and geographic diversity
- **Hierarchy Awareness**: Distributes replicas across node hierarchy levels
- **Performance Optimization**: Balances trust requirements with performance

## Kubernetes Integration

### CSI Driver
Custom Container Storage Interface driver for K3s integration:
- **Dynamic Provisioning**: Automatic volume provisioning with trust constraints
- **Transparent Failover**: Seamless failover between storage nodes
- **Trust Policies**: Enforce trust-based storage policies
- **Performance Optimization**: Optimize for edge deployment scenarios

## Usage

```rust
use mycnet_storage::{TrustAwareStorageManager, StorageRequest, DataClassification};

// Create storage manager
let mut storage_manager = TrustAwareStorageManager::new();

// Request storage with trust requirements
let request = StorageRequest {
    volume_id: Uuid::new_v4(),
    size_bytes: 1024 * 1024 * 1024, // 1GB
    data_classification: DataClassification::Sensitive,
    replication_requirements: ReplicationRequirements {
        replica_count: 3,
        consistency_level: ConsistencyLevel::Strong,
        geographic_distribution: true,
    },
};

let allocation = storage_manager.allocate_storage(request).await?;
```

## Dependencies

- **sled**: Embedded database for metadata
- **kube**: Kubernetes client for CSI integration
- **k8s-openapi**: Kubernetes API types

## Testing

```bash
cargo test -p mycnet-storage
```

## Related Documentation

- [Storage Integration Architecture](../../.kiro/specs/mycelium-net/architecture/core-systems/storage-system.md)
- [Trust Management](../../.kiro/specs/mycelium-net/architecture/security/security-architecture.md)
- [Container Models](../../.kiro/specs/mycelium-net/architecture/container-models/)