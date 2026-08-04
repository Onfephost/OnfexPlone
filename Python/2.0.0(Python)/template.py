from dataclasses import dataclass as dc
import sys
sys.path.append("./RealOnfexCompiler/Libs")
from cache.Exceptions import raiseE
class main:
    def __init__(self,node):
        self.node = node
        self.funcs = {}
        self.vars = {
            "verzen":"0.0.0",
        }
        self.metodes = {}
        self.classes = {}
            
if __name__ == "__main__":
    pass