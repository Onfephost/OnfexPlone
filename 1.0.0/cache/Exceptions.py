# Exceptions.py
class OnfexError(Exception):
    def __init__(self,pos,line,col,err_type,message):
        self.pos = pos
        self.line = line
        self.col = col
        self.err_type = err_type
        self.message = message

class YieldEx(Exception):
    def __init__(self, value):
        self.value = value
        
class BreakExcp(Exception):
    def __init__(self):
        pass
        
class ContExcp(Exception):
    def __init__(self):
        pass
        
class RetEx(Exception):
    def __init__(self,value):
        self.value = value
        
def show_error(code: str, error: OnfexError,doc = "Main.onfex"):

    start = error.pos[0]
    end = error.pos[1]
    line = error.line
    col = error.col   # bunu ekliyoruz

    lines = code.split("\n")

    if line-1 < len(lines):
        line_text = lines[line-1]
    else:
        line_text = ""

    print("Errors:")
    print(f"/{doc} Line: {line}")
    print()

    print(f"{line} | {line_text}")

    pointer = " " * (len(str(line)) + 3 + col) + "^" * max(1, end-start)

    print(pointer)
    print()
    print(f"{error.err_type}: {error.message}")
    
def raiseE(tok,a,b,c=None,skip=0):
    res = tok.pos
    l = []
    if c:
        l.append(tok.pos[0]+c[0])
        l.append(tok.pos[1]+c[1])
        res = l
    raise OnfexError(res,tok.line,tok.col+skip,a,b)

def raiseOE(e):
    raise OnfexError(e.pos,e.line,e.col,e.err_type,e.message)