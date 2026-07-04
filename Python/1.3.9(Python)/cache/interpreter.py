from cache.ast_nodes import *
from cache.environment import *
import os
from cache.Exceptions import *
import Libs.LibAcces as la
from cache.lexer import lex
from cache.parser import *
from cache.document import OnfexPloneDoph as OPD

def check(l,*args):
    rs = []
    for i in l:
        if isinstance(i,args):
            rs.append(True)
        else:
            rs.append(False)
    return rs
                    
def wrap(node,x,y=None,typ=None):
    if y is not None:x = (y.ty)(x)
    if isinstance(x,int) or typ is int :
        return Int(node,x)
    elif isinstance(x,float) or typ is float:
        return Float(node,x)
    elif isinstance(x,str) or typ is str:
        return String(node,x)
    elif isinstance(x,bool) or typ is bool:
        return Bool(node,x)
    elif isinstance(x,dict) or typ is dict:
        return DictLiteral(node,{wrap(node,a):wrap(node,b) for a,b in zip(x.keys(),x.values())})
    elif isinstance(x,list) or typ is list:
        return ListLiteral(node,[wrap(node,i) for i in x])
    elif isinstance(x,(DATATYPE,ObjectInstance,DEFINE)) and typ is None:
        return x
    elif x is None:
        return Null(node)
    return x

def typeOf(node,val):
    if hasattr(val, "__type__"):
        res = val.__type__()
        return res
    return Null(node)
# Helpers
class Spread:
    def __init__(self, _,values):self.values = values

class Frame:
    def __init__(self, statements):
        self.statements = statements
        self.index = 0
        
class GeneratorObj(DATATYPE):
    def __init__(self, interpreter, func_node, args):
        self.interpreter = interpreter
        self.func_node = func_node
        self.args = args
        self.env = Environment(interpreter.env)
        self.finished = False
        self.stack = [Frame(func_node.body.statements)]

        for i, p in enumerate(func_node.params):
            self.env.set(func_node, p, None, args[i])

    def __iter__(self):
        return self
        
    def __next__(self):
        if self.finished:raise StopIteration
        old_env = self.interpreter.env
        self.interpreter.env = self.env
        try:
            return self.run()
        except StopIteration:
            self.finished = True
            raise
        finally:
            self.interpreter.env = old_env

    def run(self):
        while self.stack:
            frame = self.stack[-1]
            if frame.index >= len(frame.statements):
                self.stack.pop();continue
            stmt = frame.statements[frame.index]
            frame.index += 1
            #  BLOCK
            if isinstance(stmt, Block):
                self.stack.append(Frame(stmt.statements))
                continue
            #  IF
            if isinstance(stmt, If):
                if self.interpreter.unwrap(self.interpreter.eval(stmt.condition)):
                    self.stack.append(Frame(stmt.body.statements))
                else:
                    handled = False
                    for el in stmt.elifBodies:
                        if self.interpreter.unwrap(self.interpreter.eval(el.cond)):
                            self.stack.append(Frame(el.body.statements))
                            handled = True
                            break
                    if not handled and stmt.else_body:
                        self.stack.append(Frame(stmt.else_body.statements))
                continue
            #  WHILE eower
            if isinstance(stmt, While):  # senin eower node'un
                if self.interpreter.unwrap(self.interpreter.eval(stmt.cond)):
                    # tekrar kendini stack'e koy loop
                    frame.index -= 1
                    self.stack.append(Frame(stmt.body.statements))
                continue
            #  FORP
            if isinstance(stmt, ForpNode):
                iterable = self.interpreter.eval(stmt.enter)
                if not hasattr(stmt, "_iter"):
                    stmt._iter = iter(iterable)
                try:
                    value = next(stmt._iter)
                except StopIteration:
                    stmt._iter
                    continue
                # tekrar loop
                frame.index -= 1
                if isinstance(value, (list, tuple)):
                    for name, val in zip(stmt.var, value):
                        self.env.set(stmt, name, None, val)
                else:
                    self.env.set(stmt, stmt.var[0], None, value)
                self.stack.append(Frame(stmt.body.statements))
                continue

            #  NORMAL EXEC
            try:
                self.interpreter.eval(stmt)
            except YieldEx as y:
                return y.value
            except RetEx:
                raise StopIteration
        raise StopIteration
        
    def __out__(self):
        return f"<typect|Iterfal|Frounct:{self.func_node.name}>"
    
    def __type__(self):
        return f"<typect|Iterfal|Frounct>"
        
    def __onfex_value__(self):
        return self
    
class BoundMethod:
    def __init__(self, obj, func):
        self.obj = obj
        self.func = func

    def __call__(self, *args):
        old_env = self.obj.interpreter.env
        self.obj.interpreter.env = Environment(self.obj.env)
        self.obj.interpreter.env.set(self.func, "srel", None, self.obj)
        # param check
        if len(self.func.params) != len(args):
            raiseE(self.func, "Input Error",f"Method takes {len(self.func.params)} param but {len(args)} given")
        for p, a in zip(self.func.params, args):
            self.obj.interpreter.env.set(self.func, p, None, a)
        try:
            result = self.obj.interpreter.eval(self.func.body)
        except RetEx as e:
            result = e.value
            
        self.obj.interpreter.env = old_env
        return result
# ObjectInstance
class ObjectInstance(DATATYPE):
    def __init__(self, cls_node, args, interpreter, nodex):
        self.cls_node = cls_node;self.interpreter = interpreter
        self.nodex = nodex;self.parent = None
        self.name = cls_node.name;self.env = Environment(interpreter.env)

        # INHERITANCE
        if cls_node.parent:
            parent_name = cls_node.parent.value
            parent_cls = interpreter.env.get_class(nodex, parent_name)
            if not parent_cls:
                raiseE(cls_node, "Inhentope Ern",f"Meoter kalfen {parent_name} asp froundvegnosan")
            # parent instance constructor ÇAĞRILMAZ
            self.parent = ObjectInstance.__new__(ObjectInstance)
            self.parent.cls_node = parent_cls
            self.parent.interpreter = interpreter
            self.parent.parent = None
            self.parent.env = Environment(interpreter.env)
            self.parent.name = parent_name
            #  parent env'i child'a AKTAR
            for k, v in self.parent.env.vars.items():
                self.env.vars[k] = v
            #  parent'a erişim super gibi
            self.env.set(cls_node, parent_name, None, self.parent)
        # CLASS ENV CACHE
        if not hasattr(cls_node, "env"):
            cls_node.env = Environment(interpreter.env)
        self.__env__ = cls_node.env
        # CONSTRUCTOR SADECE CHILD
        init_func = None
        for s in cls_node.body.statements:
            if isinstance(s, Func) and s.name == "__mehen__":
                init_func = s;break
        if init_func:
            old_env = interpreter.env
            interpreter.env = Environment(self.env)
            interpreter.env.set(init_func, "srel", None, self)
            expected = len(init_func.params)-1
            if expected != len(args):
                raiseE(cls_node, "Cernen Ern",
                       f"{expected} asp wraithvegnosan apht {len(args)} asp gephvegnosan")
            for p, a in zip(init_func.params[1:], args):
                interpreter.env.set(init_func, p, None, a)
            try:interpreter.eval(init_func.body)
            except RetEx:pass
            if self.parent:
                # parent constructor bittikten sonra tekrar merge et
                for k, v in self.parent.env.vars.items():
                    self.env.vars[k] = v
            interpreter.env = old_env
    # GET ATTR
    def get_attr(self, name, node=None):
        # instance vars
        if node is None:node = self.nodex
        if self.env.save_get(node, name):
            return self.interpreter.ptrOut(self.env.get(self, name))
        # dict
        if name == "__dicth__":
            ls = self.env.pointers.copy()
            if self.parent is not None:del ls[self.parent.name]
            return ls
        # class methods
        for s in self.cls_node.body.statements:
            if isinstance(s, Func) and s.name == name:
                return BoundMethod(self, s)
        # parent fallback
        if self.parent:
            method = self.parent.get_attr(name)
            if isinstance(method, BoundMethod):
                return BoundMethod(self, method.func)  #  KRİTİK
            return method
        raiseE(node, "Etriben Ern", f"{name} asp neat infernosins etriben")
    # -------------------------
    def set_attr(self, name, value):
        self.env.set(self, name, None, value)
    # -------------------------
    def __repr__(self):
        return f"<{self.cls_node.name}|Kalfen>"
    
    def __out__(self):
        return f"<Kalfen:{self.cls_node.name}>"
        
# Interpreter
class Interpreter:
    def __init__(self):
        self.env = Environment()
        self.path = ""
        self.mainDoc = None
        self.mainOn = False
        self.builtinFuncs = {
            "pyrintnos": self.fn_print,"clephnos": self.fn_clear,
            "morfenlnos": self.fn_ask,"typectChengnos": self.fn_tt,
            "lenev":self.fn_len,"per":Spread,"resnos":self.fn_raise,
            "keonInkleodnos": self.fn_unImport,
            "opnos":self.fn_open,"sortnos":self.sort,"tupez":Zip,
            "typect":typeOf,"pasTypect":self.fn_ic,"env":self.fn_env,
            "mappe":self.fn_map,"filret":self.fn_filt
        }
        self.metodes = {
            "adepnos":self.mt_append,
        }
        
        self.builtinClasses = {"apoter":Range}
        self.builtinThreads = {}
        # LibFeature
        self.libs = {}
        self.libs["quant"] = la.loadLib(None,"quant")
        self.libs["estorge"] = la.loadLib(None,"estorge")
        self.libs["estorge"].docName = "onfextrode"
        self.libs["onfextrode"] = la.loadLib(None,"onfextrode")
        self.moduls = {}
	
    def unwrap(self,val):
        if hasattr(val, "__onfex_value__"):
            res = val.__onfex_value__()
            if isinstance(res,list):res = [self.unwrap(a) for a in res]
            if isinstance(res,dict):res = {self.unwrap(a):self.unwrap(b) for a,b in zip(res.keys(),res.values())}
            return res
        return val
        
    def outOf(self,val):
        if hasattr(val, "__out__"):
            res = val.__out__()
            if isinstance(res,list):
                res = [str(self.outOf(a)) for a in res]
                res = "[" + ", ".join(res) + "]"
            if isinstance(res,dict):
                res = {str(self.outOf(a)):str(self.outOf(b)) for a,b in zip(res.keys(),res.values())}
                res = "{" + ", ".join([f"{k}: {v}" for k,v in res.items()]) + "}"
            return res
        return val
    
    def ptrOut(node,x):
        if isinstance(x,Peontderen):
            return x.valtue
        else:
            return (x)

    def getParams(self,fn):
            import inspect
            parC=inspect.signature(fn).parameters
            return parC
                    
    def eval(self, node):        
        # ---------------- BLOCK ----------------
        if isinstance(node, Block):
            for stmt in node.statements:
                self.eval(stmt)
            return None
        # ---------------- MAIN ----------------
        if isinstance(node, Mehen) and self.mainOn:
            return self.eval(node.statements)
        if isinstance(node, Mehen) and not self.mainOn:
            return None
        # ---------------- FUNCTION DEFINE ----------------;
        if isinstance(node, Func):
            self.env.set(node,node.name,None,node)
            return None
        # ---------------- FUTURE FUNCTION DEFINE ----------------
        if isinstance(node, FutureFunc):
            self.env.set(node,node.name,None,node)
            return None
        # ---------------- CLASS DEFINE ----------------
        if isinstance(node, KalfenNode):
            self.env.set(node,node.name,None,node)
            return None
        # ---------------- ASSIGN ----------------
        if isinstance(node, Assign):
            target = node.var
            value = (self.eval(node.value))
            if isinstance(target,Variable):
                self.env.set(node,target.name,None,value)
            elif isinstance(target,ModulVariable):
                self.moduls.get(node.modul).set(node,target.name,None,value)
            elif isinstance(target,IndexAccess):
                self.env.get(node,target.target.name)
                tg = self.unwrap(self.env.heap[self.env.pointers[target.target.name]].valtue)
                index = self.unwrap(self.eval(target.index))
                if index >= len(tg):
                    raiseE(node,"Idx Ern",f"Idenex asp banev frasta serl lenev")
                tg[index] = self.unwrap(value)
                self.env.heap[self.env.pointers[target.target.name]].valtue = tg
            elif isinstance(target,LibVariable):
                lib = self.libs.get(target.lib)             
                if not lib:
                    raiseE(node,"Lib Error",f"Unnamed lib {node.lib}")
                lib.node = target
                varss = lib.vars
                varss[target.name.value] = self.unwrap(value)
                if hasattr(lib,"__renew__"):
                    lib.__renew__()
            elif isinstance(target,MemberAccess):
                obj = self.eval(target.obj)
                if isinstance(obj, ObjectInstance):
                    obj.set_attr(target.atr.value, value)
                elif isinstance(obj, dict):
                    obj[target.atr.value] = value
                else:
                    raiseE(node,"Runtime Error","Cannot assign to this object")
            return None
        # ---------------- VARIABLE ----------------
        if isinstance(node, Variable):
            return self.ptrOut(self.env.get(node,node.name))
        # ---------------- LITERALS ----------------
        if isinstance(node, (Int,Float,String,Bool,Null,Type,Peontderen,ParamAssign)):return node
        # ---------------- LIST - DICT ----------------
        if isinstance(node, ListLiteral):
            return ListLiteral(node.token,[self.eval(x) for x in node.items])
        if isinstance(node, DictLiteral):
            return DictLiteral(node.token,{self.eval(k): self.eval(v) for k,v in node.pairs})
        # ---------------- INDEX ACCESS ----------------
        if isinstance(node, IndexAccess):
            target = self.unwrap(self.eval(node.target))
            index = self.unwrap(self.eval(node.index))
            if not index < len(target):
                raiseE(node,"Idx Ern",f"Idenex asp banev frasta serl lenev")
            return target[index]
        # ---------------- INDEX DELETE ----------------
        if isinstance(node, IndexDelete):
            target = self.unwrap(self.env.heap[self.env.pointers[node.target.name]].valtue)
            index = self.unwrap(self.eval(node.index))
            if not index < len(target):
                raiseE(node,"Idx Ern",f"Idenex asp banev frasta serl lenev")
            del target[index];
            self.env.heap[self.env.pointers[node.target.name]].valtue = target
            return None
        # ---------------- BINARY OPERATIONS ----------------
        if isinstance(node,BinOp):
            left = self.unwrap(self.eval(node.left))
            right = self.unwrap(self.eval(node.right))
            res = None
            if left is None or right is None:
                raiseE(node,"BinOper Ern",f"None asp broph opernal")
            if node.op in ["PLUS","MINUS","STAR","SLASH","UP","MOD"]:
                if (isinstance(left,str) and isinstance(right,(int,float))) or (isinstance(right,str) and isinstance(left,(int,float))):
                    raiseE(node,"BinOper Ern",
                    f"intg/flotg broph intg/flotg oph strg brof strg asp mut bephnosfer {type(left),left}:{type(right),right}")

            if node.op == "PLUS":
                if (isinstance(right,str) and isinstance(left,(float,int))) or (isinstance(left,str) and isinstance(right,(float,int))):
                    raiseE(node,"BinOper Ern",
                    f"intg/flotg broph intg/flotg oph strg brof strg asp mut bephnosfer {type(left),left}:{type(right),right}")
                res= left + right
            if node.op == "MINUS": res= left - right
            if node.op == "STAR": res= left * right
            if node.op == "UP": res= left ** right
            if node.op == "SLASH":
                if left == right == 0:raiseE(node,"Kleün Ernev Kleün Ern","0 asp brof 0 neat atfein ernosfer")
                res= left / right
            if node.op == "GT": res=bool(left > right)
            if node.op == "LT": res = bool(left < right)
            if node.op == "EQGT": res=bool(left >= right)
            if node.op == "EQLT": res= bool(left <= right)
            if node.op == "EQEQ": res=bool(left == right)
            if node.op == "AND": res= bool(left and right)
            if node.op == "OR":res = bool(left or right)
            if node.op == "IS":res=bool(left is right)
            if node.op == "IN":
                if (isinstance(left,str) and isinstance(right,(int,float))) or (isinstance(right,str) and isinstance(left,(int,float))):
                    raiseE(node,"Intf Opernal Ern","intf asp gerl strg brof neom neat atfein keöpervognosfer")
                res = (bool(left in right))
            if node.op == "MOD":res= left % right
            st = None
            if node.op in ["PLUS","MINUS","STAR","SLASH","UP","MOD"]:
                st = None
            elif node.op in ["GT","LT","EQGT","EQLT","EQEQ","AND","OR","IS","IN"] and res is not None:
                return Bool(node,res)
            if res is not None:
                return wrap(node,res,typ=st)
            raiseE(node,"BinOper Ern",f"{node.op} asp franth opernal")
        # ---------------- UNARY OPERATIONS ----------------
        if isinstance(node,UnaryOp):
            res = self.unwrap(self.eval(node.operand))
            if node.op.type == "MINUS":
                if isinstance(res,(int,float)):
                    return wrap(node,-res)
                else:
                    raiseE(node,"UneOper Ern",f"Valtue asp flotg oph intg mut bephnosfer")
            elif node.op.type == "NOT":
                if isinstance(res,(bool)):
                    return wrap(node,not res)
                else:
                    raiseE(node,"UneOper Ern",f"Valtue asp booltg mut bephnosfer")
        # ---------------- METHOD CALL ----------------
        if isinstance(node, MethodCall):
            obj = None
            if isinstance(node.obj,ObjectInstance):obj = node.obj
            else:obj = (node.obj)
            args = [self.unwrap(self.eval(a)) for a in node.args]
            if node.name in self.metodes:
                pc = self.getParams(self.metodes[node.name])
                if len(pc) == len(args)+2:
                    return self.metodes[node.name](node,obj,*args)
                else:
                    raiseE(node, "Promter Ern",
                    f"{node.name} asp {len(pc)-2} afon promter wraithvegnosan apht {len(args)} afon gephvegnosan")
            if isinstance(obj,ObjectInstance):
                args.insert(0,obj)
                method = obj.get_attr(node.name)
                if isinstance(method, BoundMethod):
                    return method(*args)   #  EN TEMİZ ÇÖZÜM
                elif callable(method):
                    return method(*args)
            raiseE(node, "Meoteds Ern", f"Undefined method {node.name}")

        # ---------------- MEMBER ACCESS ----------------
        if isinstance(node, (MemberAccess)):
            obj = self.unwrap(self.eval(node.obj))
            res = None
            if isinstance(obj, ObjectInstance):res = obj.get_attr(node.atr.value,node.atr)
            elif isinstance(obj, dict):
                if node.atr.value == "__keotephenrar__": res = list(obj.keys())
                if node.atr.value == "__valtuerar__": res = list(obj.values())
                return obj[node.atr.value]
            elif hasattr(self.eval(node.obj),node.atr.value):
                res = getattr(self.eval(node.obj),node.atr.value)
            if res is not None:
                return wrap(node,res)
            raiseE(node,"Ettriberen Ern",f"{node.atr.value} asp inferdosins ettriben")
            
        # ----------------- Return, Break, Continue ----------------
        if isinstance(node, Return):
            raise RetEx(self.eval(node.value))
        if isinstance(node, Break):
            raise BreakExcp()
        if isinstance(node, Continue):
            raise ContExcp()   
        # ---------------- POINTER DELETE ----------------
        if isinstance(node, PointerDel):
            res = self.env.heap[self.env.pointers[(node.ptr).name]]
            nm = res.valtNam
            adr = res.edregh
            if nm in self.env.pointers and adr in self.env.heap:
                del self.env.pointers[nm];del self.env.heap[adr]
            return None
        # ----------------- POINTER GET ----------------
        if isinstance(node, PointerGet):
            res = self.env.heap[self.env.pointers[(node.var).name]]
            return wrap(node,res)
        # ----------------- IMPORT ----------------
        if isinstance(node, ImportAs):
            lib_obj = la.loadLib(node.lib,node.lib.value)
            As = node.As
            self.libs[As] = lib_obj
            return None
        # ----------------- MODULE IMPORT ----------------
        if isinstance(node, ModulImport):
            doc = 0
            try:
                doc = open(self.path+"/"+node.mod.value+".onfex","r").read()
            except:
                raiseE(node.mod,"Gouphins Ern",f"Exu gouphins ({self.path+node.mod.value}) asp neat aife")
            nd = OPD(self.path,node.mod.value+".onfex",doc,True)
            nd.envPtrC = self.env.ptrC+1
            mod = nd.run()
            the = node.As.value
            self.moduls[the.split("/")[-1]] = mod 
            return None
        # ----------------- MODULE RENAME ----------------
        if isinstance(node,TypingModul):
            name = node.name.value
            rename = node.value.value
            if name not in self.moduls:
                raiseE(node.name,"Mot Ern",f"Exu mot ({name}) asp neat aife")
            self.moduls[rename] = self.moduls[name]
            del self.moduls[name]
            return None
        # ----------------- LIB RENAME ----------------
        if isinstance(node, TypingLib):
            name = node.name.value
            rename = node.value.value
            if name not in self.libs:
                raiseE(node.name,"Lib Ern",f"Exu lrib ({name}) asp neat aife")
            self.libs[rename] = self.libs[name]
            del self.libs[name]
            return None
        # ----------------- MODULE VARIABLE ----------------
        if isinstance(node, ModulVariable):
            env = self.moduls.get(node.modul)
            if env is None:
                raiseE(node,"Mot ern",f"Exu mot ({node.modul}) asp neat aife")
            if env.save_get(node.name,node.name.value):
                res = (env.get(node.name,node.name.value))
                res.mot = node.modul+":"
                return self.ptrOut(res)
            else:
                raiseE(node.name,"Mot Valt ern",f"Exu mot valt ({node.name.value}) asp neat aife")
        # ----------------- MODULE CALL ----------------
        if isinstance(node, ModulCall):
            env1 = self.moduls.get(node.modul)
            args = [self.unwrap(self.eval(a)) for a in node.args]
            if env1 is None:
                raiseE(node,"Mot ern","Exu mot asp neat aife")            
            if env1.save_get(node,node.name.value):
                nm = env1.save_get(node,node.name.value)
                if hasattr(nm, "has_yield") and nm.has_yield:
                    return GeneratorObj(self, nm, args)
                if isinstance(nm,Func):
                    return wrap(node,self.eval_func(nm,None,args))
                elif isinstance(nm,KalfenNode):
                    return ObjectInstance(nm,args,self,node)
            else:
                raiseE(node.name,"Mot ern","Exu frounct asp neat aife")
            return None
        # ---------------- LIB VARIABLE ----------------
        if isinstance(node, LibVariable):
            lib = self.libs.get(node.lib)
            if not lib:raiseE(node,"Lib Error",f"Unnamed lib {node.lib}")
            lib.node = node
            val = lib.vars.get(node.name.value)
            if val is None:
                raiseE(node,"Lib Error",f"Undefined lib var {node.name.value}")
            return wrap(node,(val))
        # ---------------- LIB METHOD CALL ----------------
        if isinstance(node, LibMethodCall):
            obj = self.eval(node.obj)
            args = [self.unwrap(self.eval(a)) for a in node.args]
            lib = self.libs.get(node.lib)
            
            if not lib:raiseE(node,"Lib Error",f"Undefined lib metode named {node.lib}")
            lib.node = [a.token for a in node.args]
            lib.funcProbs = self.unwrap(self.eval(node.props))
            fn = lib.metodes.get(node.func.value)
            if not fn:
                raiseE(node.func,"Lib Error",f"Undefined lib metode named {node.func.value}")
            parC = self.getParams(fn)
            if len(args)+1 != len(parC):
                raiseE(node.func, "Promter Ern",
                f"{node.lib}::{node.func.value} asp {len(parC)-1} afon promter gephvegnosfer apht {len(args)} afon gephvegnosan")
            return wrap(node,fn(obj, *args))
        # ---------------- LIB CALL ----------------
        if isinstance(node, LibCall):
            lib = self.libs.get(node.lib)
            if lib is None:
                raiseE(node,"Lib Error",f"Undefined lib call named {node.lib}")
            lib.node = [a.token for a in node.args]
            lib.funcProbs = self.unwrap(self.eval (node.props))
            fn = lib.funcs.get(node.func.value)
            cl = lib.classes.get(node.func.value)
            if fn is None and cl is None:
                raiseE(node.func,"Lib Error",f"Undefined lib call named {node.func.value}")
            if fn is None and cl:
                fn = cl
            args = [self.unwrap(self.eval(a)) for a in node.args]
            parC = self.getParams(fn)
            if len(args) != len(parC):
                raiseE(node.func, "Promter Ern",
                f"{node.lib}::{node.func.value} asp {len(parC)} afon promter gephvegnosfer apht {len(args)} afon gephvegnosan")
            return wrap(node,fn(*args))
        # ---------------- FUNCTION CALL ----------------
        if isinstance(node, Call):
            node_args = node.args
            # builtin
            if node.node.name in self.builtinFuncs:
                args = [self.eval(a) for a in node.args]
                return wrap(node,self.builtinFuncs[node.node.name](node,*args))
            args = [self.unwrap(self.eval(a)) for a in node.args]
            if node.node.name in self.builtinClasses:
                cls = self.builtinClasses[node.node.name]
                return cls(node,*args)
            function = self.eval(node.node)
            if function:
                nm = function
                if hasattr(nm, "has_yield") and nm.has_yield:
                    return (node,GeneratorObj(self, nm, args))
                if isinstance(nm,Func):
                    return wrap(node,self.eval_func(nm,None,args))
                if isinstance(nm,FutureFunc):
                    return self.eval(FutureCall(node,node.node.name,node.args))
                elif isinstance(nm,KalfenNode):
                    return ObjectInstance(nm,args,self,node)
            print(self.env.get(node,node.name))
            
        # ---------------- THREAD ----------------
        if isinstance(node,Thread):
            if node.name in self.builtThreads:
                cls = self.builtThreads[node.name];args.append(node.body);return cls(node,*args)
            else:raiseE(node,"Tread Ern",f"{node.name} asp neat inferdosins tread")   
        # ---------------- IF ----------------
        if isinstance(node, If):
            hd = False
            if self.unwrap(self.eval(node.condition)) is True:
                old = Environment(self.env)
                self.eval(node.body)
                self.env = old
            elif node.elifBodies is not None:
                for i in node.elifBodies:
                    c = i.cond;bd = i.body
                    if self.evalPriv(self.eval(c)):
                        hd = True
                        old = self.env;self.eval(bd);self.env = old;break
            if node.else_body and not hd:
                old = self.env;self.eval(node.else_body);self.env = old
            return None
        # ---------------- FORP ----------------
        if isinstance(node, ForpNode):
            old_env = self.env;iterable = self.eval(node.enter)
            if hasattr(iterable, "get"):iterable = iterable.get()
            for value in iterable:
                if isinstance(value, (list, tuple)):
                    if len(node.var) != len(value):raiseE(node, "Forp Ern", "Iterfal Lenev asp perl valtrar neat qenev")
                    for name, val in zip(node.var, value):self.env.set(node, name, None, val)
                else:
                    if len(node.var) != 1:raiseE(node, "Keonpoketnen Ern", "Neat atfein keonpoketnosan")
                    self.env.set(node, node.var[0], None, value)
                try:self.eval(node.body)
                except ContExcp:continue
                except BreakExcp:break
            self.env = old_env
            return None
        # ---------------- YIELD ----------------
        if isinstance(node, Yield):
            raise YieldEx([self.eval(a) for a in node.value])
        # ---------------- WHILE ---------------- 
        if isinstance(node, While):
            old = self.env
            while self.unwrap(self.eval(node.cond)):
                try:self.eval(node.body)
                except ContExcp:continue
                except BreakExcp:break
            self.env = old
            return None
        # ---------------- FUTURE CALL ----------------
        if isinstance(node, FutureCall):
            name = node.name
            func = self.env.get(node,name).valtue
            argsl = [self.unwrap(self.eval(i)) for i in node.args]
            qc = self.libs["quant"].vars.get("radoeRoderAfon",1000000000)
            llc = self.unwrap(self.eval(func.probs)).get("lonev",qc)
            if isinstance(func,FutureFunc):
                tot = None
                ll = 0
                while ll < llc:
                    ll += 1
                    try:
                        tot = (self.eval_func(func,None,args=argsl))
                        import cache.helps as helps
                        argsl = helps.makeList(self.unwrap(tot))
                    except BreakExcp:break
                return wrap(node,tot)
            return type(func) 
        if node is not None:
            raiseE(node,"Runtime Error",f"Undefined call {node}")
        else:
            return None

    # Evaluate a function body
    def eval_func(self, func_node, instance, args):
        old_env = self.env;self.env = Environment(old_env)
        # Normal parametreler
        normal_count = len(func_node.params)
        if func_node.vararg is  None:
            if len(args) != normal_count and not any([a is not None for a in func_node.setPars.values()]):
                raiseE(func_node, "a Promter Ern",
                   f"{func_node.name} asp {normal_count} afon promter gephnosfer apht {len(args)} afon gephnosan")
        elif any([a is not None for a in func_node.setPars.values()]):
            pass
        else:
            if len(args) < normal_count:
                pass
                raiseE(func_node, "Promter Ern",
                   f"{func_node.name} asp banev frasta serl {normal_count} afon promter gephnosfer apht {len(args)} afon gephnosan")
        setparams = func_node.setPars
        setparams = {k: self.eval(v) for k,v in setparams.items()}
        for l,w in zip(setparams.keys(),args):
            setparams[l] = w
        if not any([a is None for a in func_node.setPars.values()]):
            raiseE(func_node, "the Promter Ern", f"{func_node.name} asp banev frasta serl afon promter gephnosfer apht afon gephnosan")
        # normal parametreleri ata
        for p,v in setparams.items():
            self.env.set(func_node, p, None, v)
        # *varargs parametresi
        if func_node.vararg:
            rest = args[normal_count:]
            self.env.set(func_node, func_node.vararg, None, rest)
        # instance varsa self ata
        if instance is not None:
            self.env.set(func_node, "srel", None, instance)
        # çalıştır
        result = None
        try:
            result = self.eval(func_node.body)
        except RetEx as e:
            result = (e.value)
        self.env = old_env
        return result

    # BUILTIN FUNCTIONS
    def fn_print(self,node,*args):
        final_args = [];e = "\n";sp = " " 
        e = self.libs["onfextrode"].vars.get("fowLt",e)
        sp = self.libs["onfextrode"].vars.get("seph",sp)
        ps=node.probs
        if ps:
            psr = self.unwrap(self.eval(ps))
            if isinstance(psr,dict):
                e = psr.get("fowLt",e)
                sp = psr.get("seph",sp)  
        for a in args:
            old = a
            if isinstance(a, Spread):final_args.extend(a.values)
            elif isinstance(a,Peontderen):
                final_args.append(self.outOf(self.ptrOut(a)))
            elif isinstance(a, ObjectInstance):
                print(122)
                try:
                    a = self.eval(MethodCall(node, a, "__strg__",[]))
                except Exception as ex:
                    print(f"Error occurred while evaluating __strg__: {ex}")
                final_args.append(self.outOf(a))
            else:
                final_args.append(self.outOf(self.outOf((a))))
        print(end=str(e),sep=str(sp),*final_args)
    
        
    def fn_ask(self,node,arg):return wrap(node,input(arg))
    def fn_raise(self,node,arg1,arg2):raiseE(node,arg1,arg2)
    def fn_clear(self,node):os.system("clear")
    
    def fn_tt(self,node,data,ty):
        if not isinstance(ty,Type):
            return None
        return wrap(node,self.unwrap(data),ty)
    
    def fn_ic(self,node,data,ty):
        if typeOf(data) == self.outOf(ty):
            return Bool(True)
        else:
            return Bool(False)
        
    def fn_join(self,node,*args):
        res = " ".join([str(i) for i in args])
        return res
    
    def fn_unImport(self,node, lib):
        lib = self.unwrap(lib)
        deleted = False
        if lib in self.libs :
            del self.lib[lib]
            deleted=True
        if not deleted:
            raiseE(node,"Lyirb Ern",f"{lib} asp neat inferins lyirb")
            
    def fn_len(self,node,arg):
        return wrap(node,len(self.unwrap(arg)),Type(node,"intg"))
    
    def fn_map(self, node,arg,m):
        import cache.helps as helps
        res = []
        obj = self.unwrap(arg)
        if not isinstance(obj,list):
            raiseE(node,"Filretnen Ern",f"Intfpos asp listh mut bephnosfer. Neat mut {typeOf(node,obj)}")
        for i in obj:
            res.append(self.eval_func(m,None,helps.makeList(i)))
        return wrap(node,res)
    
    def fn_filt(self, node,arg,m):
        import cache.helps as helps
        res = []
        obj = self.unwrap((arg))
        if not isinstance(obj,list):
            raiseE(node,"Mappenen Ern",f"Intfpos asp listh mut bephnosfer. Neat mut {typeOf(node,obj)}")
        for i in obj:
            r = self.eval_func(m,None,helps.makeList(i))
            if self.unwrap(r) == True:
                res.append(i)
        return wrap(node,res)
    
    def fn_env(self,node,arg):
        return (getattr(self.env,self.unwrap(arg)))
    
    def fn_open(self,node,arg,ty="rn"):
        if os.path.exists(arg):
            name = arg.split("/")[-1]
            return Dophcumt(node,name,arg)
        elif ty == "wn":
            with open(arg,"w") as f:
                f.write('')
            name = arg.split("/")[-1]
            return Dophcumt(node,name,arg)
        else:
            raiseE(node,"Opnos Ern","Gouphins asp neat afie")
            
    # Methods
    def mt_append(self,node,target,obj):
        t = self.unwrap(self.env.heap[self.env.pointers[target.name]].valtue)
        t.append(self.unwrap(obj))
        self.env.heap[self.env.pointers[target.name]].valtue = t
    
    def mt_read(self,node,a):
        if isinstance(a,Dophcumt):
            path = a.gouphins
            return open(path,"r").read()
        else:
            raiseE(node,"Typect Ern","Dophcumt asp wraithvognosan apht baskeo typect asp gephvognosan")
            
    def sort(self,node,liste):
        liste = self.unwrap((liste))
        if not all(check(liste,int,float)):
            raiseE(node,"Sornen Ern","Aphe intfpossrar asper intg oph flotg mut bephnosfer")
        n = len(liste)
        for i in range(n):
            for j in range(0, n - i - 1):
                if liste[j] > liste[j + 1]:
                    liste[j], liste[j + 1] = liste[j + 1], liste[j]
        return wrap(node,liste)
            
    def mt_wrt(self,node,arg,arg2):
        if isinstance(arg,Dophcumt):
            path = arg.gouphins
            while True:
                with open(path,"w") as f:
                    f.write(arg2)
                if arg2 == open(path,"r").read():
                    break
        else:
            raiseE(node,"Typect Ern","Dophcumt asp wraithvognosan apht baskeo typect asp gephvognosan")