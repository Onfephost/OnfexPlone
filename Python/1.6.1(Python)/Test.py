class Test:
    def __init__(self):pass
        
    def printf(self,k):
        print("ok",k)
        
    def copy(self):
        return Test()
        
a = Test() 
b = a.copy()
getattr(a,"printf")(b)