import characterpack as chp

class main:
    def __init__(self,node):
        self.node = node
        self.funcs = {
        "gephnos":self.fn_get,
        }
        self.vars = {
        "verzen":[1],
        "utf_8":chp.getCharPack("utf-8"),
        "turkishchars":chp.getCharPack("turkish"),
        "germanchars":chp.getCharPack("german"),
        "englishchars":chp.getCharPack("english"),
        }
        self.metodes = {}
        self.classes = {}
    def fn_get(self,ask):
        return chp.get(ask)
    
            
    
if __name__ == "__main__": 
    pass


