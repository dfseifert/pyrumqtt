use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use rumqttd;
use std::collections::HashMap;
use std::sync::OnceLock;
use std::{net, thread};

#[pyclass]
#[derive(Debug, Default, Clone)]
pub struct Config {
    inner: rumqttd::Config,
}

#[pymethods]
impl Config {
    #[new]
    pub fn new(
        id: usize,
        router: RouterConfig,
        v5: Option<HashMap<String, ServerSettings>>,
    ) -> Self {
        Self {
            inner: rumqttd::Config {
                id,
                router: router.into_inner(),
                v5: v5.map(|s| {
                    s.into_iter()
                        .map(|(k, v)| (k.to_owned(), v.into_inner()))
                        .collect()
                }),
                v4: None,
                ws: None,
                cluster: None,
                console: None,
                bridge: None,
                prometheus: None,
                metrics: None,
            },
        }
    }

    #[getter]
    pub fn id(&self) -> usize {
        self.inner.id
    }

    #[setter]
    pub fn set_id(&mut self, id: usize) {
        self.inner.id = id;
    }

    #[getter]
    pub fn router(&self) -> RouterConfig {
        RouterConfig::from_inner(self.inner.router.clone()) // Wrap the `router` from the external struct.
    }

    #[setter]
    pub fn set_router(&mut self, router: RouterConfig) {
        self.inner.router = router.into_inner();
    }
}

// Wrapper for `RouterConfig`
#[pyclass]
#[derive(Debug, Clone)]
pub struct RouterConfig {
    inner: rumqttd::RouterConfig, // Store the external `RouterConfig` struct.
}

#[pymethods]
impl RouterConfig {
    #[new]
    pub fn new(
        max_connections: usize,
        max_outgoing_packet_count: u64,
        max_segment_size: usize,
        max_segment_count: usize,
    ) -> Self {
        Self {
            inner: rumqttd::RouterConfig {
                max_connections,
                max_outgoing_packet_count,
                max_segment_size,
                max_segment_count,
                custom_segment: None,
                initialized_filters: None,
                shared_subscriptions_strategy: Default::default(),
            },
        }
    }

    #[getter]
    pub fn max_connections(&self) -> usize {
        self.inner.max_connections
    }

    #[setter]
    pub fn set_max_connections(&mut self, max_connections: usize) {
        self.inner.max_connections = max_connections;
    }

    #[getter]
    pub fn max_outgoing_packet_count(&self) -> u64 {
        self.inner.max_outgoing_packet_count
    }

    #[setter]
    pub fn set_max_outgoing_packet_count(&mut self, max_outgoing_packet_count: u64) {
        self.inner.max_outgoing_packet_count = max_outgoing_packet_count;
    }

    #[getter]
    pub fn max_segment_size(&self) -> usize {
        self.inner.max_segment_size
    }

    #[setter]
    pub fn set_max_segment_size(&mut self, max_segment_size: usize) {
        self.inner.max_segment_size = max_segment_size;
    }

    #[getter]
    pub fn max_segment_count(&self) -> usize {
        self.inner.max_segment_count
    }

    #[setter]
    pub fn set_max_segment_count(&mut self, max_segment_count: usize) {
        self.inner.max_segment_count = max_segment_count;
    }
}

impl RouterConfig {
    pub fn into_inner(self) -> rumqttd::RouterConfig {
        self.inner
    }

    pub fn from_inner(inner: rumqttd::RouterConfig) -> Self {
        Self { inner }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct ServerSettings {
    inner: rumqttd::ServerSettings,
}

#[pymethods]
impl ServerSettings {
    #[new]
    pub fn new(
        name: String,
        listen: SocketAddr,
        next_connection_delay_ms: u64,
        connections: ConnectionSettings,
    ) -> Self {
        Self {
            inner: rumqttd::ServerSettings {
                name,
                listen: listen.into_inner(),
                tls: None,
                next_connection_delay_ms,
                connections: connections.into_inner(),
            },
        }
    }

    #[getter]
    pub fn name(&self) -> String {
        self.inner.name.clone()
    }

    #[setter]
    pub fn set_name(&mut self, name: String) {
        self.inner.name = name;
    }

    #[getter]
    pub fn listen(&self) -> SocketAddr {
        SocketAddr::from_inner(self.inner.listen)
    }

    #[setter]
    pub fn set_listen(&mut self, listen: SocketAddr) {
        self.inner.listen = listen.into_inner();
    }

    #[getter]
    pub fn next_connection_delay_ms(&self) -> u64 {
        self.inner.next_connection_delay_ms
    }

    #[setter]
    pub fn set_next_connection_delay_ms(&mut self, delay: u64) {
        self.inner.next_connection_delay_ms = delay;
    }

    #[getter]
    pub fn connections(&self) -> ConnectionSettings {
        ConnectionSettings::from_inner(self.inner.connections.clone()) // Wrap ConnectionSettings as PyConnectionSettings.
    }

    #[setter]
    pub fn set_connections(&mut self, connections: ConnectionSettings) {
        self.inner.connections = connections.into_inner();
    }
}

impl ServerSettings {
    pub fn into_inner(self) -> rumqttd::ServerSettings {
        self.inner
    }

    pub fn from_inner(inner: rumqttd::ServerSettings) -> Self {
        Self { inner }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct ConnectionSettings {
    inner: rumqttd::ConnectionSettings,
}

#[pymethods]
impl ConnectionSettings {
    #[new]
    pub fn new(
        connection_timeout_ms: u16,
        max_payload_size: usize,
        max_inflight_count: usize,
        dynamic_filters: bool,
        auth: Option<HashMap<String, String>>, // Map stays compatible.
    ) -> Self {
        Self {
            inner: rumqttd::ConnectionSettings {
                connection_timeout_ms,
                max_payload_size,
                max_inflight_count,
                auth,
                external_auth: None, // Not exposed to Python for simplicity.
                dynamic_filters,
            },
        }
    }

    #[getter]
    pub fn connection_timeout_ms(&self) -> u16 {
        self.inner.connection_timeout_ms
    }

    #[setter]
    pub fn set_connection_timeout_ms(&mut self, timeout: u16) {
        self.inner.connection_timeout_ms = timeout;
    }

    #[getter]
    pub fn max_payload_size(&self) -> usize {
        self.inner.max_payload_size
    }

    #[setter]
    pub fn set_max_payload_size(&mut self, size: usize) {
        self.inner.max_payload_size = size;
    }

    #[getter]
    pub fn max_inflight_count(&self) -> usize {
        self.inner.max_inflight_count
    }

    #[setter]
    pub fn set_max_inflight_count(&mut self, count: usize) {
        self.inner.max_inflight_count = count;
    }

    #[getter]
    pub fn auth(&self) -> Option<HashMap<String, String>> {
        self.inner.auth.clone() // Clone to ensure compatibility in Python.
    }

    #[setter]
    pub fn set_auth(&mut self, auth: Option<HashMap<String, String>>) {
        self.inner.auth = auth;
    }

    #[getter]
    pub fn dynamic_filters(&self) -> bool {
        self.inner.dynamic_filters
    }

    #[setter]
    pub fn set_dynamic_filters(&mut self, filters: bool) {
        self.inner.dynamic_filters = filters;
    }
}

impl ConnectionSettings {
    pub fn into_inner(self) -> rumqttd::ConnectionSettings {
        self.inner
    }

    pub fn from_inner(inner: rumqttd::ConnectionSettings) -> Self {
        Self { inner }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct SocketAddr {
    inner: net::SocketAddr, // Store the Rust `SocketAddr` struct.
}

#[pymethods]
impl SocketAddr {
    #[new]
    pub fn new(ip: &str, port: u16) -> PyResult<Self> {
        let ip_addr: net::IpAddr = ip
            .parse()
            .map_err(|_err| PyValueError::new_err("Invalid IP address"))?;
        let socket_addr = net::SocketAddr::new(ip_addr, port);
        Ok(Self { inner: socket_addr })
    }

    #[getter]
    pub fn ip(&self) -> String {
        self.inner.ip().to_string()
    }

    #[setter]
    pub fn set_ip(&mut self, ip: &str) -> PyResult<()> {
        let ip_addr: net::IpAddr = ip
            .parse()
            .map_err(|_err| PyValueError::new_err("Invalid IP address"))?;
        self.inner.set_ip(ip_addr);
        Ok(())
    }

    #[getter]
    pub fn port(&self) -> u16 {
        self.inner.port()
    }

    #[setter]
    pub fn set_port(&mut self, port: u16) {
        self.inner.set_port(port);
    }

    pub fn to_string(&self) -> String {
        self.inner.to_string()
    }

    #[staticmethod]
    pub fn from_string(addr: &str) -> PyResult<Self> {
        let socket_addr: net::SocketAddr = addr
            .parse()
            .map_err(|_err| PyValueError::new_err("Invalid socket address format"))?;
        Ok(Self { inner: socket_addr })
    }
}

impl SocketAddr {
    pub fn into_inner(self) -> net::SocketAddr {
        self.inner
    }

    pub fn from_inner(inner: net::SocketAddr) -> Self {
        Self { inner }
    }
}

/// Formats the sum of two numbers as string.
/// This struct represents the embedded MQTT Broker.
#[pyclass]
struct Broker {
    config: Config,
}

#[pymethods]
impl Broker {
    #[new]
    pub fn new(config: Config) -> Self {
        Broker { config }
    }

    pub fn start(&self) {
        let config = self.config.clone();
        thread::spawn(move || {
            let mut broker = rumqttd::Broker::new(config.inner);
            broker.start().expect("Broker failed to start");
        });
    }
}

pub fn get_pyrumqttd_version() -> &'static str {
    static PYRUMQTTD_VERSION: OnceLock<String> = OnceLock::new();

    PYRUMQTTD_VERSION.get_or_init(|| {
        let version = env!("CARGO_PKG_VERSION");
        // cargo uses "1.0-alpha1" etc. while python uses "1.0.0a1",
        // While this is not full compatibility, it's good enough for now.
        version.replace("-alpha", "a").replace("-beta", "b")
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn _pyrumqttd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", get_pyrumqttd_version())?;
    m.add_class::<Config>()?;
    m.add_class::<RouterConfig>()?;
    m.add_class::<ServerSettings>()?;
    m.add_class::<ConnectionSettings>()?;
    m.add_class::<Broker>()?;
    m.add_class::<SocketAddr>()?;
    Ok(())
}
