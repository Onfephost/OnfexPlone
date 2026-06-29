from dataclasses import dataclass as dc
import sys
sys.path.append("./RealOnfexCompiler/Libs")
import random as r

class main:
    def __init__(self, node):
        self.node = node
        self.funcs = {
        "rundIntg":self.fn_randInt,
        "rundFlotg":self.fn_rand,
        "rundListh":self.fn_randList,
        }
        self.vars = {
        "version":"0.5.0",
        }
        self.metodes = {
        "mexnos":self.mt_suffle,
        "rundObjess":self.mt_ch,
        }
        self.classes = {}
        
    def fn_randInt(self,a):
        return r.randint(a.minev,a.manev)
        
    def fn_rand(self,a):
        return r.triangular(a.minev,a.manev)
        
    def mt_suffle(self,ar):
        return r.shuffle(ar)
        
    def mt_ch(self,ar):
        return r.choice(ar)
        
    def fn_randList(self,a,c,d):
        l = []
        d = d.lower()
        b = a.manev
        a = a.minev
        for i in range(0,c):
            if d == "intg":
                l.append(r.randint(a,b))
            elif d == "flotg":
                l.append(r.triangular(a,b))
        return l
            
if __name__ == "__main__":                
    pass

