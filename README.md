# Digital Mycelium Network
Digital Mycelium Network, or mycelium-net, is a completely decentralized architecture for services and storage.

*Note: This project's development is assisted by spec-driven LLM agents.*
______________________
## What Is It?

Mycelium is a fully decentralized, self-hosting distributed platform that eliminates single points of failure while providing scalable infrastructure for containerized applications. Inspired by the resilient, interconnected structure of fungal mycelium networks.

# Digital Mycelium Network
Digital Mycelium Network, or mycelium-net, is a completely decentralized architecture for services and storage.

*Note: This project's development is assisted by spec-driven LLM agents.*
______________________
## What Is It?

Mycelium is a fully decentralized, self-hosting distributed platform that eliminates single points of failure while providing scalable infrastructure for containerized applications. Inspired by the resilient, interconnected structure of fungal mycelium networks.

The Digital Mycelium Network is a **"living network"** that grows, adapts, and manages itself. Think of it as Kubernetes reimagined for true decentralization - where the network uses its own distributed processing power to manage itself, rather than relying on centralized control planes.

### Core Innovation: Self-Hosting Architecture

**Traditional Approach**: Heavy management software on every node that needs constant updates.

**Our Approach**: Lightweight ~100MB core bootstraps the network, then dynamically loads specialized components only when needed.

- **Minimal Core** (always present): Bootstrap agent, basic networking, K3s runtime, component manager
- **Dynamic Components** (on-demand): VM management, distributed RAM, advanced consensus, storage systems
- **Distributed Management** (network services): Network manager, update coordinator, configuration manager
- **Result**: Nodes stay lightweight, components loaded/unloaded as needed, network manages itself

## Why Mycelium?

Fungal mycelium networks have evolved over millions of years to be:
- **Resilient**: Survive massive damage and adapt to conditions
- **Efficient**: Optimal resource distribution with minimal waste  
- **Self-Organizing**: Complex behaviors emerge from simple local rules
- **Scalable**: Work from tiny patches to continent-spanning networks

We've translated these biological principles into distributed systems architecture.

## Key Features

### 🌐 **True Decentralization**
- No single points of failure
- Scales from 1 node to thousands
- Runs on consumer hardware (Raspberry Pi, old laptops)
- Network and container services continue operating even with massive node failures

### 🔄 **Self-Managing**
- Network updates itself without manual intervention
- Dynamic component loading based on task requirements
- Automatic resource optimization and component cleanup
- Self-healing when components fail or split
- Adaptive node roles with on-demand capability loading

### 🤝 **Federated Ecosystem**
- One network can run one container service, or many
- Configurable container distribution and characteristics
- Separate networks can communicate with each other
- Robust network identity and access protocols

### 🛡️ **Secure by Design**
- Network communications always encrypted
- Cryptographic network identity prevents cross-network contamination
- Byzantine Fault Tolerant consensus without specialized hardware
- Configurable per-container encryption

### 📦 **Developer Friendly**
- Single-package installer bundles everything needed
- Standard containerized applications work without modification
- Transparent distributed storage (apps see normal filesystems)
- Kubernetes-compatible but optimized for edge deployment
- Direct SSH access to containers for debugging and configuration
- Support for both ultimate redundancy and native decentralization

## Architecture Overview

### Node Types (Biological Metaphor)

**🏗️ Sclerotia (Full Nodes)**
- Permanent backbone nodes (like fungal survival structures)
- Run complete network services and consensus
- Two types: Dedicated (always-on) and Dynamic (adaptive)

**🌿 Rhizomorphs (Semi-Nodes)**  
- Adaptive clients that can take on varying responsibilities
- Can be promoted to Dynamic Sclerotia when needed
- Participate in peer-to-peer discovery network

**🍄 Hyphae (Pure Clients)**
- End-user connections (like fungal feeding structures)
- Consume network services without infrastructure participation
- Multi-homed connections for automatic failover and a seamless experience

### Discovery System: The Spore Network

**Primary Spore**: High-speed coordination between backbone nodes (Raft consensus)
**Seed Spore**: Optional dynamic backup and split-brain resolution (external storage)  
**Latent Spore**: Peer-to-peer discovery fabric (gossip protocol)

Authority hierarchy ensures consistency: Primary > Seed > Latent

### Container System: Three Deployment Models

Applications run as containerized organisms within the mycelium network:

**🍄 Endomycetes** (Native Distribution):
- Natively distributed containers using standard Kubernetes patterns
- Applications manage their own distribution, consensus, and coordination
- Use Service Spores for discovery and health monitoring
- Examples: Blockchain nodes, distributed databases, P2P systems

**🌿 Endophytes** (Distributed RAM VMs):
- VMs with active-passive RAM replication for near-zero downtime failover
- Primary VM runs locally, memory replicated to distributed backups via Rhizomorphs
- VM management and distributed RAM components loaded dynamically when needed
- Adaptive throttling balances performance vs replication consistency
- Examples: Legacy applications, full operating systems, game servers

**🔮 Future SSI** (Single System Image):
- Research goal for perfectly decentralized virtual machines
- Lock-step active-active fault tolerance with asynchronous processing
- Separate development track that doesn't block practical implementation

**Key Innovation**: Rhizomorphs serve as connection layer, providing parallel bandwidth aggregation and connection resilience for high-speed memory transfer.

## Getting Started

### Prerequisites
- Linux, macOS, or Windows (WSL2)
- 2GB to 4GB RAM recommended
- 10GB storage recommended
- Network connectivity

### Quick Install
```bash
# Download and run the installer (when available)
curl -sSL https://get.mycelium.network | bash

# Or build from source
git clone https://github.com/digital-mycelium-network/mycelium-net
cd mycelium-net
cargo build --release
```

### Create Your First Network
```bash
# Initialize a new network
mycelium init --network-name "my-network"

# Join an existing network  
mycelium join --seed-spore "https://example.com/spore.json"

# Deploy a service
mycelium deploy --image nginx --replicas 3
```

## Project Status

🚧 **Currently in Development** 🚧

We're building this incrementally with a focus on getting the core architecture right:

- **Phase 1**: Minimal Core + Endomycetes (1-4 nodes) *(In Progress)*
- **Phase 2**: Distributed Management + Endophytes with Distributed RAM *(Planned)*  
- **Phase 3**: Resilient Discovery and BFT Consensus *(Planned)*
- **Phase 4**: Advanced Node Hierarchy and Rhizomorph Integration *(Planned)*
- **Phase 5**: Production Security and Inter-Network Communication *(Planned)*
- **Research**: Future SSI (Single System Image) exploration *(Separate Track)*

## Documentation

### Core Specifications
- **[Project Vision & Rationale](.kiro/specs/digital-mycelium-network/outline.md)** - Why we're building this and our design philosophy
- **[Requirements](.kiro/specs/digital-mycelium-network/requirements.md)** - Formal requirements with acceptance criteria
- **[Technical Design](.kiro/specs/digital-mycelium-network/design.md)** - High-level architecture and system integration
- **[Implementation Plan](.kiro/specs/digital-mycelium-network/tasks.md)** - Development roadmap and tasks

### Architecture Documentation
- **[Core Systems](.kiro/specs/digital-mycelium-network/architecture/core-systems/)** - Spore system, consensus, storage, service lifecycle
- **[Container Models](.kiro/specs/digital-mycelium-network/architecture/container-models/)** - Endomycetes, Endophytes, distributed RAM
- **[Networking](.kiro/specs/digital-mycelium-network/architecture/networking/)** - Network protocols and inter-network communication
- **[Security](.kiro/specs/digital-mycelium-network/architecture/security/)** - Network identity, authentication, cryptography

### Source Code
- **[Core Components](src/)** - Rust workspace with all network components
  - **[mycnet-core](src/mycnet-core/)** - Minimal core components (bootstrap, networking)
  - **[mycnet-spores](src/mycnet-spores/)** - Three-tier spore discovery system
  - **[mycnet-consensus](src/mycnet-consensus/)** - BFT consensus with trust scoring
  - **[mycnet-storage](src/mycnet-storage/)** - Trust-aware distributed storage
  - **[mycnet-networking](src/mycnet-networking/)** - Multi-homing and adaptive protocols
  - **[mycnet-security](src/mycnet-security/)** - Cryptography and trust management

### Project Management
- **[Project Structure](.kiro/specs/digital-mycelium-network/map.md)** - File organization and development guidelines
- **[Technology Validation](.kiro/specs/digital-mycelium-network/documentation/validation-status/technology-validation.md)** - Technology stack validation results

## Contributing

We welcome contributions! The project is designed to be approachable:

1. **Start with the [Project Vision](.kiro/specs/digital-mycelium-network/outline.md)** to understand our philosophy
2. **Review the [Requirements](.kiro/specs/digital-mycelium-network/requirements.md)** to see what we're building
3. **Read the [Technical Design](.kiro/specs/digital-mycelium-network/design.md)** for implementation details
4. **Check the [Implementation Plan](.kiro/specs/digital-mycelium-network/tasks.md)** for current development priorities
5. **Explore the [Source Code](src/)** to see the current implementation progress

### Development Setup
```bash
# Clone the repository
git clone https://github.com/digital-mycelium-network/mycelium-net
cd mycelium-net

# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the workspace
cargo build --workspace

# Run tests
cargo test --workspace

# Check individual crates
cargo check -p mycnet-core
cargo check -p mycnet-spores
cargo check -p mycnet-consensus
```

## Community

- **GitHub Discussions**: Ask questions and share ideas
- **Issues**: Report bugs and request features
- **Discord**: Real-time chat with the community *(Coming Soon)*

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

Inspired by the incredible resilience and efficiency of biological mycelium networks, and built on the shoulders of giants in distributed systems research and the efforts of the Kubernetes community.

---

*"The network that manages itself, inspired by nature's most resilient distributed system."*"
