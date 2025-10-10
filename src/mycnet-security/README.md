# mycnet-security

Cryptographic security and trust management for the Mycelium Network, providing network isolation, node authentication, and trust-based access control.

## Overview

The security system provides:
- **Network Identity**: Cryptographic isolation between networks
- **Node Authentication**: Multi-factor authentication with Ed25519 signatures
- **Trust Management**: Dynamic trust scoring based on behavior and participation
- **Secure Channels**: Encrypted communication using modern cryptography
- **Access Control**: Trust-based policies for resource access

## Architecture

### Network Identity and Isolation

```rust
pub struct NetworkIdentity {
    pub network_id: Uuid,
    pub network_name: String,
    pub isolation_key: [u8; 32],           // Cryptographic isolation
    pub genesis_timestamp: DateTime<Utc>,
    pub genesis_nodes: Vec<Uuid>,
}
```

### Node Authentication

```rust
pub struct NodeCredentials {
    pub node_id: Uuid,
    pub signing_keypair: ed25519_dalek::Keypair,      // Digital signatures
    pub encryption_keypair: x25519_dalek::StaticSecret, // Key exchange
    pub network_membership_proof: Vec<u8>,             // Network membership
}
```

### Trust Management

```rust
pub struct TrustScore {
    pub overall_score: f32,
    pub consensus_participation: f32,
    pub network_contribution: f32,
    pub uptime_reliability: f32,
    pub security_compliance: f32,
    pub last_updated: DateTime<Utc>,
}
```

## Components

### NetworkIdentity
Manages cryptographic network identity and membership validation.

```rust
let network = NetworkIdentity::new_genesis("my-network".to_string(), genesis_nodes);
let is_member = network.validate_membership(&node_proof, &signature, &public_key);
```

### NodeCredentials
Handles node authentication credentials and cryptographic operations.

```rust
let credentials = NodeCredentials::generate_for_network(&network_identity);
let signature = credentials.sign_message(b"authentication challenge");
```

### TrustManager
Evaluates and manages node trust scores based on behavior.

```rust
let mut trust_manager = TrustManager::new();
let trust_score = trust_manager.evaluate_trust(node_id);
let has_access = trust_manager.check_access_permission(&node_id, AccessLevel::Standard);
```

### SecureChannel
Provides encrypted communication channels between nodes.

```rust
let channel = SecureChannel::establish(local_secret, remote_public)?;
let encrypted_data = channel.encrypt(b"secret message")?;
let decrypted_data = channel.decrypt(&encrypted_data)?;
```

### AuthenticationManager
Manages the authentication process for network nodes.

```rust
let mut auth_manager = AuthenticationManager::new(network_identity, node_credentials);
let challenge = auth_manager.create_auth_challenge();
let authenticated = auth_manager.authenticate_node(node_id, public_key, &proof, &signature);
```

## Cryptographic Primitives

### Digital Signatures (Ed25519)
- **Node Identity**: Each node has unique Ed25519 keypair
- **Message Authentication**: All network messages cryptographically signed
- **Non-Repudiation**: Signatures provide proof of message origin

### Key Exchange (X25519)
- **Secure Channels**: Diffie-Hellman key exchange for session keys
- **Perfect Forward Secrecy**: Session keys don't compromise long-term keys
- **Connection Security**: All inter-node communication encrypted

### Authenticated Encryption (ChaCha20-Poly1305)
- **Fast Encryption**: Optimized for performance on various hardware
- **Authenticated**: Prevents tampering and ensures integrity
- **Secure**: Resistant to timing attacks and other cryptographic vulnerabilities

### Hashing (BLAKE3)
- **Fast Hashing**: High-performance cryptographic hashing
- **Network Membership**: Used for network membership proofs
- **Data Integrity**: Ensures data hasn't been tampered with

## Trust System

### Trust Evaluation Factors
- **Consensus Participation**: Correct participation in consensus rounds
- **Network Uptime**: Reliability and availability metrics
- **Security Compliance**: Adherence to security policies
- **Network Contribution**: Positive contributions to network health

### Access Levels
```rust
pub enum AccessLevel {
    Full,           // Full network access (0.9+ trust)
    Standard,       // Standard operations (0.7+ trust)
    Limited,        // Limited operations only (0.5+ trust)
    ReadOnly,       // Read-only access (0.3+ trust)
    Restricted,     // Heavily restricted (0.1+ trust)
}
```

### Trust Policies
- **Dynamic Evaluation**: Trust scores updated based on ongoing behavior
- **Slashing Mechanisms**: Trust penalties for malicious or incorrect behavior
- **Recovery Paths**: Gradual trust restoration through consistent good behavior
- **Threshold Exclusion**: Nodes below trust thresholds excluded from critical operations

## Network Isolation

### Cryptographic Isolation
- **Isolation Keys**: Each network has unique cryptographic isolation key
- **Membership Proofs**: Nodes must prove membership to join network
- **Cross-Network Prevention**: Prevents accidental cross-network communication
- **Identity Validation**: Cryptographic validation of network membership

## Usage

```rust
use mycnet_security::{NetworkIdentity, NodeCredentials, TrustManager, SecureChannel};

// Create network identity
let genesis_nodes = vec![Uuid::new_v4()];
let network = NetworkIdentity::new_genesis("secure-network".to_string(), genesis_nodes);

// Generate node credentials
let credentials = NodeCredentials::generate_for_network(&network);

// Set up trust management
let mut trust_manager = TrustManager::new();
trust_manager.update_consensus_participation(node_id, true, true);
let trust_score = trust_manager.evaluate_trust(node_id);

// Establish secure channel
let local_secret = x25519_dalek::StaticSecret::new(&mut rand::rngs::OsRng);
let remote_public = x25519_dalek::PublicKey::from(&remote_secret);
let channel = SecureChannel::establish(local_secret, remote_public)?;
```

## Dependencies

- **ed25519-dalek**: Ed25519 digital signatures
- **x25519-dalek**: X25519 key exchange
- **chacha20poly1305**: Authenticated encryption
- **blake3**: Cryptographic hashing
- **rustls**: TLS integration
- **webpki-roots**: Certificate validation

## Testing

```bash
cargo test -p mycnet-security
```

## Related Documentation

- [Security Architecture](../../.kiro/specs/mycelium-net/architecture/security/security-architecture.md)
- [Network Identity](../../.kiro/specs/mycelium-net/architecture/security/network-identity.md)
- [Trust Management](../../.kiro/specs/mycelium-net/architecture/core-systems/consensus-system.md)