

def makeList(x):
    if isinstance(x,(list,tuple,set)):
        return list(x)
    elif isinstance(x,(int,float,str,bool)):
        return [x]