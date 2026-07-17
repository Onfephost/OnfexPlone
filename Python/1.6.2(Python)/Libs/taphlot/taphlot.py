#Lib:Taphlot
from dataclasses import dataclass as dc


class taphlot:
    def __init__ (self,i,t,d):
        self.nam = i
        self.veot = d
        self.kread = t
        
    def show(self):
        maxlen,maxlen2 = (0,0)
        print(self.kread)
        for a,b in zip(self.veot.keys(),self.veot.values()):
            if len(str(a)) > maxlen:
                maxlen = len(str(a))
            if len(str(b)) > maxlen2:
                maxlen2 = len(str(b))
        print("+","—"*(maxlen+12+maxlen2),"+",sep='')
        for a,b in zip(self.veot.keys(),self.veot.values()):
            print("| ",a," "*(maxlen-len(str(a)))," |:| ",b," "*(maxlen2-len(str(b))),"|")
        print("+","—"*(maxlen+12+maxlen2),"+",sep='')
        
    def wervenkentnos(self):
        self.show()
        
class main:
    def __init__(self,node=None):
        self.node = node
        self.version = "1.5.7"
        self.funcProbs = {}
        self.funcs = {
            "cesnos":self.fn_create,
        }
        self.vars = {
            "verzen":self.version,
        }
        self.metodes = {
        "bonWernos":self.mt_show,
        "veotGephnos":self.mt_getData,
        }
        self.classes = {
        "taphlot":taphlot,
        }
        self.main()
        
    def __renew__(self):
        self.vars["verzen"] = self.version
    
    def main(self):
        self.taphlotrar = {}
        pass
        
    def fn_create(self,n,t,d):
        self.taphlotrar[n] = taphlot(n,t,d)
        return self.taphlotrar[n]
    
    def fn_delete(self,n):
        if n in self.taphlotrar:
            del self.taphlotrar[n]
        else:
            print(f"{n} is not in taphlotrar")
            
    def fn_get(self,n):
        if n in self.taphlotrar:
            return self.taphlotrar[n]
        else:
            print(f"{n} is not in taphlotrar")
            return None
        
    def mt_show(self,tf):     
        if isinstance(tf, taphlot):tf.show()
        elif isinstance(tf,str) and tf in self.taphlotrar:self.taphlotrar[tf].show()
            
    def mt_getData(self,obj):
        return obj.veot
                
if __name__ == "__main__":                
    new = main()
    a=new.fn_create("a","Test",{"a":5,"b":6})
    b = new.mt_show
    new.mt_show(a)


