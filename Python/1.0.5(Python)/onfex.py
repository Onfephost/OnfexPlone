
import os
from cache.document import OnfexPloneDoph as OPD

main_onfex = OPD(os.path.dirname(os.path.abspath(__file__)),"main.onfex",None,False)
add_onfex = OPD(os.path.dirname(os.path.abspath(__file__)),"add.onfex",None,True)

main_onfex.run()