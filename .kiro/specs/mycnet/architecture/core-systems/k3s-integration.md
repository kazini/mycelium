# K3s Integration Architecture

## Overview

The Digital Mycelium Network uses a pragmatic approach to K3s integration, starting with stock K3s for security and maintainability, with custom K3s builds as a future optimization goal.

## Integration Strategy

### Phase 1: Stock K3s + Custom Add-ons (MVP)

**Rationale**: 
- **Security First**: Automatic security updates from K3s upstream
- **Faster Development**: Focus on core networking and storage logic
- **Lower Risk**: Proven K3s stability and community support
- **Easier Maintenance**: No need to track and merge upstream changes

**Trade-offs Accepted**:
- ~15-20MB larger footprint (acceptable for MVP)
- Slightly slower startup (~30 seconds additional)
- Some unused components remain (minimal security impact)

### Stock K3s Configuration

Disable default components via configuration:
- **Traefik**: Disable default ingress controller
- **ServiceLB**: Disable default load balancer
- **Local-Storage**: Disable default storage provisioner
- **Flannel**: Replace with Mycelium networking

Custom add-ons:
- **MyceliumNetworking CNI**: Spore-aware networking with trust policies
- **MyceliumStorage CSI**: Distributed storage with trust-based replication
- **MyceliumServiceMesh**: Service discovery and load balancing

### Future Custom Build Strategy (Phase 2+)

**When to Consider Custom Build**:
- After MVP proves the architecture works
- When performance optimization becomes critical
- When dedicated resources for K3s maintenance are available
- When security update process is well-established

**Custom Build Benefits**:
1. **Automated Merge Pipeline**: CI/CD system to merge upstream security updates
2. **Component Removal**: Strip unused components for optimization
3. **First-Class Integration**: Build Mycelium components directly into K3s
4. **Regression Testing**: Comprehensive test suite for custom builds

## Implementation Phases

### Phase 1: Minimal Viable K3s (Weeks 1-4)
Start with stock K3s, disable default components:
- Use host networking initially
- Use local storage initially
- Focus on bootstrap and handoff mechanisms

### Phase 2: Custom Networking Integration (Weeks 5-8)
Replace Flannel with MyceliumNetworking:
- Spore-aware routing
- Multi-homing support
- Trust-based policies
- Direct node communication

### Phase 3: Custom Storage Integration (Weeks 9-12)
Replace local-path-provisioner with MyceliumStorage:
- Distributed block management
- Trust-based replication
- Encryption support
- Transparent failover

### Phase 4: Custom K3s Fork (Weeks 13-16)
Create custom K3s build with integrated components:
- Integrated networking and storage
- Bootstrap integration
- Monitoring and metrics
- Single binary distribution

## Technical Integration Points

### Agent Configuration
Add Mycelium-specific configuration to K3s agent:
- Network identity configuration
- Spore endpoint configuration
- Trust threshold settings
- Storage policy configuration

### CNI Integration
Replace default networking with MyceliumNetworking:
- Initialize spore-aware networking
- Set up trust-based policies
- Configure multi-homing
- Enable direct node communication

### Storage Integration
Integrate MyceliumStorage as default CSI:
- Initialize distributed storage
- Configure trust-based replication
- Set up encryption policies
- Enable transparent failover

## Deployment Strategy

### Single Package Structure
```
mycelium-network-installer/
├── bin/
│   ├── mycelium-k3s          # Custom K3s binary (future)
│   ├── bootstrap-agent       # Minimal core bootstrap
│   ├── mycelium-cni         # Custom CNI plugin
│   └── mycelium-csi         # Custom CSI driver
├── config/
│   ├── k3s-config.yaml      # K3s configuration
│   ├── network-config.yaml  # Network configuration
│   └── storage-config.yaml  # Storage configuration
└── install.sh               # Installation script
```

### Installation Process
1. **Validate Environment**: Check system requirements and dependencies
2. **Install K3s**: Install stock K3s with custom configuration
3. **Deploy Add-ons**: Install Mycelium CNI and CSI components
4. **Bootstrap Network**: Initialize network identity and spore discovery
5. **Handoff Control**: Transfer management to distributed services

This pragmatic approach enables rapid development while maintaining a clear path to optimization and customization as the project matures.