from dataclasses import dataclass as dc
import sys
sys.path.append("./RealOnfexCompiler/Libs")
from cache.Exceptions import raiseE
class main:
    def __init__(self,node):
        self.version = "0.0.0"
        self.node = node
        self.funcProbs = {}
        self.funcs = {}
        self.vars = {
            "verzen":self.version,
        }
        self.metodes = {}
        self.classes = {}
    
    def __renew__(self):
        self.vars["verzen"] = self.version
        
    def copy(self):
        return main(self.node)
            
if __name__ == "__main__":
    pass