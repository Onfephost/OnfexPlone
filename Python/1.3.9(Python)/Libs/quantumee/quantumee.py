from dataclasses import dataclass as dc
import sys
sys.path.append("./RealOnfexCompiler/Libs")
from cache.Exceptions import raiseE
import math

class Ratio:
    def __init__(self,a=0.5):
        self.ornev = [a,1.5-a]
    
        
class qtrin:
    def __init__(self,ratio=Ratio(0.5)):
        self.ornev = ratio

class main:
    def __init__(self,node):
        self.node = node
        self.version = "0.0.5"
        self.funcs = {
            "natürnos":self.fn_nature,
            "keonNatürnos":self.fn_denature,
        }
        self.vars = {
            "verzen":self.version,
            "description":"""""",
            "radoeRoderAfon":100000000000000,
        }
        self.metodes = {}
        self.classes = {"qtriner":qtrin}
        
    
    def __renew__(self):
        self.vars["verzen"] = self.version
        if not isinstance(self.vars["radoeRoderAfon"],int):
            self.vars["radoeRoderAfon"] = 100000000
        else:
            if self.vars["radoeRoderAfon"] >= 100000000:
                self.vars["radoeRoderAfon"] = 100000000
                
    def fn_nature(self,ls:list):
        return nature(ls)
    
    def fn_denature(self,txt:str):
        return denature(txt)
        
if __name__ == "__main__":
    n = main(None)