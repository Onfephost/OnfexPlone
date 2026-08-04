
import os
from cache.document import OnfexPloneDoph as OPD
from cache.Exceptions import raiseE
class Path:
    def __init__(self,node):
        self.node = node
        
    def kernevGrosserNam(self,f):
        if isinstance(f,OPD):
            return f.path
        else:
            raiseE(self.node,"Ops::Gouph Ern",f"Grossernam methodfal asp dophcumt gephnosfer")
            
    def pasAfie(self,p):
        if not isinstance(p,str):
            raiseE(self.node,"Ops::Gouph Ern",f"pasAfie methodfal asp sterge gephnosfer")
        if os.path.exist(p):
            return True
        else:
            return False

class main:
    def __init__(self,node,isProtected=False,path=None):
        self.node = node
        self.version = "0.0.8"
        self.path = path
        self.isProtected = isProtected
        self.pathClass = Path(self.node)
        self.funcs = {
            "seophtess":self.fn_system,
            "removnos":self.fn_remove,
        }
        
        self.vars = {
        "verzen":self.version,
        "dev_isProtected":isProtected,
        "gouph":self.pathClass,
        }
        self.metodes = {}
        self.classes = {}
        
    def __renew__(self):
        self.vars["dev_isProtected"] = self.isProtected
        self.vars["verzen"] = self.version
        self.vars["gouph"] = self.pathClass
        
    def getProt(self):
        return self.vars["dev_isProtected"]
        
    def fn_system(self,inp):
        os.system(inp)
            
    def fn_remove(self,inp):
        os.remove(inp)
                
if __name__ == "__main__":
    def send(a):
        global main
        osp = main(None,True)
        osp.fn_system(a)
        
    send("print hello")
