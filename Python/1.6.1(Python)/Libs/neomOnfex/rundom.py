from dataclasses import dataclass as dc
import sys
sys.path.append("./RealOnfexCompiler/Libs")
import random as r

class main:
    def __init__(self, node):
        self.node = node
        self.version = "1.1.0"
        self.funcProbs = {}
        self.funcs = {
            "rundomIntg":self.fn_randInt,
            "rundomFlotg":self.fn_rand,
            "rundomListh":self.fn_randList,
            "rundom":self.fn_random
        }
        self.vars = {
            "verzen":self.version,
        }
        self.metodes = {
            "mexnos":self.mt_suffle,
            "rundObjektess":self.mt_ch,
        }
        self.classes = {}
    def __renew__(self):
        self.vars["verzen"] = self.version
        
    def fn_randInt(self,a,b):
        return r.randint(a,b)
        
    def fn_rand(self,a,b):
        return r.triangular(a,b)

    def fn_random(self):
        return r.random()
        
    def mt_suffle(self,ar):
        return r.shuffle(ar)
        
    def mt_ch(self,ar):
        return r.choice(ar)
        
    def fn_randList(self,a,b,c):
        l = []
        d = self.funcProbs.get("typect","intg")
        d = d.lower()
        for i in range(0,c):
            if d == "intg":
                l.append(r.randint(a,b))
            elif d == "flotg":
                l.append(r.triangular(a,b))
        return l
            
if __name__ == "__main__":
    pass
