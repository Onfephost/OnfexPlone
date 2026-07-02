#!/usr/bin/env python3
import subprocess
import sys
from pathlib import Path

workspace_root = Path(__file__).resolve().parent.parent
interpreter_script = workspace_root / "Python" / "1.3.6(Python)" / "onfex"

if len(sys.argv) < 2:
    print("Usage: run_onfex.py <file.onfex>")
    sys.exit(1)

file_path = Path(sys.argv[1]).expanduser().resolve()
if not file_path.exists():
    print(f"File not found: {file_path}")
    sys.exit(1)

cmd = [sys.executable, str(interpreter_script), file_path.name]
result = subprocess.run(cmd, cwd=str(file_path.parent), check=False)
sys.exit(result.returncode)
