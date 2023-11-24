from validx import validx  # type: ignore[attr-defined]
from validx.validx import *  # type: ignore[attr-defined] # noqa: F401, F403

__doc__ = validx.__doc__
if hasattr(validx, "__all__"):
    __all__ = validx.__all__
