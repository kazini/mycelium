# Mycelium Network: Architecture Reference

This directory contains detailed architectural documentation and specifications for the Mycelium Network components.

## Directory Structure

- **`core-systems/`** - Core system architectures (Spores, Consensus, Storage, Dynamic Components)
- **`interfaces/`** - Data models and API specifications
- **`container-models/`** - Container deployment models (Endomycetes, Endophytes, SSI)
- **`networking/`** - Network protocols and communication patterns
- **`security/`** - Security models and cryptographic systems
- **`self-hosting/`** - Self-hosting architecture details

## Usage

These documents provide the theoretical foundation and detailed specifications for implementing the Mycelium Network. Each directory contains focused documentation on specific architectural aspects.

## Implementation Status

The architecture has been validated and is ready for implementation. See the [Technology Validation Report](../documentation/validation-status/technology-validation.md) for details on technology stack validation and implementation readiness.

## Source Code Structure

The architecture is being implemented in the [src/](../../../src/) directory with the following Rust crates:

- **`mycnet-core/`** - Minimal core components (bootstrap agent, basic networking, spore client, dynamic component manager)
- **`mycnet-spores/`** - Three-tier spore discovery system implementation
- **`mycnet-consensus/`** - BFT consensus with commit-reveal protocol and trust scoring (dynamically loaded)
- **`mycnet-storage/`** - Trust-aware distributed storage system (dynamically loaded)
- **`mycnet-networking/`** - Multi-homing and adaptive network protocols
- **`mycnet-security/`** - Cryptographic security and trust management

## Dynamic Component Architecture

The system implements a dynamic component loading architecture where specialized components (VM management, distributed RAM, advanced consensus, storage systems) are loaded on-demand only when nodes need them for specific tasks. This keeps the base installation lightweight while providing full functionality when required.

Key benefits:
- **Minimal Base Footprint**: Core installation under 100MB
- **Memory Efficiency**: Components loaded only when actively needed
- **Automatic Cleanup**: Unused components automatically unloaded and cleaned up
- **Network Distribution**: Efficient P2P component sharing between nodes