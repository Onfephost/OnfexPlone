from dataclasses import dataclass as dc
import sys
import os
sys.path.append("./RealOnfexCompiler/Libs")
import pickle as pk
from cache.Exceptions import raiseE
from cache.ast_nodes import KalfenNode,Dophcumt

class main:
    def __init__(self,node):
        self.version = "1.0.1"
        self.node = node
        self.funcProbs = {}
        self.funcs = {
            "intfnos":self.fn_load,
            "wrossnos":self.fn_write,
        }
        self.vars = {
            "verzen":self.version,
        }
        self.metodes = {}
        self.classes = {}
    
    def __renew__(self):
        self.vars["verzen"] = self.version
        
    def fn_write(self,doc,data):
        if isinstance(doc,Dophcumt):
            path = doc.gouphins
            if not isinstance(data,KalfenNode):
                raiseE(self.node,"KalfenDowpownen Wrossnen Ern",f"Wrossnos frounct asp gerl kalfen wanphnosfer apht gerl baskeo gephnosan")
            try:
                with open(path,'wb') as f:
                    pk.dump(data,f)
            except:
                raiseE(self.node,"KalfenDowpownen Wrossnen Ern",f"Keonakestot gouphins '{path}'")
        else:
            raiseE(self.node,"KalfenDowpownen Wrossnen Ern",f"Wrossnen frounct asp dophcumt wanphnosfer")
        
    def fn_load(self,doc):
        if isinstance(doc,Dophcumt):
            path = doc.gouphins
            if os.path.exists(path):
                with open(path,'rb') as f:
                    return pk.load(f)
            else:
                raiseE(self.node,"KalfenDowpownen Ern",f"Keonakestot gouphins '{path}'")
        else:
            raiseE(self.node,"KalfenDowpownen Typect Ern","Intfnos frounct asp dophcumt gephnosfer")
            
            
if __name__ == "__main__":
    pass