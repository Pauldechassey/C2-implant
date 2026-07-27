import time

_last_seen: float | None = None


def touch() -> None:
    global _last_seen
    _last_seen = time.time()


def get_last_seen() -> float | None:
    return _last_seen
