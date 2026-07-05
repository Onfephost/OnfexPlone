from cache.environment import Environment
from cache.ast_nodes import Int,Assign,ASTNode,Mehen
import os

import cache.byte_codes as bc
os.system("clear")

mehenc = 0
asts = [
    Int(None,6),
    Mehen(None,[Int(None,5),Assign(None,"ali",None,Int(None,4))])
]

bcs = []
block = None
def t(i,block):
    global mehenc
    if isinstance(i,Int):
        return (bc.IntValue(i.token,block,i.value))
    elif isinstance(i,Mehen):
        res = (bc.MainCode(i.token,block,ts(i.statements,f"Main{mehenc}")))
        mehenc += 1
        return res
    elif isinstance(i,Assign):
        return bc.Push(i.token,block,i.var,t(i.value,block))
    else:
        return i
def ts(asts,block=None):
    global mehenc
    bcs = []
    for i in asts:
        bcs.append(t(i,block))
        
    return bcs

class VM:
    def __init__(self,prog:bc.Program):
        self.p = prog
        self.env = Environment()
        self.heap = []
    
    def start(self):
        return self.run(self.p.bcs)
        
    def run(self,b):
        if isinstance(b,bc.BlockCode):
            for s in b.stmts:
                self.run(s)
            return None
        if isinstance(b,bc.MainCode):
            old = self.env
            for s in b.stmts:
                self.run(s)
            self.env = old
            return None
                
        if isinstance(b,bc.Push):
            self.heap.append(self.expr(b.value))
            self.env.set(b,b.var,b.typ,self.heap[-1])
            print("pushed")
            return None
            
    def expr(self,e):
        if isinstance(e,bc.IntValue):
            return e
bcs = ts(asts)
print(bcs)
codes = bc.BlockCode(None,None,bcs)
prg = bc.Program(codes)
vm = VM(prg)
vm.start()
print(vm.p.bcs)
print(vm.env.__dict__)
    