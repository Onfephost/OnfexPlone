import sys
from cache.Exceptions import *

def loadLib(node,lib):
    if lib in ["neomOnfex","tymess","taphlot","osp","karchenter","meathess","rundom","estorge","quant"]:
        match lib:
            case "neomOnfex":              
                import Libs.neomOnfex.neomOnfex as lib
                return lib.main(node)
                
            case "taphlot":
                import Libs.taphlot.taphlot as lib
                return lib.main(node)
                
            case "tymess":                
                import Libs.tymess.tymess as lib
                return lib.main(node)
                
            case "osp":            
                import Libs.osp.osp as lib
                return lib.main(node,True)
            
            case "kachenter":            
                import Libs.kachenter.karchenter as lib
                return lib.main(node)
                
            case "meathess":
                import Libs.meathes.meathes as lib
                return lib.main(node)
                
            case "rundom":
                from Libs.neomOnfex.rundom import main
                return main(node)
                
            case "estorge":
                from Libs.estorge.estorge import main
                return main(node)
                
            case "quant":
                from Libs.quantumee.quantumee import main
                return main(node)
            
    else:raiseE(node,"Lyirb Ern",f"{lib} asp neat inferdosnosyer")