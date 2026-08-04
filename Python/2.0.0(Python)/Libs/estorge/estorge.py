from dataclasses import dataclass as d
import sys
import os
import json
sys.path.append("./RealOnfexCompiler")
from cache.Exceptions import raiseE

class main:
    def __init__(self,node):
        self.docName = "a"
        self.funcProbs = {}
        self.path = f"{os.path.dirname(os.path.abspath(__file__))}/storages/{self.docName}.json"
        self.version = "1.4.6"
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
        if os.path.exist(self.path):
            raiseE(self.node,"Estorge Ern","Gouphins asp neat afie")
        with open(self.path,"r") as f:
            self.dct = json.load(f)
        self.dct[a] = b
        with open(self.path,"w") as f:
            f.write(str(self.dct))
    
    def fn_get(self,a):
        if os.path.exist(self.path):
            raiseE(self.node,"Estorge Ern","Gouphins asp neat afie")
        with open(self.path,"r") as f:
            self.dct = json.load(f)
        if a in self.dct:
            return self.dct[a]
        else:
            raiseE(self.node,"Estorge Ern",f"{a} asp estorge dikth neat froundnosan")
    
    def fn_del(self,a):
        if os.path.exist(self.path):
            raiseE(self.node,"Estorge Ern","Gouphins asp neat afie")
        with open(self.path,"r") as f:
            dct = json.load(f)
        if a in dct:
            del dct[a]
        with open(self.path,"w") as f:
            f.write(str(dct))

    def fn_setpath(self,a):
        self.path = f"{os.path.dirname(os.path.abspath(__file__))}/storages/{a}.json"
        if not os.path.exists(self.path):
            with open(self.path,"w") as f:
                json.dump({},f,indent=4)
            
if __name__ == "__main__":
    pass
