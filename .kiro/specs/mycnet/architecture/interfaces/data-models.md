# Data Models and Core Interfaces

## Overview

This document defines the core data models and interfaces used throughout the Digital Mycelium Network for consistent communication between components.

## Core Network Entities

### Node Identity
```rust
struct NodeIdentity {
    node_id: NodeId,
    node_type: NodeType,
    cryptographic_identity: PublicKey,
    network_addresses: Vec<SocketAddr>,
    capabilities: NodeCapabilities,
    trust_score: TrustScore,
    last_seen: Timestamp,
    activity_periods: Vec<ActivityPeriod>,
    offline_tracking: OfflineTrackingData,
}

enum NodeType {
    DedicatedSclerotia,
    DynamicSclerotia { current_load: f32 },
    Rhizomorph { promotion_eligible: bool },
    Hyphae,
}
```

### Network Identity
```rust
struct NetworkIdentity {
    network_id: NetworkId,
    network_name: String,
    genesis_timestamp: Timestamp,
    genesis_nodes: Vec<NodeId>,
    network_config: NetworkConfig,
    isolation_key: IsolationKey,
    compatibility_version: u32,
}
```

### Spore Data
```rust
struct SporeData {
    version: u32,
    network_identity: NetworkIdentity,
    active_nodes: Vec<NodeIdentity>,
    service_registry: HashMap<ServiceId, ServiceSpore>,
    trust_rankings: HashMap<NodeId, TrustScore>,
    last_updated: Timestamp,
    signature: Signature,
}
```

## Core System Interfaces

### Spore System Interface
```rust
trait SporeSystem {
    async fn register_node(&self, node: NodeIdentity) -> Result<()>;
    async fn get_active_nodes(&self) -> Result<Vec<NodeIdentity>>;
    async fn validate_network_identity(&self, identity: &NetworkIdentity) -> Result<bool>;
    async fn update_trust_scores(&self, scores: HashMap<NodeId, TrustScore>) -> Result<()>;
}
```

### Consensus Interface
```rust
trait ConsensusSystem {
    async fn propose_operation(&self, operation: Operation) -> Result<OperationId>;
    async fn commit_operation(&self, op_id: OperationId, commitment: Hash) -> Result<()>;
    async fn reveal_operation(&self, op_id: OperationId, result: OperationResult) -> Result<()>;
    async fn get_consensus_result(&self, op_id: OperationId) -> Result<ConsensusResult>;
}
```

### Storage Interface
```rust
trait DistributedStorage {
    async fn provision_volume(&self, service_id: ServiceId, size: u64) -> Result<VolumeId>;
    async fn replicate_data(&self, volume_id: VolumeId, nodes: Vec<NodeId>) -> Result<()>;
    async fn read_data(&self, volume_id: VolumeId, offset: u64, size: u64) -> Result<Vec<u8>>;
    async fn write_data(&self, volume_id: VolumeId, offset: u64, data: Vec<u8>) -> Result<()>;
}
```

## Service Management Interfaces

### Service Deployment Interface
```rust
trait ServiceManager {
    async fn deploy_service(&self, service: ServiceDefinition) -> Result<ServiceId>;
    async fn scale_service(&self, service_id: ServiceId, replicas: u32) -> Result<()>;
    async fn update_service(&self, service_id: ServiceId, update: ServiceUpdate) -> Result<()>;
    async fn terminate_service(&self, service_id: ServiceId) -> Result<()>;
}
```

### Service Discovery Interface
```rust
trait ServiceDiscovery {
    async fn register_service_instance(&self, instance: ServiceInstance) -> Result<()>;
    async fn discover_service(&self, service_id: ServiceId) -> Result<Vec<ServiceEndpoint>>;
    async fn update_service_health(&self, instance_id: InstanceId, health: HealthStatus) -> Result<()>;
    async fn deregister_service_instance(&self, instance_id: InstanceId) -> Result<()>;
}
```

## Network Management Interfaces

### Core-to-Network Interface
```rust
trait CoreToNetworkInterface {
    async fn register_node_capabilities(&self, capabilities: NodeCapabilities) -> Result<()>;
    async fn request_configuration_update(&self, node_id: NodeId) -> Result<NodeConfig>;
    async fn report_node_status(&self, status: NodeStatus) -> Result<()>;
    async fn handle_network_command(&self, command: NetworkCommand) -> Result<CommandResult>;
}
```

### Network Command Types
```rust
enum NetworkCommand {
    UpdateCoreComponent { component: CoreComponent, version: Version },
    ReconfigureNode { config: NodeConfig },
    DeployService { service: ServiceDefinition },
    TerminateService { service_id: ServiceId },
    EnterMaintenanceMode { duration: Duration },
    PerformHealthCheck,
    CollectDiagnostics,
}
```

## Error Types and Result Handling

### Common Error Types
```rust
#[derive(Debug, thiserror::Error)]
enum MyceliumError {
    #[error("Network error: {0}")]
    Network(#[from] NetworkError),
    
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    
    #[error("Consensus error: {0}")]
    Consensus(#[from] ConsensusError),
    
    #[error("Service error: {0}")]
    Service(#[from] ServiceError),
    
    #[error("Configuration error: {message}")]
    Configuration { message: String },
    
    #[error("Authentication error: {message}")]
    Authentication { message: String },
}
```

### Result Types
```rust
type MyceliumResult<T> = Result<T, MyceliumError>;

// Specialized result types for different subsystems
type SporeResult<T> = Result<T, SporeError>;
type ConsensusResult<T> = Result<T, ConsensusError>;
type StorageResult<T> = Result<T, StorageError>;
type ServiceResult<T> = Result<T, ServiceError>;
```

## Configuration Models

### Network Configuration
```rust
struct NetworkConfig {
    network_identity: NetworkIdentity,
    consensus_config: ConsensusConfig,
    storage_config: StorageConfig,
    security_config: SecurityConfig,
    performance_config: PerformanceConfig,
}
```

### Node Configuration
```rust
struct NodeConfig {
    node_type: NodeType,
    resource_limits: ResourceLimits,
    network_settings: NetworkSettings,
    service_assignments: Vec<ServiceAssignment>,
    monitoring_config: MonitoringConfig,
}
```

These data models and interfaces provide the foundation for consistent communication and interaction between all components of the Digital Mycelium Network.