# Spore System Architecture

## Overview

The Spore System implements a three-tier discovery and coordination architecture that enables decentralized network formation and service discovery without single points of failure.

## Three-Tier Architecture

### Primary Spore (Tier 1)
- **Technology**: In-memory Raft consensus with persistent backup
- **Purpose**: High-speed coordination between Sclerotia nodes
- **Authority**: Highest priority in conflict resolution
- **Scope**: Network-wide coordination and authoritative state

### Seed Spore (Tier 2)
- **Technology**: External file storage (configurable locations)
- **Purpose**: Backup discovery and split-brain resolution
- **Authority**: Secondary priority, used when Primary unavailable
- **Scope**: Bootstrap coordination and partition recovery

### Latent Spore (Tier 3)
- **Technology**: Adaptive gossip protocol with CRDTs
- **Purpose**: P2P discovery fabric maintained by Rhizomorphs
- **Authority**: Lowest priority, validated against higher tiers
- **Scope**: Peer discovery and offline node tracking

## Authority Hierarchy

**Primary > Seed > Latent**

Conflicts are resolved by preferring data from higher-authority sources. All spore data is validated before merging, with tampering detection and connection confirmation.

## Data Structure

Spores contain:
- Network identity and compatibility information
- Active node registry with capabilities and trust scores
- Service registry with discovery endpoints
- Activity tracking and offline node data
- Cryptographic signatures for integrity

## Validation and Security

- **Tampering Detection**: Cryptographic signatures and integrity checks
- **Connection Confirmation**: Independent verification of node connections
- **Authority Validation**: Higher-tier spores validate lower-tier data
- **Network Isolation**: Cryptographic network identity prevents cross-network contamination

## Operational Modes

- **Network Formation**: Bootstrap new networks using Seed Spores
- **Normal Operation**: Primary Spore coordination with Latent Spore backup
- **Partition Recovery**: Seed and Latent Spores enable partition healing
- **Split-Brain Resolution**: Authority hierarchy and activity tracking resolve conflicts