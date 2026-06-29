#Lib Meathes
from dataclasses import dataclass as dc
import math
from cache.Exceptions import *

def multies(node,x):
    if not isinstance(x,int):
        raiseE(node[0],"Meathess Ern",
               "Neom asp intg mut bephnosfer")
    if x <= 0:
        raiseE(node[0],"Meathess Ern",
               "Neom asp 0 brof qenev mut bephnosfer")
    if isinstance(x,float) and round(x) != x:
        raiseE(node[0],"Meathess Ern",
               "Neom asp integ mut bephnosfer")
    x = int(x)
    c = 2
    if x == 1:return 1
    for i in range(2,x):
        if (x%i)==0:c+=1
    return c

class main:
    def __init__(self,node=None):
        self.node = node
        self.vars = {
        "version":"1.0.2","infy":math.inf,"e":math.e,"NkN":math.nan,
        }
        self.classes = {}
        self.metodes = {
            "brodernos":self.mt_interpole,
            "minev":self.mt_min,
            "manev":self.mt_max,
        }
        self.funcs = {
            "kosnev":self.fn_cos, "akosnev":self.fn_acos, "kosihnev":self.fn_cosh, "akosihnev":self.fn_acosh,
            "sintnev":self.fn_sin,"asintnev":self.fn_asin,"sinihnev":self.fn_sinh,"asinihnev":self.fn_asinh,
            "tantnev":self.fn_tan,"atantnev":self.fn_atan,"tanihnev":self.fn_tanh,"atanihnev":self.fn_atanh,
            "pasEsalNeom":self.fn_asal,"expe":self.fn_exp,
            "tünkernev":self.fn_tunc,
            "seaketnev":self.fn_ceil,
            "fotkatnev":self.fn_floor,
        }
    def __renew__(self):
        self.vars["infy"] = math.inf
        self.vars["e"] = math.e
        self.vars["NkN"] = math.nan
        
    def mt_min(self,val,must):
        if val <= must:
            return must
        else:
            return val
        
    def mt_max(self,val,must):
        if val >= must:
            return must
        else:
            return val
            
    def fn_cos(self,val):return math.cos(val)
    def fn_sin(self,val):return math.sin(val)
    def fn_tan(self,val):return math.tan(val)
        
    def fn_cosh(self,val):return math.cosh(val)
    def fn_sinh(self,val):return math.sinh(val)
    def fn_tanh(self,val):return math.tanh(val)
        
    def fn_acos(self,val):return math.acos(val)
    def fn_asin(self,val):return math.asin(val)
    def fn_atan(self,val):return math.atan(val)
        
    def fn_acosh(self,val):return math.acosh(val)
    def fn_asinh(self,val):return math.asinh(val)
    def fn_atanh(self,val):return math.atanh(val)
        
    def fn_radians(self,val):return math.radians(val)
    def fn_degrees(self,val):return math.degrees(val)
        
    def fn_tunc(self,val):return math.trunc(val)
    def fn_floor(self,val):return math.floor(val)
    def fn_ceil(self,val):return math.ceil(val)

    def fn_asal(self,val):
        if multies(self.node,val) == 2:return True
        else:return False
        
    def fn_exp(self,val):
        return math.exp(val)

    def mt_interpole(self,val,minimum,maximum):
        if val < minimum:return minimum
        elif val > maximum:return maximum
        else:return val
