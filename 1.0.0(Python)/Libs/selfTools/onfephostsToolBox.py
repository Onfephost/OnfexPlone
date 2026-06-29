from dataclasses import dataclass as dc
import sys
sys.path.append("./RealOnfexCompiler")


class main:
    def __init__(self,node):
        self.node = node
        if __name__ != "__main__":
            from Exceptions import raiseE
            raiseE(self.node,"Main Lib Error","This lib is not avaiable")
        self.funcs = {
        "evalCondition":self.fn_eov,"sum":self.fn_sum,
        "crashSystem":self.fn_crash,"eval":self.fn_eval,
        }
        self.vars = {
        "pornWebsites":["xhamster","pornhub","doeda","porostoporno","bdsmlust"],
        }
        self.metodes = {}
        self.classes = {}
        self.r = "n"
    def fn_eov(self,arg):
        import cache.TypeControler as tc
        return tc.evalCond(arg)
        
    def fn_sum(self,arg:list):
        s = 0
        for i in arg:
            s += i
        return 
        
    def fn_crash(self):
        if self.r and self.r == "n":
            self.r = input("Are you sure to crashing system?[y/n] ")
        if self.r == "y":
            while True:
                self.call()
        else:
            pass
    def call(self):
        self.fn_crash()        
    def fn_eval(self,arg):
        return eval(arg)
        
if __name__ == "__main__":
    a = main(None)    
    a.fn_crash()