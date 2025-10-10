// Reference Implementation: Adaptive Throttling System
// This demonstrates the configurable throttling curves and emergency pause mechanisms
// for the distributed RAM replication system.

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};

/// Configuration for adaptive throttling system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveThrottlingConfig {
    /// Percentage of buffer when throttling starts (0.0 to 1.0)
    pub throttle_threshold: f32,
    
    /// Maximum CPU/IO reduction allowed (0.0 to 1.0)
    pub max_throttling_intensity: f32,
    
    /// Throttling curve configuration
    pub throttling_curve: ThrottlingCurve,
    
    /// Whether to enable emergency pause when buffer is full
    pub emergency_pause_enabled: bool,
}

/// Different throttling curve strategies for adaptive performance control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThrottlingCurve {
    /// Linear increase in throttling intensity
    Linear,
    
    /// Exponential increase with configurable exponent
    Exponential { exponent: f32 },
    
    /// Custom curve defined by control points for fine-tuned control
    Custom { control_points: Vec<(f32, f32)> },
}

/// Adaptive throttling controller with configurable curves
pub struct AdaptiveThrottlingController {
    /// Configuration parameters
    config: AdaptiveThrottlingConfig,
    
    /// Current throttling state per VM
    throttling_states: Arc<RwLock<HashMap<String, ThrottlingState>>>,
}

impl AdaptiveThrottlingController {
    pub fn new(config: AdaptiveThrottlingConfig) -> Self {
        Self {
            config,
            throttling_states: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Apply adaptive throttling based on buffer level and configured curve
    pub async fn apply_adaptive_throttling(&self, vm_id: &str, buffer_level: f32) -> Result<(), ThrottlingError> {
        let throttling_intensity = self.calculate_throttling_intensity(buffer_level);
        
        // Apply CPU and I/O throttling to slow dirty page generation
        self.throttle_vm_cpu(vm_id, throttling_intensity).await?;
        self.throttle_vm_io(vm_id, throttling_intensity).await?;
        
        // Update throttling state for monitoring
        let mut states = self.throttling_states.write().await;
        states.insert(vm_id.to_string(), ThrottlingState {
            intensity: throttling_intensity,
            applied_at: std::time::Instant::now(),
        });
        
        Ok(())
    }
    
    /// Calculate throttling intensity based on configured curve
    fn calculate_throttling_intensity(&self, buffer_level: f32) -> f32 {
        if buffer_level <= self.config.throttle_threshold {
            return 0.0;
        }
        
        let normalized_level = (buffer_level - self.config.throttle_threshold) / (1.0 - self.config.throttle_threshold);
        
        match &self.config.throttling_curve {
            ThrottlingCurve::Linear => {
                self.config.max_throttling_intensity * normalized_level
            },
            ThrottlingCurve::Exponential { exponent } => {
                self.config.max_throttling_intensity * normalized_level.powf(*exponent)
            },
            ThrottlingCurve::Custom { control_points } => {
                self.interpolate_custom_curve(normalized_level, control_points)
            },
        }
    }
    
    /// Interpolate custom throttling curve using linear interpolation between control points
    fn interpolate_custom_curve(&self, level: f32, control_points: &[(f32, f32)]) -> f32 {
        // Simple linear interpolation between control points
        for window in control_points.windows(2) {
            let (x1, y1) = window[0];
            let (x2, y2) = window[1];
            
            if level >= x1 && level <= x2 {
                let t = (level - x1) / (x2 - x1);
                return y1 + t * (y2 - y1);
            }
        }
        
        // Default to max intensity if beyond curve
        self.config.max_throttling_intensity
    }
    
    /// Emergency pause VM when buffer is full (configurable behavior)
    pub async fn emergency_pause_vm(&self, vm_id: &str) -> Result<(), ThrottlingError> {
        if self.config.emergency_pause_enabled {
            // Pause VM execution to prevent data loss
            self.pause_vm_execution(vm_id).await?;
            
            // Wait for buffer to clear
            while self.get_buffer_level(vm_id).await? > 0.8 {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
            
            // Resume VM execution
            self.resume_vm_execution(vm_id).await?;
        } else {
            // Best effort: discard oldest pages to make room
            self.discard_oldest_pages(vm_id).await?;
        }
        
        Ok(())
    }
    
    // Stub implementations for compilation
    async fn throttle_vm_cpu(&self, _vm_id: &str, _intensity: f32) -> Result<(), ThrottlingError> { Ok(()) }
    async fn throttle_vm_io(&self, _vm_id: &str, _intensity: f32) -> Result<(), ThrottlingError> { Ok(()) }
    async fn pause_vm_execution(&self, _vm_id: &str) -> Result<(), ThrottlingError> { Ok(()) }
    async fn resume_vm_execution(&self, _vm_id: &str) -> Result<(), ThrottlingError> { Ok(()) }
    async fn discard_oldest_pages(&self, _vm_id: &str) -> Result<(), ThrottlingError> { Ok(()) }
    async fn get_buffer_level(&self, _vm_id: &str) -> Result<f32, ThrottlingError> { Ok(0.5) }
}

#[derive(Debug, Clone)]
pub struct ThrottlingState {
    pub intensity: f32,
    pub applied_at: std::time::Instant,
}

#[derive(Debug, thiserror::Error)]
pub enum ThrottlingError {
    #[error("VM not found: {0}")]
    VMNotFound(String),
    
    #[error("Throttling failed: {0}")]
    ThrottlingFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_linear_throttling_curve() {
        let config = AdaptiveThrottlingConfig {
            throttle_threshold: 0.7,
            max_throttling_intensity: 0.9,
            throttling_curve: ThrottlingCurve::Linear,
            emergency_pause_enabled: true,
        };
        
        let controller = AdaptiveThrottlingController::new(config);
        
        // Test different buffer levels
        assert_eq!(controller.calculate_throttling_intensity(0.5), 0.0); // Below threshold
        assert_eq!(controller.calculate_throttling_intensity(0.85), 0.45); // Linear interpolation
        assert_eq!(controller.calculate_throttling_intensity(1.0), 0.9); // Max intensity
    }
    
    #[tokio::test]
    async fn test_exponential_throttling_curve() {
        let config = AdaptiveThrottlingConfig {
            throttle_threshold: 0.7,
            max_throttling_intensity: 0.9,
            throttling_curve: ThrottlingCurve::Exponential { exponent: 2.0 },
            emergency_pause_enabled: true,
        };
        
        let controller = AdaptiveThrottlingController::new(config);
        
        // Test exponential curve behavior
        assert_eq!(controller.calculate_throttling_intensity(0.5), 0.0); // Below threshold
        assert!((controller.calculate_throttling_intensity(0.85) - 0.225).abs() < 0.001); // Exponential curve
        assert_eq!(controller.calculate_throttling_intensity(1.0), 0.9); // Max intensity
    }
}