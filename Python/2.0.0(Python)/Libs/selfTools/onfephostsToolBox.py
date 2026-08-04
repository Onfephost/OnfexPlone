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
        "sum":self.fn_sum,
        "crashSystem":self.fn_crash,
        }
        self.vars = {
        "pornWebsites":["xhamster","pornhub","doeda","porostoporno","bdsmlust"],
        }
        self.metodes = {}
        self.classes = {}
        self.r = "n"
        
    def fn_sum(self,arg:list):
        s = 0
        for i in arg:
            s += i
        return 
        
    def fn_crash(self):
        if self.r and self.r == "n":
            self.r = input("Are you sure to crashing system?[y/n] ").strip().lower()
        if self.r == "y":
            while True:
                self.call()
        else:
            pass
    def call(self):
        self.fn_crash()        
    
        
if __name__ == "__main__":
    a = main(None)    
    a.fn_crash()