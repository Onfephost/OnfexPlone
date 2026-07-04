from dataclasses import dataclass as d
import sys
sys.path.append("./RealOnfexCompiler")
from cache.Exceptions import raiseE

class main:
    def __init__(self,node):
        self.docName = "a"
        self.funcProbs = {}
        self.path = f"./Python/1.3.9(Python)/Libs/estorge/{self.docName}.json"
        self.version = "0.5.1"
        self.node = node
        self.funcs = {
            "cesnos":self.fn_cre,
            "gephnos":self.fn_get,
            "delnos":self.fn_del,
            "namSernos":self.fn_setpath,
        }
        self.vars = {
            "verzen":self.version,
        }
        self.metodes = {}
        self.classes = {}

    def __renew__(self):
        self.vars["verzen"] = self.version
    
    def fn_cre(self,a,b):
        with open(self.path,"r") as f:
            self.dct = eval(f.read())
        self.dct[a] = b
        with open(self.path,"w") as f:
            f.write(str(self.dct))
    
    def fn_get(self,a):
        with open(self.path,"r") as f:
            self.dct = eval(f.read())
        if a in self.dct:
            return self.dct[a]
        else:
            raiseE(self.node,"Estorge Ern",f"{a} asp estorge dikth neat froundnosan")
    
    def fn_del(self,a):
        with open(self.path,"r") as f:
            dct = eval(f.read())
        if a in dct:
            del dct[a]
        with open(self.path,"w") as f:
            f.write(str(dct))

    def fn_setpath(self,a):
        self.path = f"./Python/1.3.9(Python)/Libs/estorge/{a}.json"
        with open(self.path,"w") as f:
            f.write("{}")
            
if __name__ == "__main__":
    pass
