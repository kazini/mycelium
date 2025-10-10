# Latency Adaptation Strategies

## Overview

The Digital Mycelium Network automatically adapts its behavior based on network latency conditions, optimizing performance for both WAN (high latency) and LAN (low latency) deployments.

## High Latency Adaptation (WAN Deployment)

### Buffer Management Strategy

**Larger Buffer Sizes**: Accommodate network delays and reduce frequency of throttling events.

- **Buffer Size Multiplier**: 3.0x standard size
- **Throttle Threshold**: 80% (higher tolerance for lag)
- **Emergency Pause**: Disabled (best-effort replication)

### Replication Timing

**Longer Intervals**: Reduce network overhead and allow for batch processing.

- **Replication Interval**: 100ms (vs 10ms for low latency)
- **Convergence Timeout**: 30 seconds for planned migration
- **Heartbeat Interval**: 5 seconds between nodes

### Throttling Behavior

**Aggressive Throttling Curves**: Prevent buffer overflow in high-latency scenarios.

```
Exponential Curve: intensity = max_intensity × (buffer_level - threshold)^3
- More aggressive throttling as buffer fills
- Prevents cascade failures in high-latency networks
- Prioritizes primary VM performance over perfect sync
```

### Data Transfer Optimization

**Parallel Distribution**: Maximize bandwidth utilization across multiple connections.

- **Chunk Size**: 64MB per transfer
- **Parallel Connections**: Up to 8 simultaneous transfers
- **Compression**: Enabled for all transfers
- **Retry Logic**: Exponential backoff with jitter

## Low Latency Adaptation (LAN Deployment)

### Buffer Management Strategy

**Smaller Buffer Sizes**: Enable responsive throttling and near-real-time replication.

- **Buffer Size Multiplier**: 0.5x standard size
- **Throttle Threshold**: 60% (lower tolerance for lag)
- **Emergency Pause**: Enabled (guaranteed RPO)

### Replication Timing

**Shorter Intervals**: Minimize replication lag and enable near-synchronous operation.

- **Replication Interval**: 5ms (high frequency)
- **Convergence Timeout**: 5 seconds for planned migration
- **Heartbeat Interval**: 1 second between nodes

### Throttling Behavior

**Responsive Throttling Curves**: Quick response to buffer changes.

```
Linear Curve: intensity = max_intensity × (buffer_level - threshold) / (1 - threshold)
- Immediate response to buffer level changes
- Prioritizes near-zero RPO over primary performance
- Enables sub-second failover times
```

### Data Transfer Optimization

**Low-Latency Transfers**: Optimize for speed over bandwidth efficiency.

- **Chunk Size**: 4MB per transfer
- **Parallel Connections**: Up to 16 simultaneous transfers
- **Compression**: Disabled (CPU vs latency trade-off)
- **Retry Logic**: Immediate retry with circuit breaker

## Automatic Latency Detection

### Network Measurement

**Continuous Monitoring**: Measure round-trip time and bandwidth to peer nodes.

```rust
pub struct LatencyMonitor {
    measurements: RingBuffer<LatencyMeasurement>,
    current_profile: LatencyProfile,
}

impl LatencyMonitor {
    pub async fn measure_latency(&self, peer: &NodeId) -> Duration {
        // Send ping and measure round-trip time
    }
    
    pub fn determine_profile(&self) -> LatencyProfile {
        let avg_latency = self.measurements.average();
        match avg_latency {
            d if d < Duration::from_millis(10) => LatencyProfile::Low,
            d if d < Duration::from_millis(100) => LatencyProfile::Medium,
            _ => LatencyProfile::High,
        }
    }
}
```

### Adaptive Configuration

**Dynamic Reconfiguration**: Automatically adjust parameters based on detected conditions.

```rust
pub struct AdaptiveConfiguration {
    base_config: MyceliumConfig,
    current_profile: LatencyProfile,
}

impl AdaptiveConfiguration {
    pub fn adapt_for_latency(&mut self, profile: LatencyProfile) {
        match profile {
            LatencyProfile::High => self.apply_high_latency_optimizations(),
            LatencyProfile::Low => self.apply_low_latency_optimizations(),
            LatencyProfile::Medium => self.apply_balanced_configuration(),
        }
    }
}
```

## Hybrid Scenarios

### Mixed Latency Networks

**Per-Connection Adaptation**: Different strategies for different peer connections.

- **Local Peers**: Low-latency configuration
- **Remote Peers**: High-latency configuration
- **Dynamic Switching**: Adapt based on real-time measurements

### Geographic Distribution

**Zone-Aware Configuration**: Optimize based on geographic placement.

- **Same Zone**: Low-latency mode
- **Cross-Zone**: Medium-latency mode
- **Cross-Region**: High-latency mode

## Performance Metrics

### Key Performance Indicators

**Latency-Specific Metrics**: Track performance across different scenarios.

- **Replication Lag**: Time between primary write and replica application
- **Failover Time**: Time to promote replica to primary
- **Throughput**: Data transfer rate under different latency conditions
- **Buffer Utilization**: Average and peak buffer usage

### Monitoring and Alerting

**Adaptive Thresholds**: Alert thresholds that adapt to latency profile.

```rust
pub struct AdaptiveAlerts {
    thresholds: HashMap<LatencyProfile, AlertThresholds>,
}

impl AdaptiveAlerts {
    pub fn check_replication_lag(&self, lag: Duration, profile: LatencyProfile) -> Option<Alert> {
        let threshold = self.thresholds[&profile].max_replication_lag;
        if lag > threshold {
            Some(Alert::ReplicationLagExceeded { lag, threshold })
        } else {
            None
        }
    }
}
```

## Configuration Examples

### High Latency Configuration

```yaml
latency_profile: high
distributed_ram:
  max_buffer_size: 3145728  # 3MB
  throttle_threshold: 0.8
  throttling_curve:
    type: exponential
    exponent: 3.0
  emergency_pause_enabled: false
  replication_interval: 100ms
networking:
  chunk_size: 67108864  # 64MB
  parallel_connections: 8
  compression_enabled: true
```

### Low Latency Configuration

```yaml
latency_profile: low
distributed_ram:
  max_buffer_size: 524288  # 512KB
  throttle_threshold: 0.6
  throttling_curve:
    type: linear
  emergency_pause_enabled: true
  replication_interval: 5ms
networking:
  chunk_size: 4194304  # 4MB
  parallel_connections: 16
  compression_enabled: false
```

This adaptive approach ensures optimal performance across diverse network conditions while maintaining the core guarantees of the distributed RAM system.