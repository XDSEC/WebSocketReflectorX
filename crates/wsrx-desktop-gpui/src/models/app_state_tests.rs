// Tests for AppState
use super::*;
use crate::models::{Tunnel, LogEntry, LogLevel};

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{SocketAddr, IpAddr, Ipv4Addr};

    fn create_test_tunnel(id: &str, name: &str) -> Tunnel {
        Tunnel {
            id: id.to_string(),
            name: name.to_string(),
            local_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            remote_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 80),
            enabled: true,
        }
    }

    #[test]
    fn test_app_state_new() {
        let state = AppState::new();
        assert_eq!(state.current_page, Page::GetStarted);
        assert_eq!(state.tunnels.len(), 0);
        assert_eq!(state.connections.len(), 0);
        assert_eq!(state.recent_logs.len(), 0);
        assert_eq!(state.daemon_status, DaemonStatus::Stopped);
    }

    #[test]
    fn test_upsert_tunnel_add() {
        let mut state = AppState::new();
        let tunnel = create_test_tunnel("t1", "Test Tunnel");
        
        state.upsert_tunnel(tunnel.clone());
        assert_eq!(state.tunnels.len(), 1);
        assert_eq!(state.tunnels[0].id, "t1");
    }

    #[test]
    fn test_upsert_tunnel_update() {
        let mut state = AppState::new();
        let tunnel1 = create_test_tunnel("t1", "Test Tunnel");
        let mut tunnel2 = tunnel1.clone();
        tunnel2.name = "Updated Tunnel".to_string();
        
        state.upsert_tunnel(tunnel1);
        state.upsert_tunnel(tunnel2);
        
        assert_eq!(state.tunnels.len(), 1);
        assert_eq!(state.tunnels[0].name, "Updated Tunnel");
    }

    #[test]
    fn test_remove_tunnel() {
        let mut state = AppState::new();
        let tunnel = create_test_tunnel("t1", "Test Tunnel");
        
        state.upsert_tunnel(tunnel);
        assert_eq!(state.tunnels.len(), 1);
        
        state.remove_tunnel("t1");
        assert_eq!(state.tunnels.len(), 0);
    }

    #[test]
    fn test_get_tunnel() {
        let mut state = AppState::new();
        let tunnel = create_test_tunnel("t1", "Test Tunnel");
        
        state.upsert_tunnel(tunnel.clone());
        
        let found = state.get_tunnel("t1");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Test Tunnel");
        
        let not_found = state.get_tunnel("t2");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_add_log() {
        let mut state = AppState::new();
        let log = LogEntry {
            timestamp: "2025-01-01T00:00:00Z".to_string(),
            level: LogLevel::Info,
            target: "test".to_string(),
            message: "Test message".to_string(),
        };
        
        state.add_log(log);
        assert_eq!(state.recent_logs.len(), 1);
    }

    #[test]
    fn test_add_log_capacity() {
        let mut state = AppState::new();
        state.max_logs = 3;
        
        for i in 0..5 {
            let log = LogEntry {
                timestamp: format!("2025-01-01T00:00:{:02}Z", i),
                level: LogLevel::Info,
                target: "test".to_string(),
                message: format!("Message {}", i),
            };
            state.add_log(log);
        }
        
        assert_eq!(state.recent_logs.len(), 3);
        // Should keep the last 3 logs (messages 2, 3, 4)
        assert_eq!(state.recent_logs[0].message, "Message 2");
        assert_eq!(state.recent_logs[2].message, "Message 4");
    }

    #[test]
    fn test_clear_logs() {
        let mut state = AppState::new();
        let log = LogEntry {
            timestamp: "2025-01-01T00:00:00Z".to_string(),
            level: LogLevel::Info,
            target: "test".to_string(),
            message: "Test message".to_string(),
        };
        
        state.add_log(log);
        assert_eq!(state.recent_logs.len(), 1);
        
        state.clear_logs();
        assert_eq!(state.recent_logs.len(), 0);
    }
}
