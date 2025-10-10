# mycnet-consensus

Byzantine Fault Tolerant consensus system for the Mycelium Network using software-based trust establishment and commit-reveal protocols.

## Overview

The consensus system provides BFT consensus without specialized hardware by using:
- **Commit-Reveal Protocol**: Prevents result copying through cryptographic commitments
- **Trust Scoring**: Dynamic trust evaluation based on consensus participation
- **Software-Based BFT**: Tolerates up to 1/3 Byzantine failures using trust mechanisms
- **Spore Integration**: Works seamlessly with the spore authority hierarchy

## Architecture

### Commit-Reveal Protocol
1. **Commit Phase**: Nodes submit cryptographic hashes of their results
2. **Reveal Phase**: Nodes reveal actual results after all commits received
3. **Consensus**: Majority result accepted as canonical truth
4. **Trust Update**: Participating nodes receive trust score adjustments

### Trust Scoring System
- **Consensus Participation**: Nodes gain trust for correct consensus participation
- **Slashing Mechanism**: Dissenting nodes lose trust through penalty system
- **Threshold Exclusion**: Nodes below trust threshold excluded from critical operations
- **Recovery Path**: Gradual trust restoration through consistent good behavior

## Components

### BFTConsensusEngine
Main consensus coordinator using commit-reveal protocol.

```rust
let mut engine = BFTConsensusEngine::new(node_id);
let operation_id = engine.propose_operation(operation).await?;
let result = engine.execute_reveal_phase(operation_id).await?;
```

### TrustScoring
Dynamic trust evaluation system for consensus participants.

```rust
let mut trust_scoring = TrustScoring::new();
trust_scoring.update_trust_score(node_id, participation_record);
let trust_level = trust_scoring.get_trust_score(&node_id);
```

### ConsensusOperation
Different types of operations requiring consensus:

```rust
pub enum ConsensusOperation {
    NetworkConfiguration { /* ... */ },  // Full BFT required
    TrustScoreModification { /* ... */ }, // Full BFT required
    NodeAdmission { /* ... */ },          // Full BFT required
    ServiceDeployment { /* ... */ },      // Authority hierarchy
}
```

## Operation Types

### Critical Operations (Full BFT)
- Network configuration changes
- Trust score modifications
- Security policy updates
- Node admission/exclusion

### Standard Operations (Authority Hierarchy)
- Service deployment decisions
- Resource allocation
- Routine maintenance tasks
- Performance optimizations

## Security Properties

- **Byzantine Fault Tolerance**: Tolerates arbitrary node failures and malicious behavior
- **Sybil Resistance**: Trust scoring prevents identity-based attacks
- **Gradual Degradation**: System performance degrades gracefully under attack
- **Recovery Capability**: Network can recover from temporary majority compromise

## Integration with Spore System

The consensus system integrates with the spore authority hierarchy:
- **Primary Spore**: Coordinates high-priority consensus operations
- **Seed Spore**: Provides backup consensus and conflict resolution
- **Latent Spore**: Enables distributed consensus fabric
- **Authority Resolution**: Conflicts resolved using spore hierarchy

## Usage

```rust
use mycnet_consensus::{BFTConsensusEngine, ConsensusOperation, TrustScoring};

// Create consensus engine
let mut engine = BFTConsensusEngine::new(node_id);

// Propose network configuration change
let operation = ConsensusOperation::NetworkConfiguration {
    config_type: "security_policy".to_string(),
    proposed_change: new_policy_bytes,
};

let operation_id = engine.propose_operation(operation).await?;
let result = engine.execute_reveal_phase(operation_id).await?;
```

## Dependencies

- **async-raft**: Raft consensus implementation
- **crdt**: Conflict-free replicated data types
- **ed25519-dalek**: Cryptographic signatures for commit-reveal
- **blake3**: Hashing for cryptographic commitments

## Testing

```bash
cargo test -p mycnet-consensus
```

## Related Documentation

- [Consensus System Architecture](../../.kiro/specs/mycelium-net/architecture/core-systems/consensus-system.md)
- [Trust Scoring](../../.kiro/specs/mycelium-net/architecture/security/security-architecture.md)
- [Spore Integration](../../.kiro/specs/mycelium-net/architecture/core-systems/spore-system.md)