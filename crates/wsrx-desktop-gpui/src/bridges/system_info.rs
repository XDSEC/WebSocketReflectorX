// System info bridge - System resource monitoring
use sysinfo::System;
use std::sync::{Arc, Mutex};

/// System information and monitoring
pub struct SystemInfoBridge {
    /// System info handle
    system: Arc<Mutex<System>>,
}

impl SystemInfoBridge {
    /// Create a new system info bridge
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        
        Self {
            system: Arc::new(Mutex::new(system)),
        }
    }
    
    /// Get CPU usage percentage
    pub fn cpu_usage(&self) -> f32 {
        let mut system = self.system.lock().unwrap();
        system.refresh_cpu_all();
        
        let cpus = system.cpus();
        if cpus.is_empty() {
            return 0.0;
        }
        
        cpus.iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / cpus.len() as f32
    }
    
    /// Get memory usage percentage
    pub fn memory_usage(&self) -> f32 {
        let mut system = self.system.lock().unwrap();
        system.refresh_memory();
        
        let total = system.total_memory();
        let used = system.used_memory();
        
        if total == 0 {
            return 0.0;
        }
        
        (used as f32 / total as f32) * 100.0
    }
    
    /// Get total memory in bytes
    pub fn total_memory(&self) -> u64 {
        let mut system = self.system.lock().unwrap();
        system.refresh_memory();
        system.total_memory()
    }
    
    /// Get used memory in bytes
    pub fn used_memory(&self) -> u64 {
        let mut system = self.system.lock().unwrap();
        system.refresh_memory();
        system.used_memory()
    }
    
    /// Refresh all system information
    pub fn refresh(&self) {
        let mut system = self.system.lock().unwrap();
        system.refresh_all();
    }
}

impl Default for SystemInfoBridge {
    fn default() -> Self {
        Self::new()
    }
}

