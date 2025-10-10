//! Mycnet Consensus - Byzantine Fault Tolerant consensus system

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// BFT consensus operation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsensusOperation {
    /// Network configuration changes requiring full BFT
    NetworkConfiguration {
        config_type: String,
        proposed_change: Vec<u8>,
    },
    /// Trust score modifications
    TrustScoreModification {
        target_node: Uuid,
        score_change: f32,
        justification: String,
    },
    /// Node admission to network
    NodeAdmission {
        candidate_node: Uuid,
        admission_criteria: Vec<u8>,
    },
    /// Service deployment decisions (authority hierarchy)
    ServiceDeployment {
        service_spec: Vec<u8>,
        deployment_strategy: String,
    },
}

/// Consensus result with trust implications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusResult {
    pub operation_id: Uuid,
    pub result: ConsensusOutcome,
    pub participating_nodes: Vec<Uuid>,
    pub trust_adjustments: HashMap<Uuid, f32>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Outcome of consensus operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsensusOutcome {
    Approved,
    Rejected { reason: String },
    Timeout,
}

/// Trust scoring system for consensus participants
pub struct TrustScoring {
    trust_scores: HashMap<Uuid, f32>,
    participation_history: HashMap<Uuid, Vec<ParticipationRecord>>,
}

/// Record of node participation in consensus
#[derive(Debug, Clone)]
pub struct ParticipationRecord {
    pub operation_id: Uuid,
    pub participated: bool,
    pub correct_vote: bool,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// BFT consensus engine using commit-reveal protocol
pub struct BFTConsensusEngine {
    node_id: Uuid,
    trust_scoring: TrustScoring,
    active_operations: HashMap<Uuid, ConsensusOperation>,
    quorum_threshold: f32,
}

impl BFTConsensusEngine {
    /// Create new consensus engine
    pub fn new(node_id: Uuid) -> Self {
        Self {
            node_id,
            trust_scoring: TrustScoring::new(),
            active_operations: HashMap::new(),
            quorum_threshold: 0.67, // 2/3 majority
        }
    }
    
    /// Propose a new consensus operation
    pub async fn propose_operation(&mut self, operation: ConsensusOperation) -> Result<Uuid, Box<dyn std::error::Error>> {
        let operation_id = Uuid::new_v4();
        
        tracing::info!("Proposing consensus operation: {:?}", operation_id);
        
        // Store operation
        self.active_operations.insert(operation_id, operation);
        
        // Begin commit-reveal protocol
        self.begin_commit_phase(operation_id).await?;
        
        Ok(operation_id)
    }
    
    /// Begin commit phase of commit-reveal protocol
    async fn begin_commit_phase(&self, operation_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        tracing::debug!("Beginning commit phase for operation: {:?}", operation_id);
        
        // In real implementation:
        // 1. Nodes submit cryptographic hashes of their results
        // 2. Wait for all commits to be received
        // 3. Move to reveal phase
        
        Ok(())
    }
    
    /// Execute reveal phase and determine consensus
    pub async fn execute_reveal_phase(&mut self, operation_id: Uuid) -> Result<ConsensusResult, Box<dyn std::error::Error>> {
        tracing::debug!("Executing reveal phase for operation: {:?}", operation_id);
        
        // In real implementation:
        // 1. Nodes reveal their actual results
        // 2. Majority result is accepted as canonical
        // 3. Trust scores are updated based on participation
        
        let result = ConsensusResult {
            operation_id,
            result: ConsensusOutcome::Approved,
            participating_nodes: vec![self.node_id],
            trust_adjustments: HashMap::new(),
            timestamp: chrono::Utc::now(),
        };
        
        Ok(result)
    }
}

impl TrustScoring {
    /// Create new trust scoring system
    pub fn new() -> Self {
        Self {
            trust_scores: HashMap::new(),
            participation_history: HashMap::new(),
        }
    }
    
    /// Update trust score based on consensus participation
    pub fn update_trust_score(&mut self, node_id: Uuid, participation: ParticipationRecord) {
        let current_score = self.trust_scores.get(&node_id).copied().unwrap_or(0.5);
        
        let adjustment = if participation.participated {
            if participation.correct_vote {
                0.01 // Small increase for correct participation
            } else {
                -0.05 // Penalty for incorrect vote
            }
        } else {
            -0.02 // Small penalty for non-participation
        };
        
        let new_score = (current_score + adjustment).clamp(0.0, 1.0);
        self.trust_scores.insert(node_id, new_score);
        
        // Record participation history
        self.participation_history
            .entry(node_id)
            .or_insert_with(Vec::new)
            .push(participation);
    }
    
    /// Get current trust score for a node
    pub fn get_trust_score(&self, node_id: &Uuid) -> f32 {
        self.trust_scores.get(node_id).copied().unwrap_or(0.5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_consensus_engine_creation() {
        let node_id = Uuid::new_v4();
        let engine = BFTConsensusEngine::new(node_id);
        assert_eq!(engine.node_id, node_id);
        assert_eq!(engine.quorum_threshold, 0.67);
    }
    
    #[test]
    fn test_trust_scoring() {
        let mut trust_scoring = TrustScoring::new();
        let node_id = Uuid::new_v4();
        
        let participation = ParticipationRecord {
            operation_id: Uuid::new_v4(),
            participated: true,
            correct_vote: true,
            timestamp: chrono::Utc::now(),
        };
        
        trust_scoring.update_trust_score(node_id, participation);
        assert!(trust_scoring.get_trust_score(&node_id) > 0.5);
    }
}