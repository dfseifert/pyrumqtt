__version__: str

class Config:
    id: int
    router: RouterConfig
    v5: dict[str, ServerSettings] | None

    def __init__(
        self,
        id: int,
        router: "RouterConfig",
        v5: dict[str, "ServerSettings"] | None = None,
    ) -> None: ...

class RouterConfig:
    max_connections: int
    max_outgoing_packet_count: int
    max_segment_size: int
    max_segment_count: int

    def __init__(
        self,
        max_connections: int,
        max_outgoing_packet_count: int,
        max_segment_size: int,
        max_segment_count: int,
    ) -> None: ...

class ServerSettings:
    name: str
    listen: SocketAddr
    next_connection_delay_ms: int
    connections: ConnectionSettings

    def __init__(
        self,
        name: str,
        listen: SocketAddr,
        next_connection_delay_ms: int,
        connections: ConnectionSettings,
    ) -> None: ...

class SocketAddr:
    ip: str
    port: int

    def __init__(self, ip: str, port: int) -> None: ...

class ConnectionSettings:
    connection_timeout_ms: int
    max_payload_size: int
    max_inflight_count: int
    dynamic_filters: bool
    auth: dict[str, str] | None

    def __init__(
        self,
        connection_timeout_ms: int,
        max_payload_size: int,
        max_inflight_count: int,
        dynamic_filters: bool,
        auth: dict[str, str] | None = None,
    ) -> None: ...

class Broker:
    def __init__(self, config: Config) -> None: ...
    def start(self) -> None: ...
