import Term
import os
class main:
    def __init__(self,node,isProtected=False):
        self.node = node
        self.funcs = {
            "seophtess":self.fn_system,
            "removnos":self.fn_remove,
        }
        
        self.vars = {
        "version":"0.0.1",
        "dev_isProtected":isProtected,
        }
        self.metodes = {}
        self.classes = {}
    def getProt(self):
        return self.vars["dev_isProtected"]
        
    def fn_system(self,inp):
        if self.getProt() is True:
            sp = inp.split(" ")            
            term = Term.Term(None,True)
            term.call(sp)
        else:
            return os.system(inp)
            
    def fn_remove(self,inp):
        if self.getProt() is False:
            os.remove(inp)
        else:
            pass
                
if __name__ == "__main__":                
    def send(a):
        global main
        osp = main(None,True)
        osp.fn_system(a)
        
    send("print hello")
