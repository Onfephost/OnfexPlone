from cache.Exceptions import *
from cache.lexer import lex
from cache.parser import Parser
import cache.interpreter as intp
import os
#OPD
class OnfexPloneDoph:
    def __init__(self,docpath,name,code,isImport:bool):
        self.name = name
        self.path = docpath
        self.main = None
        if code is not None:
            self.code = code
        else:
            self.code = str(open(docpath+"/"+name,"r").read())
        self.ii = isImport
        self.envPtrC = 1
        
    def run(self,nm=None):
        if self.ii == False:
            try:        
                tokens = lex(self.code)
                tree = Parser(tokens).parse()
                print("Onfex asp dowpownosyer...")
                intr = intp.Interpreter()
                #os.system("clear")
                intr.path = self.path
                intr.mainOn = True
                print("[Onfex Run]")
                intr.eval(tree)
            except OnfexError as e:show_error(self.code,e)
        else:
            try:
                self.main = nm                
                tokens = lex(self.code)
                tree = Parser(tokens).parse()
                intr = intp.Interpreter()
                intr.env.ptrC = self.envPtrC
                intr.path = self.path 
                intr.mainDoc = self.main
                intr.eval(tree)
                return intr.env
            except OnfexError as e:show_error(self.code,e)
        
