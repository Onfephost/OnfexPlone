import sys
from dataclasses import dataclass as dc
import random as ra
import os

class Neom:
    def __init__(self,a):
        self.veot = a
        self.data = a
        
    def __out__(self):
        return f"neomArrey([{' '.join([str(a) for a in self.veot])}])"
    
    def __type__(self):
        return "<NeomArrey>"
    
    def __onfex_eval__(self):
        return self.data
    
class main:
    def __init__(self,node):
        self.node = node
        self.vars = {
        "verzen":"0.9.6",
        }
        self.metodes = {
        "sumnev":self.mt_sum,
        "listhPerl":self.mt_list,        
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
        "neom":Neom,
        "Neom":Neom,
        }
        self.main()
        
    def main(self):
        #Private
        self.neoms = {}
        
    def __renew__(self):
        self.vars["verzen"] = "0.9.6"
   
    def turn(self,val,mustBeNeom=False):
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
            raiseE(self.node,"NeomOnfex Ern",f"NeomArrey wraithvognosan apht listh gephvognosan")
        else:
            raiseE(self.node,"NeomOnfex Ern",f"listh or NeomArrey wraithvognosan apht {type(val)} gephvognosan")

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
    os.system("clear")
    new = main(None)
    res=new.fn_randArr([1,10,8,"intg"])
    print(new.mt_reshape(Neom(res),2).data)
