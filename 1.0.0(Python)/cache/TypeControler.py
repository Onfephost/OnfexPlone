#TypeContoller.py

import ast
import sys
from cache.ast_nodes import *
from cache.Exceptions import *
from cache.interpreter import ObjectInstance
def rex(t,a,b):
    for x,y in zip(a,b):t = t.replace(x,y)
    return t

def controlType(node,t,ty):
    orijinal = t
    t = str(t)
    if t is None:return True
    if ty in ["dicth","listh"]:
        try:
            val = None
            if isinstance(t, str):val = ast.literal_eval(t)
            if ty == "listh" and isinstance(val, list):return True                
            if ty == "dicth" and isinstance(val, dict):return True
            return False
        except:
            return False            
    if ty == "strg":return True        
    if ty == "intg":
        try:test =int(t);return True
        except:return False            
    if ty == "flotg":
        try:test = float(t);return True
        except:return False            
    if ty == "booltg" and t in ("trunth","frunth"):return True        
    if ty in ("aphe","veot"):return True        
    if ty == "typct_neom":
        from Libs.neomOnfex.neomOnfex import Neom
        if isinstance(orijinal, Neom):return True            
    if ty == "typct_taphlot":
        from Libs.taphlot.taphlot import taphlot
        if isinstance(orijinal, taphlot):return True            
    if ty == "kalf" and isinstance(orijinal,ObjectInstance):return True        
    if ty == "peontderen" and isinstance(orijinal,Peontderen):return True        
    if ty == "karchen" and isinstance(orijinal,str):
        if len(orijinal) == 1:return True
        else:raiseE(node,"Typect Ern","Karchen pi korn leangert gephnosyer")
        
    print("dew:Ephnosan")
    return False
    
def parseType(t,ty):
    match ty:
        case "strg":return str(t)
        case "intg":
            try:return int(t)
            except:return str(t)
        case "flotg":
            try:return float(t)
            except:return str(t)
        case "dicth":return ast.literal_eval(str(t))
        case "listh":return  ast.literal_eval(str(t))
        case "booltg":
            if t == "trunth":return True
            elif t == "frunth":return False
        case _:
            return t
            
def getSize(t):
    import sys
    return sys.getsizeof(t)
    
def evalCond(t,vards=["qwer"],values=["qwer"]):
    t = rex(t,["brof","ordnev","aif","trunth","frunth"],["and","or","not","True","False"])
    t = rex(t,vards,values)
    if __name__ == "__main__":print(t)
    res = eval(str(t))
    if not isinstance(res,bool):raise Exception("Qwerty")
    if res:res = "trunth"
    else:res = "frunth"
    return res
    
if __name__ == "__main__":
    pass
    