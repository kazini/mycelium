//! Mycnet Security - Cryptographic security and trust management

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Network identity with cryptographic isolation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkIdentity {
    pub network_id: Uuid,
    pub network_name: String,
    pub isolation_key: [u8; 32],
    pub genesis_timestamp: chrono::DateTime<chrono::Utc>,
    pub genesis_nodes: Vec<Uuid>,
}

/// Node authentication credentials
#[derive(Debug, Clone)]
pub struct NodeCredentials {
    pub node_id: Uuid,
    pub signing_keypair: ed25519_dalek::Keypair,
    pub encryption_keypair: x25519_dalek::StaticSecret,
    pub network_membership_proof: Vec<u8>,
}

/// Trust management system
pub struct TrustManager {
    trust_scores: HashMap<Uuid, TrustScore>,
    trust_policies: Vec<TrustPolicy>,
    consensus_participation: HashMap<Uuid, ParticipationMetrics>,
}

/// Trust score with components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustScore {
    pub overall_score: f32,
    pub consensus_participation: f32,
    pub network_contribution: f32,
    pub uptime_reliability: f32,
    pub security_compliance: f32,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// Trust policy for access control
#[derive(Debug, Clone)]
pub struct TrustPolicy {
    pub policy_id: Uuid,
    pub minimum_trust_score: f32,
    pub required_capabilities: Vec<String>,
    pub access_level: AccessLevel,
}

/// Access levels based on trust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessLevel {
    Full,           // Full network access
    Standard,       // Standard operations
    Limited,        // Limited operations only
    ReadOnly,       // Read-only access
    Restricted,     // Heavily restricted
}

/// Participation metrics for trust calculation
#[derive(Debug, Clone)]
pub struct ParticipationMetrics {
    pub total_consensus_rounds: u64,
    pub successful_participations: u64,
    pub correct_votes: u64,
    pub network_uptime_hours: f64,
    pub last_participation: chrono::DateTime<chrono::Utc>,
}

/// Secure channel for encrypted communication
pub struct SecureChannel {
    local_secret: x25519_dalek::StaticSecret,
    remote_public: x25519_dalek::PublicKey,
    shared_secret: [u8; 32],
    cipher: chacha20poly1305::ChaCha20Poly1305,
}

/// Authentication manager
pub struct AuthenticationManager {
    network_identity: NetworkIdentity,
    node_credentials: NodeCredentials,
    trusted_nodes: HashMap<Uuid, ed25519_dalek::PublicKey>,
}

impl NetworkIdentity {
    /// Create new network identity for genesis
    pub fn new_genesis(network_name: String, genesis_nodes: Vec<Uuid>) -> Self {
        let mut isolation_key = [0u8; 32];
        getrandom::getrandom(&mut isolation_key).expect("Failed to generate isolation key");
        
        Self {
            network_id: Uuid::new_v4(),
            network_name,
            isolation_key,
            genesis_timestamp: chrono::Utc::now(),
            genesis_nodes,
        }
    }
    
    /// Validate network membership cryptographically
    pub fn validate_membership(&self, node_proof: &[u8], signature: &ed25519_dalek::Signature, public_key: &ed25519_dalek::PublicKey) -> bool {
        use ed25519_dalek::Verifier;
        
        // Verify signature
        if public_key.verify(node_proof, signature).is_err() {
            return false;
        }
        
        // Verify network membership proof contains isolation key
        let expected_proof = blake3::hash(&self.isolation_key);
        let provided_proof = blake3::hash(node_proof);
        
        expected_proof == provided_proof
    }
}

impl NodeCredentials {
    /// Generate new node credentials for network
    pub fn generate_for_network(network_identity: &NetworkIdentity) -> Self {
        let mut csprng = rand::rngs::OsRng;
        
        let signing_keypair = ed25519_dalek::Keypair::generate(&mut csprng);
        let encryption_keypair = x25519_dalek::StaticSecret::new(&mut csprng);
        
        // Create network membership proof
        let mut proof_data = Vec::new();
        proof_data.extend_from_slice(&network_identity.isolation_key);
        proof_data.extend_from_slice(signing_keypair.public.as_bytes());
        
        let network_membership_proof = blake3::hash(&proof_data).as_bytes().to_vec();
        
        Self {
            node_id: Uuid::new_v4(),
            signing_keypair,
            encryption_keypair,
            network_membership_proof,
        }
    }
    
    /// Sign a message with node's signing key
    pub fn sign_message(&self, message: &[u8]) -> ed25519_dalek::Signature {
        use ed25519_dalek::Signer;
        self.signing_keypair.sign(message)
    }
    
    /// Get public signing key
    pub fn public_signing_key(&self) -> ed25519_dalek::PublicKey {
        self.signing_keypair.public
    }
    
    /// Get public encryption key
    pub fn public_encryption_key(&self) -> x25519_dalek::PublicKey {
        x25519_dalek::PublicKey::from(&self.encryption_keypair)
    }
}

impl TrustManager {
    /// Create new trust manager
    pub fn new() -> Self {
        Self {
            trust_scores: HashMap::new(),
            trust_policies: Vec::new(),
            consensus_participation: HashMap::new(),
        }
    }
    
    /// Evaluate node trust score
    pub fn evaluate_trust(&mut self, node_id: Uuid) -> TrustScore {
        let participation = self.consensus_participation.get(&node_id);
        
        let consensus_score = if let Some(metrics) = participation {
            if metrics.total_consensus_rounds > 0 {
                (metrics.correct_votes as f32) / (metrics.total_consensus_rounds as f32)
            } else {
                0.5 // Neutral score for new nodes
            }
        } else {
            0.5
        };
        
        let uptime_score = if let Some(metrics) = participation {
            (metrics.network_uptime_hours / (24.0 * 30.0)).min(1.0) // Max 30 days
        } else {
            0.5
        };
        
        let trust_score = TrustScore {
            overall_score: (consensus_score + uptime_score) / 2.0,
            consensus_participation: consensus_score,
            network_contribution: 0.5, // Placeholder
            uptime_reliability: uptime_score,
            security_compliance: 1.0, // Placeholder
            last_updated: chrono::Utc::now(),
        };
        
        self.trust_scores.insert(node_id, trust_score.clone());
        trust_score
    }
    
    /// Update consensus participation metrics
    pub fn update_consensus_participation(&mut self, node_id: Uuid, participated: bool, correct_vote: bool) {
        let metrics = self.consensus_participation.entry(node_id).or_insert(ParticipationMetrics {
            total_consensus_rounds: 0,
            successful_participations: 0,
            correct_votes: 0,
            network_uptime_hours: 0.0,
            last_participation: chrono::Utc::now(),
        });
        
        metrics.total_consensus_rounds += 1;
        if participated {
            metrics.successful_participations += 1;
        }
        if correct_vote {
            metrics.correct_votes += 1;
        }
        metrics.last_participation = chrono::Utc::now();
    }
    
    /// Check if node meets trust policy requirements
    pub fn check_access_permission(&self, node_id: &Uuid, required_access: AccessLevel) -> bool {
        if let Some(trust_score) = self.trust_scores.get(node_id) {
            let required_score = match required_access {
                AccessLevel::Full => 0.9,
                AccessLevel::Standard => 0.7,
                AccessLevel::Limited => 0.5,
                AccessLevel::ReadOnly => 0.3,
                AccessLevel::Restricted => 0.1,
            };
            
            trust_score.overall_score >= required_score
        } else {
            false // No trust score = no access
        }
    }
}

impl SecureChannel {
    /// Establish secure channel with remote node
    pub fn establish(local_secret: x25519_dalek::StaticSecret, remote_public: x25519_dalek::PublicKey) -> Result<Self, Box<dyn std::error::Error>> {
        let shared_secret = local_secret.diffie_hellman(&remote_public);
        
        // Derive encryption key from shared secret
        let key_material = blake3::hash(shared_secret.as_bytes());
        let key = chacha20poly1305::Key::from_slice(key_material.as_bytes());
        let cipher = chacha20poly1305::ChaCha20Poly1305::new(key);
        
        Ok(Self {
            local_secret,
            remote_public,
            shared_secret: *shared_secret.as_bytes(),
            cipher,
        })
    }
    
    /// Encrypt message for secure transmission
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        use chacha20poly1305::{AeadInPlace, Nonce};
        
        let mut buffer = plaintext.to_vec();
        let nonce = Nonce::from_slice(&[0u8; 12]); // In real implementation, use random nonce
        
        self.cipher.encrypt_in_place(nonce, b"", &mut buffer)
            .map_err(|e| format!("Encryption failed: {:?}", e))?;
        
        Ok(buffer)
    }
    
    /// Decrypt received message
    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        use chacha20poly1305::{AeadInPlace, Nonce};
        
        let mut buffer = ciphertext.to_vec();
        let nonce = Nonce::from_slice(&[0u8; 12]); // In real implementation, extract nonce
        
        self.cipher.decrypt_in_place(nonce, b"", &mut buffer)
            .map_err(|e| format!("Decryption failed: {:?}", e))?;
        
        Ok(buffer)
    }
}

impl AuthenticationManager {
    /// Create new authentication manager
    pub fn new(network_identity: NetworkIdentity, node_credentials: NodeCredentials) -> Self {
        Self {
            network_identity,
            node_credentials,
            trusted_nodes: HashMap::new(),
        }
    }
    
    /// Authenticate remote node
    pub fn authenticate_node(&mut self, node_id: Uuid, public_key: ed25519_dalek::PublicKey, membership_proof: &[u8], signature: &ed25519_dalek::Signature) -> bool {
        // Validate network membership
        if !self.network_identity.validate_membership(membership_proof, signature, &public_key) {
            return false;
        }
        
        // Add to trusted nodes
        self.trusted_nodes.insert(node_id, public_key);
        true
    }
    
    /// Create authentication challenge for remote node
    pub fn create_auth_challenge(&self) -> Vec<u8> {
        let mut challenge = Vec::new();
        challenge.extend_from_slice(&self.network_identity.network_id.as_bytes());
        challenge.extend_from_slice(&chrono::Utc::now().timestamp().to_be_bytes());
        challenge
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_network_identity_creation() {
        let genesis_nodes = vec![Uuid::new_v4()];
        let network = NetworkIdentity::new_genesis("test-network".to_string(), genesis_nodes);
        assert_eq!(network.network_name, "test-network");
        assert_ne!(network.isolation_key, [0u8; 32]);
    }
    
    #[test]
    fn test_node_credentials_generation() {
        let network = NetworkIdentity::new_genesis("test".to_string(), vec![]);
        let credentials = NodeCredentials::generate_for_network(&network);
        assert_ne!(credentials.node_id, Uuid::nil());
        assert!(!credentials.network_membership_proof.is_empty());
    }
    
    #[test]
    fn test_trust_manager() {
        let mut trust_manager = TrustManager::new();
        let node_id = Uuid::new_v4();
        
        let trust_score = trust_manager.evaluate_trust(node_id);
        assert_eq!(trust_score.overall_score, 0.5); // Default score for new nodes
    }
}