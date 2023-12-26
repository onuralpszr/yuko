from yuko import yuko  # type: ignore[attr-defined]
from yuko.yuko import *  # type: ignore[attr-defined] # noqa: F401, F403

__doc__ = yuko.__doc__
if hasattr(yuko, "__all__"):
    __all__ = yuko.__all__
