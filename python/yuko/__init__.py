from yuko import yuko  # type: ignore[attr-defined]
from yuko.yuko import *  # type: ignore[attr-defined] # noqa: F401, F403
import importlib.metadata as importlib_metadata

try:
    # This will read version from pyproject.toml
    __version__ = importlib_metadata.version(__package__ or __name__)
except importlib_metadata.PackageNotFoundError:
    __version__ = "development"


__doc__ = yuko.__doc__
if hasattr(yuko, "__all__"):
    __all__ = yuko.__all__
