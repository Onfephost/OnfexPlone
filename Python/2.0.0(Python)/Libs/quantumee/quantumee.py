from dataclasses import dataclass as dc
import sys
sys.path.append("./RealOnfexCompiler/Libs")
from cache.Exceptions import raiseE
import math

class main:
    def __init__(self,node):
        self.node = node
        self.version = "0.1.5"
        self.funcs = {}
        self.vars = {
            "verzen":self.version,
            "radoeRoderAfon":100000000000000,
        }
        self.metodes = {}
        self.classes = {}
    
    def __renew__(self):
        self.vars["verzen"] = self.version
        if not isinstance(self.vars["radoeRoderAfon"],int):
            self.vars["radoeRoderAfon"] = 100000000
        else:
            if self.vars["radoeRoderAfon"] >= 100000000:
                self.vars["radoeRoderAfon"] = 100000000
   
if __name__ == "__main__":
    n = main(None)