use std::sync::Arc;

use tokio::net::TcpListener;
use wsrx::tunnel::Tunnel;

use super::model::InstanceData;
use crate::ui::Instance;

pub struct ProxyInstance {
    data: InstanceData,
    _tunnel: Tunnel,
}

impl ProxyInstance {
    pub fn new(
        label: Arc<String>, scope_host: Arc<String>, listener: TcpListener, remote: Arc<String>,
    ) -> Self {
        let tunnel = Tunnel::new(remote.clone(), listener);

        Self {
            data: InstanceData {
                label,
                remote,
                local: tunnel.local.clone(),
                latency: -1,
                scope_host,
            },
            _tunnel: tunnel,
        }
    }
}

impl From<&ProxyInstance> for InstanceData {
    fn from(value: &ProxyInstance) -> Self {
        value.data.clone()
    }
}

impl From<&ProxyInstance> for Instance {
    fn from(value: &ProxyInstance) -> Self {
        Instance {
            label: value.label.as_str().into(),
            remote: value.remote.as_str().into(),
            local: value.local.as_str().into(),
            latency: value.latency,
            scope_host: value.scope_host.as_str().into(),
        }
    }
}

impl std::ops::Deref for ProxyInstance {
    type Target = InstanceData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl std::ops::DerefMut for ProxyInstance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
