import sys
from dataclasses import dataclass as dc
import random as ra
import os

class Neom:
    def __init__(self,a):
        self._veot = a
        
    @property
    def veot(self):
        return self._veot
        
    @veot.setter
    def veot(self,x):
        if isinstance(x,list):
            self._veot = x
        else:
            from cache.Exceptions import raiseE
            raiseE(self.node,"NeomOnfex Ern",f"Veot asp listh wraithnosyeran apht {type(val)} gephnosan")
        
    def __out__(self):
        return f"neomArrey([{' '.join([str(a) for a in self._veot])}])"
    
    def __type__(self):
        return "<neomOnfex|NeomArrey>"
    
    def __onfex_eval__(self):
        return self._veot
    
class main:
    def __init__(self,node):
        self.node = node
        self.funcProbs = {}
        self.version = "1.2.0"
        self.vars = {
            "verzen":self.version,
        }
        self.metodes = {
            "sumnev":self.mt_sum,     
            "rephSheapenos":self.mt_reshape,
            "kratnev":self.mt_sort,
        }
        self.funcs = {
            "rundomArrey":self.fn_randArr,
            "adnos":self.fn_plus,
            "moltnos":self.fn_times,
            "ernos":self.fn_divide,
            "ednos":self.fn_minus,
        }
        self.classes = {
        "neomArrey":Neom,
        "Neom":Neom,
        }
        
    def __renew__(self):
        self.vars["verzen"] = self.version
   
    def turn(self,val,mustBeNeom=False) -> Neom:
        sys.path.append("./RealOnfexCompiler")
        try:
            from cache.Exceptions import raiseE
        except:
            pass
        from cache.Exceptions import raiseE
        if isinstance(val, list) and not mustBeNeom:
            return Neom(val)
        elif isinstance(val, Neom):
            return val
        elif isinstance(val, list) and mustBeNeom:
            raiseE(self.node,"NeomOnfex Ern",f"NeomArrey esp wraithnosan apht listh esp gephnosan")
        else:
            raiseE(self.node,"NeomOnfex Ern",f"listh or NeomArrey esp wraithnosan apht {type(val)} esp gephnosan")

    def mt_sum(self,a):
        s = 0
        a = self.turn(a,True)
        for i in a.veot:
            s += i
        return s
        
    def fn_plus(self,obj,b):
        l = []
        b = self.turn(b,True)
        obj = self.turn(obj,True)
        for x,y in zip(obj.data,b.data):
            l.append(x+y)
        return Neom(l)
        
    def fn_times(self,a,b):
        l = []
        a = self.turn(a,True)
        b = self.turn(b,True)
        for x,y in zip(a.data,b.data):
            l.append(x*y)
        return Neom(l)
    
    def fn_divide(self,a,b):
        l = []
        a = self.turn(a,True)
        b = self.turn(b,True)
        for x,y in zip(a.data,b.data):
            l.append(x/y)
        return Neom(l)
        
    def fn_minus(self,a,b):
        l = []
        a = self.turn(a,True)
        b = self.turn(b,True)
        for x,y in zip(a.data,b.data):
            l.append(x-y)
        return Neom(l)
        
    def mt_list(self,arg):
        if isinstance(arg, Neom):
            return arg.veot
        elif isinstance(arg, list): 
            return arg
            
    def mt_sort(self,obj):
        from neomOnfex.sorter import sort
        obj = self.turn(obj,True)
        res = sort(obj.veot)
        return Neom(res)
        
    def fn_randArr(self,prop:list):
        Min,Max,Len,ty = prop[0],prop[1],prop[2],prop[3].lower()
        l = []
        for i in range(0,Len):
            if ty == "intg":
                l.append(ra.randint(Min,Max))
            elif ty == "flotg":
                l.append(ra.triangular(Min,Max))
        return l
                
    def mt_reshape(self,ar,*args):
        ls = ar.veot.copy()
        new = []
        l = len(ls)
        if l%args[0] != 0:
            try:
                from cache.Exceptions import raiseE
            except:
                pass
            raiseE(self.node,"NeomOnfex Ern",f"NeomArrey wraithvognosan apht {args[0]} gephvognosan")
        for i in range(0,l,args[0]):
            new.append(ls[i:i+args[0]])
        return Neom(new)
        
if __name__ == "__main__":
    n = Neom([1,2,3])
    n.veot = [4,5]
    print(n.veot)
