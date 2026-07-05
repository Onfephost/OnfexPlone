
class Program:
    def __init__(self,bytecodes):
        self.bcs = bytecodes

class ByteCode:
    def __init__(self,token=None,block=None):
        self.token = token
        self.block = block
        self.typct = self.__class__.__name__
        if token:
            self.pos = token.pos
            self.line = token.line
            self.col = token.col
        else:
            self.pos = [0,0]
            self.line = 0
            self.col = 0

class Push(ByteCode):
    def __init__(self, token, block,varname,value):
        super().__init__(token, block)
        self.var = varname
        self.typ = None
        self.value = value
        print(f"PUSH: {self.var} = {self.value}")
    
    def __str__(self):
        return "PUSH"
class IntValue(ByteCode):
    def __init__(self, token, block,value):
        super().__init__(token, block)
        self.value = value
    
    def __str__(self):
        return "int "+str(self.value)

class MainCode(ByteCode):
    def __init__(self, token, block,value):
        super().__init__(token, block)
        self.stmts = value
    
    def __str__(self):
        return f"Block:[{",".join([str(i) for i in self.stmts])}]"


class BlockCode(ByteCode):
    def __init__(self, token, block,value):
        super().__init__(token, block)
        self.stmts = value
    
    def __str__(self):
        return f"Block:[{",".join([str(i) for i in self.stmts])}]"