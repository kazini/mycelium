# Minimal Core Architecture

## Overview

The minimal core implements the self-hosting architecture's foundation: lightweight, rarely-updated components that bootstrap the network and hand control to distributed management services.

## Core Components

### Bootstrap Agent
- **Purpose**: Initialize node and join network
- **Responsibilities**: Network identity validation, initial service discovery, handoff coordination
- **Size**: ~5MB binary
- **Update Frequency**: Rare (major version changes only)
- **Component Management**: Includes dynamic component manager for on-demand loading

### Basic Networking
- **Purpose**: Establish initial network connectivity
- **Responsibilities**: Peer discovery, secure channel establishment, heartbeat maintenance
- **Protocols**: TLS, basic discovery protocols
- **Dependencies**: Minimal system networking

### K3s Runtime
- **Purpose**: Container orchestration foundation
- **Customization**: Stripped-down build with mycelium-specific components
- **Size**: ~35MB (vs 40MB stock K3s)
- **Integration**: Custom networking, storage, and security components

### Basic Spore Client
- **Purpose**: Connect to spore discovery system
- **Responsibilities**: Fetch bootstrap spores, register with network, locate management services
- **Scope**: Read-only spore operations
- **Handoff**: Transfers control to distributed Spore Manager
- **Component Discovery**: Discovers available dynamic components via spore system

## Self-Hosting Handoff Process

1. **Bootstrap**: Minimal core starts and validates network identity
2. **Discovery**: Locate Network Manager Service endpoints via spores
3. **Registration**: Register node capabilities with Network Manager
4. **Handoff**: Transfer management control to distributed services
5. **Monitoring**: Maintain minimal monitoring of distributed services
6. **Fallback**: Resume control if distributed services fail

## Design Principles

### Minimal Footprint
- Total size under 100MB (including dynamic component manager)
- Minimal dependencies for core functionality
- Fast startup (under 30 seconds)
- Low resource usage for base operations
- Dynamic loading keeps memory usage minimal when components not needed

### Rare Updates
- Stable interfaces with distributed services
- Backward compatibility maintenance
- Update only for critical security or compatibility issues
- Self-updating capability through distributed Update Manager

### Robust Handoff
- Graceful degradation if distributed services unavailable
- Automatic retry and recovery mechanisms
- Clear success/failure indicators
- Rollback capability if handoff fails

## Interface to Distributed Services

### Network Manager Interface
- Node capability registration
- Service assignment requests
- Configuration update requests
- Health status reporting

### Command Interface
- Receive management commands from distributed services
- Execute local operations (restart, reconfigure, update)
- Report command execution status
- Handle emergency operations

## Failure Modes and Recovery

### Distributed Service Failure
- Resume basic network operations
- Maintain existing service containers
- Attempt periodic reconnection
- Provide read-only status information

### Core Component Failure
- Automatic restart with state preservation
- Fallback to last known good configuration
- Emergency contact to peer nodes for assistance
- Manual recovery procedures for catastrophic failure