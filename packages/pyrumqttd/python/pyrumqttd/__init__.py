__version__: str

class Config: ...


class Broker:

    def __init__(self, config: Config) -> None: ...

    def start(self) -> None: ...
