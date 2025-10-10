# Decentralized Update System Architecture

## Overview

The Digital Mycelium Network implements a sophisticated decentralized update system that solves the "bootstrap update problem" - how to update the core components that run the update system itself.

## The Bootstrap Update Problem

### Challenge
How do you update the core components that run the update system itself?

### Solution
Multi-layer update architecture with redundant update agents and network coordination.

## Update Architecture Layers

### Layer 1: Hardware-Level Update Agent
```rust
struct NodeUpdateAgent {
    agent_binary: PathBuf,           // Self-contained binary
    update_workspace: PathBuf,       // Isolated update directory
    rollback_snapshots: Vec<Snapshot>, // System state snapshots
    network_coordinator: UpdateCoordinator,
}
```

### Layer 2: Network-Coordinated Update Orchestration
```rust
struct NetworkUpdateOrchestrator {
    consensus_engine: BFTConsensus,
    update_distribution: UpdateDistribution,
    rollback_coordinator: RollbackCoordinator,
    health_monitor: HealthMonitor,
}
```

### Layer 3: Component-Specific Update Handlers
```rust
struct ComponentUpdateManager {
    k3s_updater: K3sUpdater,
    mycelium_core_updater: CoreUpdater,
    container_updater: ContainerUpdater,
    config_updater: ConfigUpdater,
}
```

## Decentralized Update Process

### Phase 1: Update Preparation
```rust
impl NetworkUpdateOrchestrator {
    async fn prepare_network_update(&self, update: UpdatePackage) -> Result<UpdatePlan> {
        // 1. Validate update package cryptographically
        self.validate_update_signature(&update)?;
        
        // 2. Create update plan with node coordination
        let plan = self.create_rolling_update_plan(&update).await?;
        
        // 3. Distribute update package to all nodes
        self.distribute_update_package(&update, &plan).await?;
        
        // 4. Wait for all nodes to confirm readiness
        self.wait_for_node_readiness(&plan).await?;
        
        Ok(plan)
    }
}
```

### Phase 2: Rolling Update Execution
```rust
impl NodeUpdateAgent {
    async fn execute_core_update(&self, update: CoreUpdate) -> Result<()> {
        // 1. Create system snapshot for rollback
        let snapshot = self.create_system_snapshot().await?;
        
        // 2. Download and verify update files
        let update_files = self.download_update_files(&update).await?;
        self.verify_update_integrity(&update_files)?;
        
        // 3. Coordinate with network for update window
        let update_slot = self.request_update_slot().await?;
        
        // 4. Execute the "bootstrap swap" procedure
        self.execute_bootstrap_swap(&update_files, snapshot).await?;
        
        Ok(())
    }
    
    async fn execute_bootstrap_swap(&self, update_files: &UpdateFiles, snapshot: Snapshot) -> Result<()> {
        // This is the critical "engine swap" procedure
        
        // 1. Prepare new environment in parallel workspace
        let new_workspace = self.prepare_update_workspace(&update_files).await?;
        
        // 2. Start new K3s/Mycelium stack in new workspace
        let new_stack = self.start_new_stack(&new_workspace).await?;
        
        // 3. Migrate network state to new stack
        self.migrate_network_state(&new_stack).await?;
        
        // 4. Atomic swap: redirect traffic to new stack
        self.atomic_traffic_swap(&new_stack).await?;
        
        // 5. Gracefully shutdown old stack
        self.shutdown_old_stack().await?;
        
        // 6. Cleanup old workspace (keep snapshot for rollback)
        self.cleanup_old_workspace(snapshot).await?;
        
        Ok(())
    }
}
```

## Update Coordination Strategies

### Strategy 1: Rolling Update with Quorum Maintenance
```rust
struct RollingUpdatePlan {
    update_groups: Vec<NodeGroup>,  // Groups of nodes to update together
    quorum_requirements: QuorumSpec, // Minimum nodes needed for network operation
    rollback_triggers: Vec<RollbackTrigger>,
}

impl RollingUpdatePlan {
    fn ensure_quorum_maintained(&self) -> bool {
        // Never update more nodes than would break quorum
        // Always maintain minimum Sclerotia for network operation
        self.update_groups.iter().all(|group| {
            self.remaining_nodes_after_group_update(group) >= self.quorum_requirements.minimum
        })
    }
}
```

### Strategy 2: Blue-Green Network Deployment
```rust
struct BlueGreenUpdate {
    blue_network: NetworkInstance,   // Current running network
    green_network: NetworkInstance,  // New network being prepared
    migration_coordinator: MigrationCoordinator,
}

impl BlueGreenUpdate {
    async fn execute_blue_green_update(&self, update: NetworkUpdate) -> Result<()> {
        // 1. Prepare green network with new components
        self.prepare_green_network(&update).await?;
        
        // 2. Gradually migrate services from blue to green
        self.migrate_services_gradually().await?;
        
        // 3. Switch client traffic to green network
        self.switch_client_traffic().await?;
        
        // 4. Decommission blue network
        self.decommission_blue_network().await?;
        
        Ok(())
    }
}
```

## Self-Updating Update Agent

### Agent Self-Update Process
```rust
struct SelfUpdatingAgent {
    current_binary: PathBuf,
    backup_binary: PathBuf,
    update_binary: PathBuf,
}

impl SelfUpdatingAgent {
    async fn self_update(&self, new_agent: AgentUpdate) -> Result<()> {
        // 1. Download new agent binary
        let new_binary = self.download_new_agent(&new_agent).await?;
        
        // 2. Verify new agent signature and integrity
        self.verify_agent_integrity(&new_binary)?;
        
        // 3. Backup current agent
        std::fs::copy(&self.current_binary, &self.backup_binary)?;
        
        // 4. Atomic replacement of agent binary
        std::fs::rename(&new_binary, &self.update_binary)?;
        std::fs::rename(&self.update_binary, &self.current_binary)?;
        
        // 5. Restart with new agent (exec replacement)
        self.exec_restart_with_new_agent()?;
        
        Ok(())
    }
}
```

## Rollback and Recovery Mechanisms

### Automatic Rollback System
```rust
struct RollbackCoordinator {
    health_monitors: Vec<HealthMonitor>,
    rollback_triggers: Vec<RollbackTrigger>,
    snapshot_manager: SnapshotManager,
}

impl RollbackCoordinator {
    async fn monitor_update_health(&self, update_id: UpdateId) -> Result<()> {
        let health_window = Duration::from_secs(300); // 5 minute validation window
        let start_time = Instant::now();
        
        while start_time.elapsed() < health_window {
            // Check network health metrics
            let health = self.check_network_health().await?;
            
            if health.is_critical_failure() {
                // Immediate rollback on critical failure
                self.execute_emergency_rollback(update_id).await?;
                return Err(UpdateError::RolledBackDueToCriticalFailure);
            }
            
            if health.is_degraded() && start_time.elapsed() > Duration::from_secs(60) {
                // Rollback if degraded for more than 1 minute
                self.execute_rollback(update_id).await?;
                return Err(UpdateError::RolledBackDueToDegradation);
            }
            
            tokio::time::sleep(Duration::from_secs(10)).await;
        }
        
        // Update successful, commit changes
        self.commit_update(update_id).await?;
        Ok(())
    }
}
```

## Network-Orchestrated Update Commands

### Update Command Interface
```rust
enum NetworkUpdateCommand {
    InitiateCoreUpdate {
        update_package: UpdatePackage,
        rollout_strategy: RolloutStrategy,
        approval_threshold: f32, // Percentage of nodes that must approve
    },
    
    EmergencyRollback {
        target_version: Version,
        affected_nodes: Vec<NodeId>,
    },
    
    PauseUpdate {
        update_id: UpdateId,
        reason: String,
    },
    
    ResumeUpdate {
        update_id: UpdateId,
    },
}

impl NetworkUpdateManager {
    async fn handle_admin_update_command(&self, command: NetworkUpdateCommand) -> Result<UpdateResponse> {
        match command {
            NetworkUpdateCommand::InitiateCoreUpdate { update_package, rollout_strategy, approval_threshold } => {
                // 1. Validate admin permissions
                self.validate_update_permissions().await?;
                
                // 2. Distribute update proposal to network
                let proposal = self.create_update_proposal(&update_package, &rollout_strategy).await?;
                
                // 3. Collect node approvals via BFT consensus
                let approval_result = self.collect_node_approvals(&proposal, approval_threshold).await?;
                
                // 4. Execute update if approved
                if approval_result.approved {
                    self.execute_network_update(&proposal).await?;
                }
                
                Ok(UpdateResponse::UpdateInitiated { proposal_id: proposal.id })
            },
            // ... handle other commands
        }
    }
}
```

## Implementation Phases

### Phase 1: Basic Update Agent
- Node-local update agent outside containers
- Simple file replacement and restart capability
- Basic rollback using filesystem snapshots

### Phase 2: Network Coordination
- BFT consensus for update approval
- Rolling update coordination
- Health monitoring and automatic rollback

### Phase 3: Advanced Update Strategies
- Blue-green network deployments
- Zero-downtime core component updates
- Self-updating update agents

### Phase 4: Full Decentralized Management
- Network admin interface for update orchestration
- Automated security update detection and approval
- Advanced rollback and recovery strategies

This decentralized update system ensures that the Digital Mycelium Network can update itself reliably while maintaining high availability and providing robust rollback capabilities.

## Coordinated Shutdown-Update-Restart Approach

### Philosophy
Rather than running parallel systems (resource-intensive), coordinate network-wide updates using graceful shutdown, component replacement, and restart with a lightweight updater helper.

### Update Architecture

```rust
// Lightweight updater helper (runs outside containers)
struct UpdaterHelper {
    node_id: NodeId,
    update_workspace: PathBuf,
    backup_location: PathBuf,
    network_coordinator: NetworkCoordinator,
}

// Network-wide update coordination
struct NetworkUpdateCoordinator {
    active_updates: HashMap<NodeId, UpdateStatus>,
    quorum_requirements: QuorumSpec,
    update_schedule: UpdateSchedule,
    rollback_manager: RollbackManager,
}

// Containerized components that can be updated
enum UpdatableComponent {
    K3sBinary,
    MyceliumCore,
    BootstrapAgent,
    NetworkServices,
    Configuration,
}
```

### Coordinated Update Process

#### Phase 1: Network Coordination
```rust
impl NetworkUpdateCoordinator {
    async fn coordinate_node_update(&self, node_id: NodeId, update: UpdatePackage) -> Result<UpdateSlot> {
        // 1. Ensure network quorum will be maintained
        self.validate_quorum_maintenance(&node_id)?;
        
        // 2. Schedule update slot to avoid too many nodes updating simultaneously
        let update_slot = self.schedule_update_slot(&node_id).await?;
        
        // 3. Notify other nodes of pending update
        self.broadcast_update_notification(&node_id, &update_slot).await?;
        
        // 4. Prepare network to handle node absence
        self.prepare_for_node_absence(&node_id).await?;
        
        Ok(update_slot)
    }
}
```

#### Phase 2: Graceful Shutdown and State Preservation
```rust
impl UpdaterHelper {
    async fn prepare_for_update(&self, update: UpdatePackage) -> Result<UpdateContext> {
        // 1. Request update slot from network
        let update_slot = self.network_coordinator.request_update_slot(self.node_id).await?;
        
        // 2. Save critical state before shutdown
        let state_snapshot = self.create_state_snapshot().await?;
        
        // 3. Notify network of impending shutdown
        self.network_coordinator.notify_shutdown_starting(self.node_id).await?;
        
        // 4. Gracefully shutdown all containerized components
        self.shutdown_containerized_components().await?;
        
        // 5. Verify clean shutdown
        self.verify_clean_shutdown()?;
        
        Ok(UpdateContext {
            update_slot,
            state_snapshot,
            update_package: update,
        })
    }
    
    async fn shutdown_containerized_components(&self) -> Result<()> {
        // Graceful shutdown sequence
        
        // 1. Stop accepting new connections
        self.stop_accepting_connections().await?;
        
        // 2. Drain existing connections
        self.drain_connections().await?;
        
        // 3. Save service state
        self.save_service_state().await?;
        
        // 4. Stop K3s and all containers
        self.stop_k3s_cluster().await?;
        
        // 5. Stop bootstrap agent
        self.stop_bootstrap_agent().await?;
        
        Ok(())
    }
}
```

#### Phase 3: Component Update While Offline
```rust
impl UpdaterHelper {
    async fn execute_offline_update(&self, context: UpdateContext) -> Result<()> {
        // Node is completely offline during this phase
        
        // 1. Backup current components
        self.backup_current_components().await?;
        
        // 2. Download and verify new components
        let new_components = self.download_update_components(&context.update_package).await?;
        self.verify_component_integrity(&new_components)?;
        
        // 3. Replace components atomically
        self.replace_components(&new_components).await?;
        
        // 4. Update configuration files
        self.update_configurations(&context.update_package).await?;
        
        // 5. Verify update integrity
        self.verify_update_completion()?;
        
        Ok(())
    }
    
    async fn replace_components(&self, new_components: &UpdateComponents) -> Result<()> {
        // Atomic component replacement
        
        for component in &new_components.components {
            match component.component_type {
                UpdatableComponent::K3sBinary => {
                    self.replace_k3s_binary(&component.new_binary).await?;
                },
                UpdatableComponent::MyceliumCore => {
                    self.replace_mycelium_core(&component.new_binary).await?;
                },
                UpdatableComponent::BootstrapAgent => {
                    self.replace_bootstrap_agent(&component.new_binary).await?;
                },
                UpdatableComponent::NetworkServices => {
                    self.update_container_images(&component.new_images).await?;
                },
                UpdatableComponent::Configuration => {
                    self.update_config_files(&component.new_configs).await?;
                },
            }
        }
        
        Ok(())
    }
}
```

#### Phase 4: Restart and Network Rejoin
```rust
impl UpdaterHelper {
    async fn restart_and_rejoin(&self, context: UpdateContext) -> Result<()> {
        // 1. Start bootstrap agent with new version
        self.start_bootstrap_agent().await?;
        
        // 2. Initialize K3s with new components
        self.start_k3s_cluster().await?;
        
        // 3. Restore service state
        self.restore_service_state(&context.state_snapshot).await?;
        
        // 4. Rejoin network and validate health
        self.rejoin_network().await?;
        
        // 5. Notify network of successful update
        self.network_coordinator.notify_update_complete(self.node_id).await?;
        
        Ok(())
    }
}
```

This coordinated approach ensures reliable updates while maintaining network availability and providing robust rollback capabilities.

## Rollback Strategy

```rust
impl UpdaterHelper {
    async fn execute_rollback(&self, reason: RollbackReason) -> Result<()> {
        // Emergency rollback to previous version
        
        // 1. Stop current (problematic) components
        self.emergency_shutdown().await?;
        
        // 2. Restore backed-up components
        self.restore_backup_components().await?;
        
        // 3. Restart with previous version
        self.restart_with_backup().await?;
        
        // 4. Notify network of rollback
        self.network_coordinator.notify_rollback(self.node_id, reason).await?;
        
        Ok(())
    }
}
```

## Benefits of This Approach

1. **Resource Efficient**: No parallel systems running, minimal memory overhead
2. **Simple and Reliable**: Clear shutdown → update → restart sequence
3. **Network Coordinated**: Ensures quorum maintenance and orderly updates
4. **Fast Rollback**: Previous components kept for immediate rollback
5. **Minimal Core Updates**: Most updates happen to containerized components
6. **Lightweight Helper**: Updater helper is small, rarely needs updates itself

## Minimizing Core Updates

**Strategy**: Keep as much as possible within containerized/virtualized components:

```rust
// Components that rarely need updates (minimal core)
struct MinimalCore {
    updater_helper: UpdaterHelper,        // Lightweight, rarely updated
    basic_networking: BasicNetworking,    // Simple, stable
    k3s_binary: K3sBinary,               // Track upstream, infrequent updates
}

// Components that can be updated frequently (containerized)
struct ContainerizedComponents {
    mycelium_services: Vec<MyceliumService>,  // Updated via container images
    network_manager: NetworkManager,          // Updated via container images
    user_services: Vec<UserService>,          // Updated via container images
    configurations: ConfigurationSet,         // Updated via config files
}
```

This approach minimizes the frequency of core component updates while enabling rapid iteration on the containerized network services and user applications.