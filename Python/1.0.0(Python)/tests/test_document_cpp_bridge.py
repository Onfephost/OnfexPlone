import os
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from cache.document import OnfexPloneDoph


def test_run_uses_cpp_bridge():
    with tempfile.TemporaryDirectory() as tmpdir:
        sample = Path(tmpdir) / "sample.onfex"
        sample.write_text("print('hello')", encoding="utf-8")

        obj = OnfexPloneDoph(tmpdir, sample.name, None, False)
        output = []

        import contextlib
        import io

        with contextlib.redirect_stdout(io.StringIO()) as captured:
            obj.run()

        result = captured.getvalue()
        assert "[Onfex C++ IDE Bridge]" in result
