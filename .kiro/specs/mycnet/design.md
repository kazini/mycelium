# Design Document

## Document Standards and Guidelines

**Purpose**: This design document provides a high-level architectural overview with references to detailed specifications. It serves as a navigation guide to the complete architecture.

**Structure Guidelines for Future Agents**:
- **Keep summaries concise**: Each section should provide overview-level information only
- **Reference detailed specs**: Always include `*See [Detailed Architecture](path/to/detailed/spec.md)*` for comprehensive information
- **Maintain consistency**: Follow the established pattern of brief descriptions followed by reference links
- **Avoid duplication**: Detailed implementation specifics belong in referenced architecture documents, not in this overview
- **Update references**: When adding new concepts, create appropriate detailed specification files and reference them

**Navigation Pattern**: Overview → Architecture → Core Systems → Implementation → References to Detailed Specs

## Overview

The Digital Mycelium Network is a fully decentralized, self-hosting distributed platform that eliminates single points of failure while providing scalable infrastructure for containerized applications. The system implements a **minimal core + distributed management** architecture where lightweight components bootstrap the network, then hand control to sophisticated services running within the network itself.

**Core Innovation**: Self-hosting architecture where the network manages itself using its own distributed capabilities, reducing update frequency and eliminating centralized management bottlenecks.

**Key Features**:
- Three-tier node hierarchy (Sclerotia/Rhizomorphs/Hyphae) inspired by fungal networks
- Multi-tiered discovery system (Primary/Seed/Latent Spores) with Byzantine fault tolerance
- Three container deployment models: Endomycetes, Endophytes, and future SSI research
- Distributed RAM system with Rhizomorph connection layer for near-zero downtime failover

## Architecture

### Self-Hosting Architecture Model

The Digital Mycelium Network employs a two-layer architecture that enables true self-management:

1. **Minimal Core Layer** (Node-Local): Lightweight components (~40MB) that bootstrap and maintain basic connectivity
2. **Distributed Management Layer** (Network Services): Sophisticated management running as services within the network

This architecture enables the network to manage itself using its own distributed capabilities, reducing the need for node updates and eliminating centralized management bottlenecks.

*See [Minimal Core Architecture](architecture/self-hosting/minimal-core.md) and [Distributed Management Services](architecture/self-hosting/distributed-management.md) for detailed specifications.*

### Node Hierarchy

The system implements a three-tier node hierarchy inspired by fungal networks:

- **Sclerotia (Full Nodes)**: Permanent backbone nodes running complete network services
  - *Dedicated*: Always-on high-performance nodes
  - *Dynamic*: Adaptive nodes that adjust based on constraints
- **Rhizomorphs (Semi-Nodes)**: Adaptive clients capable of promotion and specialized tasks
  - *Connection Layer*: Parallel bandwidth aggregation for distributed RAM replication
  - *Secondary Processing*: Dynamic computational assistance for all container models
  - *Task Offloading*: Specialized processing capabilities across the network
- **Hyphae (Pure Clients)**: End-user connections consuming network services

*See [Node Hierarchy Architecture](architecture/node-hierarchy/) for detailed role specifications.*

### Core Systems

- **Spore System**: Three-tier discovery (Primary/Seed/Latent) with Byzantine fault tolerance
- **Consensus System**: Software-based BFT using commit-reveal protocols and trust scoring
- **Storage System**: Transparent distributed storage with automatic replication
- **Container System**: Three deployment models (Endomycetes/Endophytes/SSI)

*See [Core Systems Architecture](architecture/core-systems/) for detailed system specifications.*

## Self-Hosting Components

### Minimal Core Components
Lightweight, node-local components that bootstrap the network:
- **Bootstrap Agent**: Network initialization and handoff coordination
- **Basic Networking**: Initial connectivity and peer discovery
- **K3s Runtime**: Custom container orchestration foundation
- **Basic Spore Client**: Read-only spore operations for bootstrap

### Distributed Management Services
Sophisticated services running as Endophytes within the network:
- **Network Manager**: Central coordination and orchestration
- **Update Manager**: Network-wide update coordination and rollback
- **Configuration Manager**: Dynamic configuration and policy management
- **Security Manager**: Access control and credential management
- **Monitoring Manager**: Network observability and health monitoring

### Core-to-Network Interface
Standardized interface enabling minimal core to communicate with distributed services:
- Node capability registration and status reporting
- Configuration updates and service assignments
- Remote command execution and health monitoring
- Emergency procedures and fallback mechanisms

*See [Self-Hosting Architecture](architecture/self-hosting/) for detailed component specifications and interfaces.*

## Core System Components

### Spore System (Discovery and Identity)
Three-tier discovery architecture with Byzantine fault tolerance:
- **Primary Spore**: In-memory Raft consensus for high-speed coordination
- **Seed Spore**: External storage for backup discovery and split-brain resolution  
- **Latent Spore**: Adaptive gossip protocol for P2P discovery fabric
- **Authority Hierarchy**: Primary > Seed > Latent for conflict resolution

*See [Spore System Architecture](architecture/core-systems/spore-system.md) for detailed specifications.*

### Consensus System
Software-based Byzantine Fault Tolerant consensus without specialized hardware:
- **Commit-Reveal Protocol**: Prevents result copying through cryptographic commits
- **Trust Scoring**: Dynamic trust management with slashing for bad actors
- **Quorum Management**: Adaptive quorum sizing based on network conditions
- **Operation Types**: Different consensus levels for different operation criticality

*See [Consensus System Architecture](architecture/core-systems/consensus-system.md) for detailed specifications.*

### Storage System
Transparent distributed storage with automatic replication:
- **Block Storage**: Distributed block storage (Longhorn/Ceph) as virtual devices
- **Automatic Replication**: Data chunked and replicated across Sclerotia nodes
- **Transparent Access**: Applications see standard filesystems
- **Parallel Access**: Clients pull chunks from multiple nodes for performance

### Service Management
Containerized application management with isolation and discovery:
- **Service Spores**: Independent discovery system for service coordination
- **Deployment Strategies**: Automatic, custom rules, or direct assignment
- **Namespace Isolation**: Kubernetes namespaces for security boundaries
- **Resource Management**: Dedicated compute, storage, and networking per service

### Container System Architecture

The Digital Mycelium Network supports three distinct container deployment models:

#### Endomycetes (Native Distribution)
Applications with built-in decentralization capabilities using standard Kubernetes patterns with Service Spores for coordination. Suitable for blockchain nodes, distributed databases, and P2P systems.

#### Endophytes (Distributed RAM VMs)  
Standard applications requiring high availability through active-passive RAM replication with Rhizomorph connection layer. Provides near-zero downtime failover for legacy applications and full operating systems.

#### Future SSI (Single System Image) - Advanced Research
Research goal for perfectly decentralized virtual machines with distributed OS process architecture:

**Distributed OS Process Architecture**:
- Individual OS processes distributed across containers for true scalability beyond single-node capacity
- Process-level containerization with network-transparent inter-process communication
- Asynchronous processing paradigms designed for distributed execution
- Inspiration from historical architectures like Kerrighed with modern container orchestration

**Truly Scalable Distributed OS**:
- Operating system not limited by single-node hardware constraints
- Dynamic resource allocation across the entire network
- Process migration and load balancing at the OS level
- Seamless scaling of individual applications beyond traditional VM boundaries

*See [Container Models Architecture](architecture/container-models/) for detailed specifications and implementation patterns.*

### Distributed RAM System

The Endophyte model uses active-passive RAM replication with Rhizomorph integration:
- **Active-Backup Architecture**: Primary VM with distributed memory replication
- **Rhizomorph Connection Layer**: Parallel bandwidth aggregation and connection resilience
- **Adaptive Throttling**: Configurable performance vs consistency trade-offs
- **Latency Adaptation**: Automatic optimization for WAN/LAN deployment scenarios

*See [Distributed RAM Architecture](architecture/container-models/distributed-ram/) for detailed specifications.*

### Rhizomorph Secondary Processing

Rhizomorph semi-nodes provide dynamic secondary processing capabilities that extend the computational capacity of all container models:

**Core Capabilities**:
- **Task Offloading**: Computational tasks dynamically distributed to available Rhizomorphs
- **Adaptive Load Balancing**: Automatic distribution based on Rhizomorph capabilities and network conditions
- **Elastic Scaling**: Processing capacity scales with the number of available Rhizomorphs
- **Specialized Processing**: Different Rhizomorphs can specialize in specific computation types

**Integration Across Container Models**:
- **Endomycete Enhancement**: Native distributed applications leverage Rhizomorphs for additional compute resources
- **Endophyte Acceleration**: VM-based applications benefit from offloaded processing without modification
- **Future SSI Support**: Individual OS processes can be distributed across Rhizomorph containers
- **Transparent Operation**: Applications see expanded processing capacity without architectural changes

*See [Rhizomorph Secondary Processing Architecture](architecture/container-models/rhizomorph-processing.md) for detailed specifications and use cases.*

## Networking and Connectivity

### Advanced Networking Features
- **VPN Capabilities**: Service-specific networking with isolated subnets
- **Static IP Allocation**: Dedicated IP addresses for services requiring them
- **External Gateway Access**: Controlled external network access through gateway nodes
- **Multi-Homing**: Clients maintain connections to multiple nodes for automatic failover
- **Connection Resilience**: Virtual endpoints that survive node failures transparently

### Service Discovery and Load Balancing
- **Service Spores**: Independent discovery system for service coordination
- **Automatic Load Balancing**: Intelligent traffic distribution across service instances
- **Health Monitoring**: Continuous health checking with automatic failover
- **Geographic Optimization**: Latency-aware routing and placement decisions

*See [Networking Architecture](architecture/networking/) for detailed networking specifications.*

## Data Models and Interfaces

### Core Network Entities
- **Node Identity**: Cryptographic identity, capabilities, trust scores, and activity tracking
- **Network Identity**: Unique network identification with isolation keys and compatibility versioning
- **Spore Data**: Discovery information with network state, service registry, and trust rankings
- **Service Definitions**: Service specifications with deployment strategies and resource requirements

### API Interfaces
- **Core-to-Network Interface**: Standardized communication between minimal core and distributed services
- **Spore System APIs**: Discovery, registration, and validation interfaces for all spore tiers
- **Consensus APIs**: BFT consensus operations with commit-reveal protocols
- **Storage APIs**: Distributed storage operations with transparent replication

*See [Data Models and APIs](architecture/interfaces/) for detailed specifications and interface definitions.*

## Security Architecture

### Network Security
- **Cryptographic Network Identity**: Unique network isolation with cryptographic keys
- **Inter-Node Communication**: All network traffic encrypted with modern protocols
- **Trust Management**: Dynamic trust scoring with Byzantine fault tolerance
- **Access Control**: RBAC with separation between network and service management

### Service Security
- **Configurable Encryption**: Per-service encryption settings for storage and communication
- **Namespace Isolation**: Kubernetes namespaces provide security boundaries between services
- **Certificate Management**: Automated certificate lifecycle management
- **Audit Logging**: Comprehensive logging of security events and access attempts

*See [Security Architecture](architecture/security/) for detailed security specifications.*



## Implementation Considerations

### Technology Stack
- **Rust**: Memory-safe, high-performance implementation with async/await patterns
- **Kubernetes/K3s**: Proven container orchestration with edge optimization
- **etcd**: Battle-tested distributed database for authoritative state
- **Standard Protocols**: gRPC, TLS, and other proven protocols for interoperability

### Performance Characteristics
- **Startup Time**: Network formation in under 30 seconds
- **Failover Time**: Service failover in under 5 seconds
- **Resource Usage**: Minimal core under 100MB RAM
- **Scalability**: Consistent performance from 1 to 1000+ nodes

### Deployment Scenarios
- **Single Node**: Full functionality on single consumer device
- **Small Networks**: 2-10 nodes for small organizations or development
- **Enterprise Networks**: 100+ nodes with advanced features and high availability
- **Geographic Distribution**: WAN deployment with latency adaptation

This design provides the theoretical foundation for implementing the Digital Mycelium Network's innovative self-hosting architecture while maintaining the biological metaphors that make the system intuitive to understand and operate.

## Error Handling and Fault Tolerance

### Fault Tolerance Strategies
- **Network Partitions**: Continue operation with eventual consistency and automatic reconciliation
- **Service Splits**: Intelligent merge strategies based on processing/data volume evaluation
- **Isolated Nodes**: Read-only mode with reconnection attempts and override mechanisms
- **Storage Failures**: Seamless I/O redirection and automatic re-replication
- **Consensus Failures**: Trust score management and automatic quorum recovery

*See [Error Handling Architecture](architecture/core-systems/error-handling.md) for detailed fault tolerance specifications.*

## Network Identity and Isolation

### Multiple Independent Networks
The system supports multiple independent networks with unique identities and isolation mechanisms:
- **Corporate vs Personal**: Separate networks with different isolation keys
- **Development vs Production**: Version compatibility and environment isolation
- **Geographic Boundaries**: Regional compliance and data sovereignty
- **Multi-Tenant**: Isolated networks per tenant with independent trust domains

### Network Bridging
- **Bridge Types**: Read-only, service proxy, and full bidirectional bridges
- **Access Control**: Fine-grained permissions and trust mapping
- **Security**: Authentication, encryption, and comprehensive audit logging

*See [Network Identity Architecture](architecture/security/network-identity.md) for detailed isolation specifications.*

## K3s Integration Strategy

### Pragmatic Integration Approach
- **Phase 1**: Stock K3s with custom add-ons for rapid development
- **Phase 2**: Custom networking integration replacing Flannel
- **Phase 3**: Custom storage integration with distributed capabilities
- **Phase 4**: Custom K3s fork with integrated Mycelium components

### Benefits
- **Security First**: Automatic security updates from K3s upstream
- **Faster Development**: Focus on core networking and storage logic
- **Lower Risk**: Proven K3s stability and community support
- **Future Optimization**: Clear path to custom builds when needed

*See [K3s Integration Architecture](architecture/core-systems/k3s-integration.md) for detailed integration specifications.*

### Decentralized Update Strategy

The system solves the "bootstrap update problem" - how to update the core components that run the update system itself - through a multi-layer update architecture:

- **Multi-Layer Architecture**: Hardware-level update agent, network-coordinated orchestration, and component-specific handlers
- **Bootstrap Swap Procedure**: Critical "engine swap" that updates core components without downtime
- **Update Coordination**: Rolling updates with quorum maintenance and blue-green network deployments
- **Self-Updating Agents**: Update agents can update themselves through atomic binary replacement
- **Automatic Rollback**: Health monitoring with automatic rollback on critical failures
- **Network Orchestration**: BFT consensus for update approval and network-wide coordination

*See [Decentralized Update System Architecture](architecture/core-systems/update-system.md) for detailed specifications.*

## Advanced System Components

The Digital Mycelium Network architecture includes the following advanced system components that provide comprehensive functionality:

### **1. Consensus and Trust Framework**
- **BFT Consensus Scope**: Critical operations use full BFT consensus while operational decisions use authority hierarchy resolution
- **Trust Score Foundation**: Genesis nodes establish initial trust, propagated through node hierarchy with cryptographic validation
- **Consensus Integration**: Three-tier spore system provides layered consensus with Primary > Seed > Latent authority resolution

*See [Consensus System Architecture](architecture/core-systems/consensus-system.md) for detailed consensus mechanisms and trust propagation.*

### **2. Service Lifecycle and Resource Management**
- **Service Assignment Logic**: Automatic, rule-based, or direct assignment strategies distribute services across node hierarchy based on capabilities
- **Resource Allocation Model**: Hierarchical resource pools with Sclerotia getting full allocation, Rhizomorphs adaptive, and Hyphae minimal
- **Service Health and Migration**: Continuous health monitoring with blue-green, rolling, and live migration strategies for service resilience

*See [Service Lifecycle Architecture](architecture/core-systems/service-lifecycle.md) for detailed service management and resource allocation.*

### **3. Storage Integration Architecture**
- **Trust-Aware Storage**: Storage allocation based on data classification and node trust scores with high-trust nodes for critical data
- **Storage Replication Strategy**: Topology-aligned replication across hierarchy levels, geographic regions, and trust diversity for resilience
- **Service Storage Isolation**: Isolated storage namespaces per service with Service Spore integration and cross-service coordination

*See [Storage System Architecture](architecture/core-systems/storage-system.md) for detailed storage integration and trust-aware allocation.*

### **4. Network Communication Protocols**
- **Inter-Node Communication**: Adaptive protocols based on node types with high-performance for Sclerotia, standard for Rhizomorphs, lightweight for Hyphae
- **Spore Exchange Protocols**: Three-tier exchange with high-priority for Primary, standard for Seed, gossip for Latent spores with conflict resolution
- **Multi-Homing Implementation**: Virtual endpoints with multiple physical connections, automatic failover, and load balancing strategies

*See [Network Protocols Architecture](architecture/networking/network-protocols.md) and [Inter-Network Communication](architecture/networking/inter-network-communication.md) for detailed communication specifications.*

### **5. Security and Cryptographic Model**
- **Network Identity Cryptography**: Ed25519 keypairs with network certificates, isolation keys, and distributed key rotation for network identity
- **Node Authentication**: Multi-factor authentication with cryptographic proofs, network knowledge, behavioral analysis, and consensus participation
- **Inter-Network Security**: Secure bridge establishment with mutual authentication, encrypted channels, and cross-network trust management

*See [Security Architecture](architecture/security/security-architecture.md) and [Network Identity](architecture/security/network-identity.md) for detailed security specifications.*

These architectural components provide comprehensive coverage of all major structural areas, completing the foundation for implementing the Digital Mycelium Network with all specified requirements: decentralization, scalability, self-optimization, configurable security, support for distributed applications with their own storage systems, inter-network communication capabilities, and efficient decentralized update management.

## System Integration Overview

The Digital Mycelium Network components integrate through well-defined interfaces and protocols:

### Component Interaction Patterns
- **Bootstrap Sequence**: Minimal Core → Spore Discovery → Network Services → Application Services
- **Service Communication**: Service Spores coordinate discovery while Core Systems handle consensus and storage
- **Failure Handling**: Graceful degradation with automatic recovery and circuit breaker patterns
- **Update Coordination**: Network-wide coordination ensures system consistency during updates

### Cross-System Dependencies
- **Spore System** provides discovery for all other components
- **Consensus System** coordinates critical decisions across the network
- **Storage System** provides persistent state for all network services
- **Security System** protects all inter-component communication

*See [System Integration Architecture](architecture/integration/system-integration.md) for detailed integration specifications.*

## Error Handling and Testing

### Error Handling Strategy
- **Graceful Degradation**: System continues operating with reduced functionality during failures
- **Automatic Recovery**: Self-healing mechanisms for common failure scenarios
- **Fallback Procedures**: Clear fallback paths when distributed services are unavailable
- **Circuit Breakers**: Fault isolation to prevent cascade failures

### Testing Strategy
- **Unit Testing**: Component-level validation with property-based testing
- **Integration Testing**: Multi-component interaction validation
- **Chaos Engineering**: Fault tolerance validation through controlled failure injection
- **Performance Testing**: Scalability and performance validation across different scenarios

*See [Testing Framework](reference/testing-framework.md) for comprehensive testing approaches.*

## Implementation Strategy

### Rust-First Architecture
- **Performance & Safety**: Async/await patterns with tokio, memory safety without garbage collection
- **Independent Libraries**: Core components as standalone crates (mycelium-ram, mycelium-spores, mycelium-consensus)
- **Kubernetes Integration**: Custom operators using kube-rs for type-safe API management
- **Cross-Platform**: Compiles to all architectures without runtime dependencies

### Development Approach
- **Incremental Phases**: Minimal core → Distributed management → Advanced features
- **Self-Hosting Focus**: Network manages itself using distributed services
- **Consumer Device First**: Optimized for Raspberry Pi to enterprise hardware
- **Independent Components**: Libraries designed for reuse in other projects

*See [Rust Implementation Patterns](reference/rust-implementation-patterns.md) and [Testing Framework](reference/testing-framework.md) for detailed implementation guidance.*

## Validation Strategy

### Comprehensive Testing Framework
- **Unit Tests**: Component validation with property-based testing using proptest
- **Integration Tests**: Multi-component interaction with mock environments  
- **Chaos Engineering**: Random failures, network partitions, and fault tolerance validation
- **Performance Tests**: Scalability from single nodes to large clusters

### Key Validation Areas
- **Container Models**: Endomycete and Endophyte deployment across scenarios
- **Distributed RAM**: Throttling curves, migration convergence, failover scenarios
- **Rhizomorph Integration**: Parallel connections, bandwidth aggregation, failure recovery
- **Spore System**: Authority hierarchy, conflict resolution, partition recovery

*See [Testing Framework Reference](reference/testing-framework.md) for comprehensive test implementations.*

## Future Research: Advanced Container Models

### Future SSI (Single System Image) Research

The Future SSI research explores distributed operating system architectures that transcend single-node limitations:

**Research Goals**:
- **Distributed OS Process Architecture**: Individual OS processes distributed across containers with network-transparent IPC
- **Historical Architecture Integration**: Leverage lessons from Kerrighed and similar projects with modern container orchestration
- **Asynchronous Processing Model**: Event-driven architecture designed for network delays and partition tolerance
- **Unlimited Scalability**: Applications not constrained by single-node hardware limits

**Key Research Areas**:
- **Process-Level Distribution**: Each OS process runs in its own container with live migration capabilities
- **Network-Transparent IPC**: Inter-process communication that works seamlessly across network boundaries
- **Dynamic Resource Allocation**: CPU, memory, and I/O resources abstracted across the entire network
- **Application Compatibility**: Standard Linux applications work without modification

*See [Distributed OS Architecture Research](architecture/container-models/distributed-os/) for detailed research specifications, implementation roadmap, and research questions.*
