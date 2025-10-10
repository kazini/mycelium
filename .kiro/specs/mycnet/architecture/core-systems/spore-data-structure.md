# Spore Data Structure and Content Architecture

## Overview

Spores function as specialized databases with specific entry limits and structured content, implementing a comprehensive data model for network discovery and coordination.

## Spore Data Architecture

### Core Data Structure

```rust
struct SporeData {
    // Core spore identification
    spore_type: SporeType,           // Master or Service spore
    network_identity: NetworkIdentity,
    
    // Node information with entry limits
    recent_node_locations: Vec<NodeLocation>,     // Limited to X entries
    node_health: HashMap<NodeId, HealthStatus>,
    node_trust: HashMap<NodeId, TrustScore>,
    node_reputation: HashMap<NodeId, ReputationScore>,
    node_usage: HashMap<NodeId, UsageMetrics>,
    
    // Activity tracking within configurable timeframe
    last_read_writes: HashMap<NodeId, ReadWriteEntry>,
    connection_disconnection_log: Vec<ConnectionEvent>,
    uptime_differentials: HashMap<NodeId, Duration>,
    processing_volumes: HashMap<NodeId, u64>,
    
    // P2P network entry points (limited entries)
    p2p_rhizomorph_entries: Vec<RhizomorphEntry>,  // Limited to X amount
    
    // Metadata
    timeframe_config: TimeframeConfig,
    last_update: Timestamp,
    signature: SporeSignature,
}

struct ConnectionEvent {
    node_id: NodeId,
    event_type: ConnectionEventType,
    timestamp: Timestamp,
    intermittency_tag: Option<IntermittencyPeriod>,  // For frequent connect/disconnect
}

enum ConnectionEventType {
    Connected,
    Disconnected,
    IntermittencyPeriod {
        start_time: Timestamp,
        end_time: Timestamp,
        event_count: u32,
    },
}
```

## Spore Content Principles

### 1. Database-Like Structure
Spores contain structured entries with specific thresholds:
- **Structured Entries**: Each data type has defined structure and validation rules
- **Entry Limits**: Maximum entry counts for each data type (X Rhizomorph entries, etc.)
- **Data Integrity**: Cryptographic signatures and integrity checking
- **Version Control**: Versioning for spore data evolution

### 2. Entry Limits and Thresholds
Each data type has maximum entry counts:
- **Node Locations**: Limited to prevent unbounded growth
- **Connection Events**: Capped with intermittency condensation
- **P2P Entries**: Fixed maximum for Rhizomorph discovery points
- **Health Metrics**: Time-bounded with automatic cleanup

### 3. Timeframe Scoping
All temporal data limited to configurable timeframes:
- **Connection History**: Configurable duration for connection tracking
- **Activity Periods**: Bounded time windows for activity analysis
- **Health Data**: Time-limited health and performance metrics
- **Trust Evolution**: Historical trust data within defined periods

### 4. Intermittency Handling
Frequent connection/disconnection periods tagged and condensed:
- **Pattern Detection**: Automatic detection of intermittent connection patterns
- **Event Condensation**: Multiple events compressed into summary periods
- **Efficiency**: Reduces spore size while preserving essential information
- **Analysis**: Enables analysis of connection stability patterns

### 5. Health/Trust Integration
Connection patterns affect health and trust calculations:
- **Behavioral Analysis**: Connection patterns influence trust scores
- **Reliability Metrics**: Uptime and availability tracking
- **Performance Correlation**: Connection quality affects health ratings
- **Reputation Building**: Long-term behavior patterns build reputation

## Spore Content Details

### Node Information
- **Recent node locations, health, trust, reputation and usage**
- **Last entries for read and writes per node**
- **Connection/disconnection times within timeframe**
- **Uptime differentials and processing volumes**
- **Limited P2P network entry points**

### Size Management Strategy
- **Entry Thresholds**: Each data type has maximum entry limits
- **Scope Limitation**: Data kept within relevant scope and timeframe
- **Intermittency Condensation**: Frequent events tagged rather than logged individually
- **No Compression**: Size managed through data structure limits, not compression algorithms

## Spore Validation and Conflict Resolution

### Core Validation Principle
Each node independently validates spore data against authoritative sources before merging, with different validation rules for different spore types.

### Spore-Specific Validation Strategy

1. **Primary Spore**: Always authoritative (result of consensus) - no validation needed
2. **Seed Spore**: Unidirectional, not merged unless connections confirmed by handling nodes
3. **Latent Spore**: Validated against Primary Spore before merging

### Validation Implementation

```rust
struct SporeValidator {
    primary_spore_reference: PrimarySporeReference,
    connection_validator: ConnectionValidator,
    tampering_detector: TamperingDetector,
}

impl SporeValidator {
    async fn validate_and_resolve_conflicts(&self, local_spore: SporeData, remote_spore: SporeData) -> Result<SporeData> {
        match (local_spore.spore_type, remote_spore.spore_type) {
            // Primary Spore is always authoritative
            (SporeType::Primary, _) => Ok(local_spore),
            (_, SporeType::Primary) => Ok(remote_spore),
            
            // Seed Spore validation - unidirectional unless confirmed
            (SporeType::Seed, SporeType::Seed) => {
                self.validate_seed_spore_conflicts(local_spore, remote_spore).await
            },
            
            // Latent Spore validation against Primary
            (SporeType::Latent, SporeType::Latent) => {
                self.validate_latent_spore_conflicts(local_spore, remote_spore).await
            },
            
            _ => Err(SporeError::IncompatibleSporeTypes),
        }
    }
    
    async fn validate_seed_spore_conflicts(&self, local_seed: SporeData, remote_seed: SporeData) -> Result<SporeData> {
        // Seed spores are NOT merged - validate connections independently
        let mut validated_spore = local_seed.clone();
        
        // Check each connection in remote seed spore
        for node_entry in &remote_seed.recent_node_locations {
            // Only include if connection confirmed by handling node
            if self.connection_validator.confirm_node_connection(&node_entry.node_id).await? {
                // Check against Primary Spore for validity
                if self.primary_spore_reference.validate_node_entry(node_entry).await? {
                    validated_spore.recent_node_locations.push(node_entry.clone());
                }
            }
        }
        
        // Enforce entry limits and remove outdated/irrelevant data
        self.enforce_validation_limits(&mut validated_spore)?;
        
        Ok(validated_spore)
    }
    
    async fn validate_latent_spore_conflicts(&self, local_latent: SporeData, remote_latent: SporeData) -> Result<SporeData> {
        // Validate Latent Spore data against Primary Spore
        let primary_reference = self.primary_spore_reference.get_current_state().await?;
        
        let mut validated_entries = Vec::new();
        
        // Validate local entries
        for entry in &local_latent.recent_node_locations {
            if self.validate_entry_against_primary(entry, &primary_reference).await? {
                validated_entries.push(entry.clone());
            }
        }
        
        // Validate remote entries
        for entry in &remote_latent.recent_node_locations {
            if self.validate_entry_against_primary(entry, &primary_reference).await? {
                validated_entries.push(entry.clone());
            }
        }
        
        // Remove duplicates and enforce limits
        validated_entries.dedup_by_key(|entry| entry.node_id);
        validated_entries.truncate(self.config.max_latent_entries);
        
        Ok(SporeData {
            recent_node_locations: validated_entries,
            ..local_latent
        })
    }
    
    async fn detect_tampering(&self, spore: &SporeData) -> Result<bool> {
        // Check for signs of tampering
        // - Invalid signatures
        // - Impossible timestamps
        // - Inconsistent node data
        // - Suspicious connection patterns
        
        if !self.verify_spore_signature(spore)? {
            return Ok(true);
        }
        
        if self.detect_impossible_timestamps(spore)? {
            return Ok(true);
        }
        
        if self.detect_inconsistent_data(spore)? {
            return Ok(true);
        }
        
        Ok(false)
    }
}
```

## Refined Merging Strategy

### Validation and Discard Policies

1. **Validate First**: Check spore data validity before any merging
2. **Authority Hierarchy**: Primary > Seed > Latent for conflict resolution
3. **Seed Spore Unidirectional**: Only merge confirmed connections
4. **Latent Spore Validation**: Check against Primary Spore before merging
5. **Tampering Detection**: Discard suspicious or invalid data
6. **Connection Confirmation**: Verify node connections before including in spores
7. **Fallback to Latent**: When no Primary/Seed available, use validated Latent Spore network

## Comprehensive Spore System Architecture

### Spore Type Definitions

```rust
#[derive(Serialize, Deserialize, Clone)]
enum SporeType {
    MasterPrimary { consensus_leader: NodeId },
    MasterSeed { locations: Vec<SeedLocation>, priority_order: Vec<usize> },
    MasterLatent { p2p_peers: Vec<PeerId> },
    ServicePrimary { service_id: ServiceId, consensus_leader: NodeId },
    ServiceSeed { service_id: ServiceId, stored_in_global_db: bool },
    ServiceLatent { service_id: ServiceId, p2p_peers: Vec<PeerId> },
}

#[derive(Serialize, Deserialize, Clone)]
struct NodeEntry {
    node_id: NodeId,
    addresses: Vec<NetworkAddress>,
    health_status: HealthStatus,
    trust_score: TrustScore,
    reputation: ReputationScore,
    usage_metrics: UsageMetrics,
    last_read: Timestamp,
    last_write: Timestamp,
    connection_time: Option<Timestamp>,
    disconnection_time: Option<Timestamp>,
    uptime_differential: Duration,
    processing_volume: u64,
}
```

### Spore Manager Implementation

```rust
struct SporeManager {
    primary_spores: HashMap<SporeScope, PrimarySpore>,
    seed_spores: HashMap<SporeScope, Vec<SeedSpore>>,
    latent_spores: HashMap<SporeScope, LatentSpore>,
    update_config: SporeUpdateConfig,
}

impl SporeManager {
    async fn resolve_spore_conflicts(&self, local_spore: SporeData, remote_spore: SporeData) -> Result<SporeData> {
        // ALWAYS merge spore data while keeping scope limited and size controlled through entry limits
        let mut merged_spore = local_spore.clone();
        
        // Merge node information (respect entry limits)
        merged_spore.recent_node_locations = self.merge_with_entry_limit(
            &local_spore.recent_node_locations,
            &remote_spore.recent_node_locations,
            self.config.max_node_entries,
        )?;
        
        // Merge health, trust, reputation, usage data
        merged_spore.node_health = self.merge_node_metrics(
            &local_spore.node_health,
            &remote_spore.node_health,
        )?;
        
        merged_spore.node_trust = self.merge_node_metrics(
            &local_spore.node_trust,
            &remote_spore.node_trust,
        )?;
        
        // Merge connection/disconnection logs within timeframe
        merged_spore.connection_disconnection_log = self.merge_connection_events(
            &local_spore.connection_disconnection_log,
            &remote_spore.connection_disconnection_log,
            &merged_spore.timeframe_config,
        )?;
        
        // Merge P2P entry points (respect X entry limit)
        merged_spore.p2p_rhizomorph_entries = self.merge_with_entry_limit(
            &local_spore.p2p_rhizomorph_entries,
            &remote_spore.p2p_rhizomorph_entries,
            self.config.max_p2p_entries,
        )?;
        
        // Merge uptime differentials and processing volumes
        merged_spore.uptime_differentials = self.merge_node_metrics(
            &local_spore.uptime_differentials,
            &remote_spore.uptime_differentials,
        )?;
        
        merged_spore.processing_volumes = self.merge_node_metrics(
            &local_spore.processing_volumes,
            &remote_spore.processing_volumes,
        )?;
        
        // Update timestamp and ensure entry limits maintained
        merged_spore.last_update = Timestamp::now();
        self.enforce_entry_limits(&mut merged_spore)?;
        
        Ok(merged_spore)
    }
    
    fn merge_with_entry_limit<T>(&self, local_entries: &Vec<T>, remote_entries: &Vec<T>, max_entries: usize) -> Result<Vec<T>> 
    where T: Clone + HasTimestamp + Ord {
        // Merge entries and keep only the most recent up to max_entries limit
        let mut all_entries = local_entries.clone();
        all_entries.extend(remote_entries.clone());
        
        // Sort by timestamp (most recent first) and take max_entries
        all_entries.sort_by(|a, b| b.timestamp().cmp(&a.timestamp()));
        all_entries.truncate(max_entries);
        
        Ok(all_entries)
    }
    
    fn merge_connection_events(&self, local_events: &Vec<ConnectionEvent>, remote_events: &Vec<ConnectionEvent>, timeframe: &TimeframeConfig) -> Result<Vec<ConnectionEvent>> {
        // Merge connection events, condensing intermittency periods
        let mut all_events = local_events.clone();
        all_events.extend(remote_events.clone());
        
        // Filter to timeframe
        let cutoff_time = Timestamp::now() - timeframe.connection_history_duration;
        all_events.retain(|event| event.timestamp >= cutoff_time);
        
        // Condense intermittency periods
        let condensed_events = self.condense_intermittency_periods(&all_events)?;
        
        Ok(condensed_events)
    }
    
    async fn evaluate_node_readiness_from_spore(&self, spore: &SporeData) -> Result<NodeReadinessDecision> {
        // Implement the differential uptime and processing logic
        let my_node_entry = spore.recent_nodes.iter()
            .find(|entry| entry.node_id == self.node_id)
            .ok_or(SporeError::NodeNotFound)?;
        
        let other_active_nodes: Vec<_> = spore.recent_nodes.iter()
            .filter(|entry| entry.node_id != self.node_id)
            .filter(|entry| self.is_potentially_active(entry))
            .collect();
        
        if other_active_nodes.is_empty() {
            // No other active nodes detected
            return Ok(NodeReadinessDecision::CanEnterWriteMode);
        }
        
        // Check differential uptime and processing
        let max_other_processing = other_active_nodes.iter()
            .map(|entry| entry.processing_volume)
            .max()
            .unwrap_or(0);
        
        let max_other_uptime = other_active_nodes.iter()
            .map(|entry| entry.uptime_differential)
            .max()
            .unwrap_or(Duration::ZERO);
        
        if my_node_entry.processing_volume < max_other_processing ||
           my_node_entry.uptime_differential < max_other_uptime {
            // Other nodes have done more work - enter read-only and wait
            Ok(NodeReadinessDecision::EnterReadOnlyWait {
                wait_duration: self.calculate_wait_duration(&other_active_nodes),
                manual_override_available: true,
            })
        } else {
            // This node has done the most work - can continue
            Ok(NodeReadinessDecision::CanEnterWriteMode)
        }
    }
}
```

## Differential Uptime and Processing Logic

### Node Readiness Evaluation
- Nodes compare uptime differentials and processing volumes from merged spore data
- Nodes with less activity enter read-only mode and wait for sync
- Manual override available after timeout periods
- Automatic write-mode entry when no other active nodes detected
- Decision based on processing volume and uptime differential comparisons

This comprehensive spore data structure provides the foundation for reliable network discovery, coordination, and split-brain resolution while maintaining efficient data management and validation.