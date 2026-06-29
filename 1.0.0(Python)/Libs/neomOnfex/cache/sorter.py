from cache.Exceptions import *

def check(l,*args):
    rs = []
    for i in l:
        if isinstance(i,args):
            rs.append(True)
        else:
            rs.append(False)
    return rs
                    
def sort(node,liste):
    if not all(check(liste,int,float)):
        raiseE(node,"Sornen Ern","Aphe intfpossrar asper intg oph flotg mut bephnosfer")
    n = len(liste)
    for i in range(n):
        for j in range(0, n - i - 1):
            if liste[j] > liste[j + 1]:
                liste[j], liste[j + 1] = liste[j + 1], liste[j]
    return liste


            
                    