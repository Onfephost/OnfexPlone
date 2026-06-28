from dataclasses import dataclass
from typing import List, Any,Optional

def detect_yield(block):
    for stmt in block.statements:
        if isinstance(stmt, Yield):
            return True
        if hasattr(stmt, "body") and stmt.body:
            if detect_yield(stmt.body):
                return True
        if hasattr(stmt, "elifBodies"):
            if stmt.elifBodies is not None:
                for el in stmt.elifBodies:
                    if detect_yield(el.body):
                        return True
        if hasattr(stmt, "else_body") and stmt.else_body:
            if detect_yield(stmt.else_body):
                return True
    return False
    
class ASTNode:
    def __init__(self, token=None,value=None):
        self.token = token
        self.typct = self.__class__.__name__
        if token:
            self.pos = token.pos
            self.line = token.line
            self.col = token.col
        else:
            self.pos = [0,0]
            self.line = 0
            self.col = 0
        self.VAL = value

class DEFINE(ASTNode):
    def __init__(self,token):
        super().__init__(token)
        
class DATATYPE(ASTNode):
    def __init__(self,token):
        super().__init__(token)
        
class Block(DEFINE):
    def __init__(self,token,items):
        super().__init__(token)
        self.statements = items

class Mehen(DEFINE):
    def __init__(self,token,items):
        super().__init__(token)
        self.statements = items
		
class Int(DATATYPE):
    def __init__(self,token,name):
        super().__init__(token)
        self.value = int(name)
        self.typct = "intg"
        
    def __out__(self):
        return self.value
    
    def __type__(self):
        return f"<typect|Integ>"
        
    def __onfex_value__(self):
        return self.value
    
class Float(DATATYPE):
    def __init__(self,token,name):
        super().__init__(token)
        self.value = name
        self.typct = "flotg"
        
    def __out__(self):
        return self.value
    
    def __type__(self):
        return f"<typect|Flotg>"
        
    def __onfex_value__(self):
        return self.value
                
class Type(DATATYPE):
    def __init__(self,token,name):
        super().__init__(token)
        self.value = name
        self.ty = None
        match self.value:
            case "strg":
                self.ty = str
            case "flotg":
                self.ty = float
            case "booltg":
                self.ty = bool
            case "intg":
                self.ty = int
            case "dicth":
                self.ty = dict
            case "listh":
                self.ty = list
            case _:
                self.ty = None
        
    def __out__(self):
        return f"<typect|{self.value.upper()}>"
    
    def __type__(self):
        return f"<typect|Typect>"
        
    def __onfex_value__(self):
        return self.value

class String(DATATYPE):
    def __init__(self,token,name):
        super().__init__(token)
        self.value = name
        self.typct = "strg(sterge)"

    def __out__(self):
        return self.value
    
    def __type__(self):
        return f"<typect|Strg>"
        
    def __onfex_value__(self):
        return self.value
        
class Iter(DATATYPE):
    def __init__(self,token,typ,value):
        super().__init__(token)
        self.typect = typ #listh,cerderen,dophcumt
        self.valtue = value
        
    def __iter__(self):
        return iter(self.valtue)

    def __out__(self):
        return "<typect|Iterfal>"
    
    def __type__(self):
        return f"<typect|Iterfal>"
        
    def __onfex_value__(self):
        return self

class Bool(DATATYPE):
    def __init__(self,token,name):
        super().__init__(token)
        self.value = bool(name)
        self.typct = "booltg"

    def __out__(self):return "trunth" if self.value else "franth"
    
    def __type__(self):return f"<typect|Booltg>"
        
    def __onfex_value__(self):return self.value
        
class Null(DATATYPE):
    def __init__(self,token):
        super().__init__(token)
        self.value = None
        self.typct = "noph"
    
    def __out__(self):return "noph"
    
    def __type__(self):return f"<typect|Noph>"
        
    def __onfex_value__(self):return self.value
        
class Peontderen(DATATYPE):
    def __init__(self,adr,vn,v):
        self.edregh = adr
        self.valtNam = vn
        self.valtue = v
        self.mot = ""

    def __out__(self):
        return f"<{self.mot}Peontderen('edregh':{self.edregh}, 'valtNam':{self.valtNam}, 'valtue':{self.valtue})>"
    
    def __type__(self):return f"<typect|Peontderen>"
        
    def __onfex_value__(self):return self
        
class Dophcumt(DATATYPE):
    def __init__(self,token,name,path):
        super().__init__(token)
        self.nam = name
        self.gouphins = path
        
    def __repr__(self):return f"<typect|Dophcumt>"

class Range(ASTNode):
    def __init__(self, token, mn, mx,s):
        super().__init__(token)
        self.mn = mn
        self.mx = mx
        self.s = s

    def __iter__(self):
        return RangeIter(self.mn, self.mx,self.s)

class RangeIter:
    def __init__(self, mn, mx,s):
        self.cur = mn
        self.mx = mx
        self.s = s

    def __next__(self):
        if self.cur < self.mx:
            val = self.cur
            self.cur += self.s
            return val
        else:
            raise StopIteration
            
class Zip(ASTNode):
    def __init__(self, token, par1, par2):
        super().__init__(token)
        self.a = par1
        self.b = par2        
    def __iter__(self):return ZipIter(self.a, self.b)
        
class ZipIter:
    def __init__(self, a, b):
        self.cur = 0
        self.l = []
        for i in zip(a,b):
            self.l.append(i)
            
    def __next__(self):
        if len(self.l) > self.cur:
            val = self.l[self.cur]
            self.cur += 1
            return val
        else:
            raise StopIteration

class Variable(ASTNode):
    def __init__(self,token,name):
        super().__init__(token)
        self.name = name
        self.typct = "valt"
        
class Yield(ASTNode):
    def __init__(self, token, value):
        super().__init__(token)
        self.value = value  
        
class PointerGet(ASTNode):
    def __init__(self,token,name):
        super().__init__(token)
        self.var = name
        
class PointerDel(ASTNode):
    def __init__(self,token,name):
        super().__init__(token)
        self.ptr = name

class Assign(ASTNode):
    def __init__(self,token,name,typ,val):
        super().__init__(token)
        self.var = name
        self.type_hin = typ
        self.typct = typ
        self.value = val

class Call(ASTNode):
    def __init__(self, token, name, value):
        super().__init__(token)
        self.node = name
        self.args = value
        
class LibCall(ASTNode):
    def __init__(self, token, lib,name, value):
        super().__init__(token)
        self.lib = lib
        self.func = name
        self.args = value

class LibMethodCall(ASTNode):
    def __init__(self, token, lib, obj, func, args):
        super().__init__(token)
        self.obj = obj
        self.lib = lib
        self.func = func
        self.args = args

class LibVariable(ASTNode):
    def __init__(self,token,lib,name):
        super().__init__(token)
        self.lib = lib
        self.name = name

class If(ASTNode):
    def __init__(self,token,cond,body,elbody=None,else_body=None):
        super().__init__(token)
        self.condition = cond
        self.body = body
        self.elifBodies = elbody
        self.else_body = else_body
        
class Elif(ASTNode):
    def __init__(self,token,cond,body):
        super().__init__(token)
        self.cond = cond
        self.body = body

class Func(DEFINE):
    def __init__(self,token,name,par,var=None,bo=None):
        super().__init__(token)
        self.name = name
        self.params = par
        self.vararg = var
        self.body = bo
        self.has_yield = detect_yield(self.body)
        
    def __out__(self):
        return f"<{self.name}|frounct|__mehen__>"
    
    def __type__(self):
        return f"<frounct|__mehen__>"
        
    def __onfex_value__(self):
        return self

class ListLiteral(DATATYPE):
    def __init__(self,token,items):
        super().__init__(token)
        self.items = items
        
    def __out__(self):
        return (self.items)
    
    def __type__(self):
        return f"<typect|Listh>"
        
    def __onfex_value__(self):
        return list(self.items)

class DictLiteral(DATATYPE):
    def __init__(self,token,items):
        super().__init__(token)
        self.pairs = items

    def __out__(self):
        return self.pairs
    
    def __type__(self):
        return f"<typect|Dicth>"
        
    def __onfex_value__(self):
        return self.pairs      

class MethodCall(ASTNode):
    def __init__(self, token, obj, name, args):
        super().__init__(token)
        self.obj = obj
        self.name = name
        self.args = args
        
class IndexAccess(ASTNode):
    def __init__(self, token, target, index):
        super().__init__(token)
        self.target = target
        self.index = index
        
class IndexAssign(ASTNode):
    def __init__(self, token, target, index, val):
        super().__init__(token)
        self.target = target 
        self.index = index
        self.value = val
        
class IndexDelete(ASTNode):
    def __init__(self, token, obj,index):
        super().__init__(token)
        self.target = obj
        self.index = index
        
class MemberAccess(ASTNode):
    def __init__(self, token, obj, name):
        super().__init__(token)
        self.obj = obj
        self.atr = name
        
class BinOp(ASTNode):
    def __init__(self, left, op, right, token=None):
        super().__init__(token)
        self.left = left
        self.op = op
        self.right = right

class UnaryOp(ASTNode):
    def __init__(self, op, operand, token=None):
        super().__init__(token)
        self.op = op
        self.operand = operand
        
class ImportAs(DEFINE):
    def __init__(self, token,par1,par2):
        super().__init__(token)
        self.lib = par1
        self.As= par2
        
class Break(ASTNode):
    def __init__(self, token):
        super().__init__(token)
        
class Continue(ASTNode):
    def __init__(self, token):
        super().__init__(token)
        
class Return(ASTNode):
    def __init__(self, token,par1):
        super().__init__(token)
        self.value = par1
        
class DataAttr(ASTNode):
    def __init__(self, token,par1,par2):
        super().__init__(token)
        self.obj = par1
        self.atr = par2
        
class CallAttr(ASTNode):
    def __init__(self, token,par1,par2,par3):
        super().__init__(token)
        self.obj = par1
        self.atr = par2
        self.args = par3

class ParamAssign(ASTNode):
    def __init__(self, token,par1,par2):
        super().__init__(token)
        self.param = par1
        self.value = par2
        
class ForpNode(ASTNode):
    def __init__(self, token, block1,e, block2):
        super().__init__(token)
        self.var = block1
        self.enter = e# sorgu ve işlem bloğu
        self.body = block2  # olay bloğu
        
class Thread(ASTNode):
    def __init__(self, token, func,val, block2):
        super().__init__(token)        
        self.func = func  # sorgu ve işlem bloğu
        self.name = func
        self.args = val
        self.body = block2  # olay bloğu
        
class KalfenNode(DEFINE):
    def __init__(self,token, i,name, body):
        super().__init__(token)
        self.parent = i 
        self.name = name
        self.body = body
    
    def __type__(self):
        return f"<{self.name}|kalfen|__mehen__>"
        
    def __out__(self):
        return f"<kalfen|__mehen__>"
        
    def __onfex_value__(self):
        return self

class ObjectNode(ASTNode):
    def __init__(self, token,name, args):
        super().__init__(token)
        self.name = name
        self.args = args
       
class MemberAssign(ASTNode):
    def __init__(self, token, obj, name, value):
        super().__init__(token)
        self.obj = obj
        self.name = name
        self.value = value

class While(ASTNode):
    def __init__(self, token,c, b):
        super().__init__(token)
        self.cond = c
        self.body = b
        
class ModulCall(ASTNode):
    def __init__(self, token, mod, name, args):
        super().__init__(token)
        self.modul = mod
        self.name = name
        self.args = args
    
class ModulVariable(ASTNode):
    def __init__(self, token, mod, name):
        super().__init__(token)
        self.modul = mod
        self.name = name
        
class ModulImport(ASTNode):
    def __init__(self, token, mod,name):
        super().__init__(token)
        self.mod = mod
        self.As = name
        
class FutureFunc(DEFINE):
    def __init__(self,token,props,name,par,bo=None):
        super().__init__(token)
        self.probs = props
        self.name = name
        self.params = par
        self.vararg = None
        self.body = bo
        
    def __out__(self):return f"<{self.name}|frutupheFrounct|__mehen__>"
    
    def __type__(self):return f"<frutupheFrounct|__mehen__>"
        
    def __onfex_value__(self):return self
        
class FutureCall(ASTNode):
    def __init__(self, token, name, value):
        super().__init__(token)
        self.name = name
        self.args = value

class TypingModul(ASTNode):
    def __init__(self, token, name, value):
        super().__init__(token)
        self.name = name
        self.value = value