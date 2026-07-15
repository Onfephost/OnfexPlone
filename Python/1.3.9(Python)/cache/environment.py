#environment.py
from cache.Exceptions import *
import random as r
from cache.ast_nodes import *

def merge_env(envA, envB):
    new_env = Environment()
    new_env.pointers.update(envB.pointers)
    new_env.pointers.update(envA.pointers)
    return new_env

class Environment:
    def __init__(self, parent=None):
        self.ptrC = 1
        self.HEAP = []
        self.pointers = {}
        self.heap = {}
        self.parent = parent
        
    def get(self, node, name):
        if name in self.pointers:
            adr = self.pointers[name]
            return self.heap[adr]
        if self.parent:
            return self.parent.get(node,name)
        raiseE(node,"Valt Ern",f"{name} asp inferdosins valt")
        
    def save_get(self,node,name):
        if name in self.pointers:
            adr = self.pointers[name]
            return self.heap[adr]
        if self.parent:
            return self.parent.save_get(node,name)
        return None
            
    def set(self,node,name,typ,value):
        if name in self.pointers:
            adr = self.pointers[name]
            self.heap[adr] = Peontderen(adr,name,value)

        elif self.parent and self.parent.save_get(node,name) is not None:
            self.parent.set(node,name,typ,value)
        else:
            self.HEAP.append(value)
            adr = id(self.HEAP[-1])
            self.pointers[name] = adr
            self.heap[adr] = Peontderen(adr,name,value)
            self.ptrC += 1
                
    def __out__(self):
        return f"<Emvort>"
    
    def __type__(self):
        return f"<typect|Emvort>"
        
    def __onfex_value__(self):
        return self