from dataclasses import dataclass as d
import sys
sys.path.append("./RealOnfexCompiler")

class main:
    def __init__(self,node):
        self.node = node
        self.funcs = {
            "cesnos":self.fn_cre,
            "gephnos":self.fn_get,
            "delnos":self.fn_del,
        }
        self.vars = {
            "verzen":"0.1.1"
        }
        self.metodes = {}
        self.classes = {}
        self.dct = {}

    def __renew__(self):
        self.vars["verzen"]="0.1.1" 
    
    def fn_cre(self,a,b):
        self.dct[a] = b
    
    def fn_get(self,a):
        self.dct.get(a,None)
    
    def fn_del(self,a):
        if a in self.dct:
            del self.dct[a]
            
if __name__ == "__main__":
    pass
