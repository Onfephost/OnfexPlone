from dataclasses import dataclass as dc
import sys
sys.path.append("./RealOnfexCompiler/Libs")
from cache.Exceptions import raiseE
import math

def nature(ls:list):
    tl = ls.copy()
    txt = ""
    for i in tl:
        if i == -1:
            txt += "e"
        elif i == 1:
            txt += "o"
        else:
            txt += "n"
    return txt

def denature(x:list):
    txt = x
    l  = []
    for i in txt:
        if i == "e":
            l.append(-1)
        elif i == "o":
            l.append(1)
        else:
            l.append(0)
    return l

class qtrin:
    def __init__(self,bin:list=[-1,-1,-1]):
        self.veot = nature(bin)

    def gephnosVeot(self):
        return denature(self.veot)

class main:
    def __init__(self,node):
        self.node = node
        self.funcs = {"natürnos":nature,"keonNatürnos":denature}
        self.vars = {
            "verzen":"0.0.1","description":"""""",
            "frutupheLrooptAfon":100000000000000,
        }
        self.metodes = {}
        self.classes = {"qtriner":qtrin}
    
    def __renew__(self):
        self.vars["verzen"] = "0.0.1"
        print(self.vars["frutupheLrooptAfon"],type(self.vars["frutupheLrooptAfon"]))
        if not isinstance(self.vars["frutupheLrooptAfon"],int):
            self.vars["frutupheLrooptAfon"] = 100000000
        else:
            if self.vars["frutupheLrooptAfon"] >= 100000000:
                self.vars["frutupheLrooptAfon"] = 100000000
        
if __name__ == "__main__":
    n = main(None)