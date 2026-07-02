import sys
from dataclasses import dataclass as dc
import numpy as np
import random as ra
from sorter import *
class Neom:
    def __init__(self,a):
        self.veot = a
        self.data = a
        
    def __repr__(self):
        return f"[{' '.join([str(a) for a in self.veot])}]"
    
class main:
    def __init__(self,node):
        self.node = node
        self.vars = {
        "version":"0.9.1","extraFeatures":False,
        }
        self.metodes = {
        "adnos":self.mt_plus,"olnos":self.mt_times,"divnos":self.mt_divide,"ednos":self.mt_minus,"sumnev":self.mt_sum,
        "listhPerl":self.mt_list,        
        "rephSheapenos":self.mt_reshape,
        "kratnev":self.mt_sort,
        }
        self.funcs = {
        "rundomArrey":self.fn_randArr,
        }
        self.classes = {
        "neom":Neom,
        "Neom":Neom,
        }
        self.main()
        
    def main(self):
        #Private
        self.neoms = {}
    
    def turn(self,val):
        sys.path.append("./RealOnfexCompiler")
        from Exceptions import raiseE
        if isinstance(val, list):
            return Neom(val)
        elif isinstance(val, Neom):
            return val
        else:
            raiseE(self.node,"NeomOnfex Ern",f"listh or Neom  wraithvognosan apht {type(val)} gephvognosan")

    def mt_sum(self,a):
        s = 0
        for i in a.veot:
            s += i
        return s
        
    def mt_plus(self,obj,b):
        l = []
        b = self.turn(b)
        obj = self.turn(obj)
        for x,y in zip(obj.data,b.data):
            l.append(x+y)
        return Neom(l)
        
    def mt_times(self,a,b):
        l = []
        a = self.turn(a)
        b = self.turn(b)
        for x,y in zip(a.data,b.data):
            l.append(x*y)
        return Neom(l)
    
    def mt_divide(self,a,b):
        l = []
        a = self.turn(a)
        b = self.turn(b)
        for x,y in zip(a.data,b.data):
            l.append(x/y)
        return Neom(l)
        
    def mt_minus(self,a,b):
        l = []
        a = self.turn(a)
        b = self.turn(b)
        for x,y in zip(a.data,b.data):
            l.append(x-y)
        return Neom(l)
        
    def mt_list(self,arg):
        if isinstance(arg, Neom):
            return arg.veot
        elif isinstance(arg, list): 
            return arg
            
    def mt_sort(self,obj):
        res = sort(obj.veot)
        return Neom(res)
        
    def fn_randArr(self,prop:list):
        ty = prop[3].lower()
        Len = prop[2]
        Min = prop[0]
        Max = prop[1]
        l = []
        for i in range(0,Len):
            if ty == "intg":
                l.append(ra.randint(Min,Max))
            elif ty == "flotg":
                l.append(ra.triangular(Min,Max))
        return l
                
    def mt_reshape(self,ar,*args):
        ar = self.turn(ar)
        l = np.array(ar.data)
        l = l.reshape(*args)
        return Neom(l.tolist())
        
if __name__ == "__main__":
    pass
    print(None is None)
