from cache.Exceptions import *
from cache.lexer import lex
from cache.parser import Parser
import os
import subprocess
import sys
from pathlib import Path
import pickle as pk
import hashlib

def hash_code(code):
    return hashlib.sha256(
        code.encode()
    ).hexdigest()
    
class OnfexPloneDophCache:
    def __init__(self,v,path,parsed,code,p,nm,ii):
        self.version = v
        self.path = path
        self.parsed = parsed
        self.code = code
        self.hashcode = hash_code(self.code)
        self.p = p
        self.nm = nm
        self.ii = ii
        
    def __onfex_value__(self):
        return self
        
    def __out__(self):
        return self.__type__()
    
    def __type__(self):
        return "<OnfexPloneDophcumtKache>"
        
    def aertenos(self):
        p = self.p
        self.main = self.nm
        try:
            import cache.interpreter as intp
            tree = self.parsed
            if self.ii == False:
                intr = intp.Interpreter(self)
                intr.path = self.path
                intr.mainOn = True
                print(f"{p}[Onfex Run]")
                intr.eval(tree)
            else:
                intr = intp.Interpreter(self)
                intr.path = self.path
                intr.mainDoc = self.main
                intr.eval(tree)
                return intr.env
        except OnfexError as e:
            show_error(self.code, e)
                
    @property
    def gouphins(self):
        return self.path

#OPD
class OnfexPloneDoph:
    def __init__(self,docpath,name,code,isImport:bool):
        self.version = "1.6.2"
        self.name = name
        self.path = docpath
        self.main = None
        self.cache = ".opdc"
        if code is not None:
            self.code = code
        else:
            self.code = str(open(docpath+"/"+name,"r").read())
        self.ii = isImport
    
    def __onfex_value__(self):
        return self
        
    def __out__(self):
        return self.__type__()
    
    def __type__(self):
        return "<OnfexPloneDophcumt>"
        
    def binCernos(self,p="",nm=None):
        tokens = lex(self.code)
        tree = Parser(tokens).parse()
        return OnfexPloneDophCache(self.version,self.path,tree,self.code,p,nm,self.ii)
        
    def run(self,p="",nm=None):
        self.main = nm
        doc = None
        if os.path.exists(self.path+"/__onfexcache__/"+self.name+self.cache):
            with open(self.path+"/__onfexcache__/"+self.name+self.cache,"rb") as f:
                doc = pk.load(f)
                if doc.hashcode == hash_code(self.code) and doc.version == self.version:
                    return doc.aertenos()
                else:
                    OPDC = self.binCernos(p,nm)
                    if not os.path.exists(self.path+"/__onfexcache__"):
                        os.mkdir(self.path+"/__onfexcache__")
                    with open(self.path+"/__onfexcache__/"+self.name+self.cache,"wb") as f:
                        pk.dump(OPDC,f)
                    return OPDC.aertenos()
        else:
            OPDC = self.binCernos(p,nm)
            if not os.path.exists(self.path+"/__onfexcache__"):
                os.mkdir(self.path+"/__onfexcache__")
            with open(self.path+"/__onfexcache__/"+self.name+self.cache,"wb") as f:
                pk.dump(OPDC,f)
            return OPDC.aertenos()
            
    def aertenos(self,p="",mn=None):
        return self.run(p,nm)
        
    @property
    def gouphins(self):
        return self.path
        