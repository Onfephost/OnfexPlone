from cache.Exceptions import *
from cache.lexer import lex
from cache.parser import Parser
import os
import subprocess
import sys
from pathlib import Path

#OPD
class OnfexPloneDoph:
    def __init__(self,docpath,name,code,isImport:bool):
        self.name = name
        self.path = docpath
        self.main = None
        if code is not None:
            self.code = code
        else:
            self.code = str(open(docpath+"/"+name,"r").read())
        self.ii = isImport
        self.envPtrC = 1
        
    def _run_cpp_bridge(self):
        bridge_src = Path(__file__).with_name("onfex_ide_bridge.cpp")
        bridge_bin = Path(__file__).with_name("onfex_ide_bridge")

        if not bridge_bin.exists():
            subprocess.run([
                "g++", str(bridge_src), "-o", str(bridge_bin)
            ], check=True, capture_output=True, text=True)

        proc = subprocess.run(
            [str(bridge_bin), self.path, self.name],
            input=self.code,
            capture_output=True,
            text=True,
            check=False,
        )

        if proc.stdout:
            print(proc.stdout, end="")
        if proc.stderr:
            print(proc.stderr, end="", file=sys.stderr)
        return proc.returncode

    def run(self,nm=None):
        self.main = nm
        try:
            self._run_cpp_bridge()
        except Exception as exc:
            print(f"[Onfex C++ bridge fallback] {exc}", file=sys.stderr)
            try:
                import cache.interpreter as intp

                if self.ii == False:
                    tokens = lex(self.code)
                    tree = Parser(tokens).parse()
                    print("Onfex asp dowpownosyer...")
                    intr = intp.Interpreter()
                    os.system("clear")
                    intr.path = self.path
                    intr.mainOn = True
                    print("[Onfex Run]")
                    intr.eval(tree)
                else:
                    tokens = lex(self.code)
                    tree = Parser(tokens).parse()
                    intr = intp.Interpreter()
                    intr.env.ptrC = self.envPtrC
                    intr.path = self.path
                    intr.mainDoc = self.main
                    intr.eval(tree)
                    return intr.env
            except OnfexError as e:
                show_error(self.code, e)


def create_and_run(docpath, name, code=None, isImport=False):
    obj = OnfexPloneDoph(docpath, name, code, isImport)
    obj.run()
    return obj


if __name__ == "__main__":
    create_and_run(sys.argv[1] if len(sys.argv) > 1 else ".", sys.argv[2] if len(sys.argv) > 2 else "main.onfex")

