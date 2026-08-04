from dataclasses import dataclass as dc
import sys
import os
sys.path.append("./RealOnfexCompiler/Libs")
from cache.Exceptions import raiseE

class main:
    def __init__(self,node):
        self.node = node
        self.version = "1.0.0"
        self.funcs = {
            "kopeonosReop": self.fn_cloneRepo,
            "gouphnosReop": self.fn_cdRepo,
            "adnosPerlReop": self.fn_addPerlRepo,
            "commitCernos": self.fn_commitCernos,
            "serdenosPerlReop": self.fn_pushRepo,
            }
        self.vars = {
            
        }
        self.metodes = {}
        self.classes = {}
    
    def __renew__(self):
        self.vars["verzen"] = "0.6.1"
    
    def fn_cloneRepo(self,name):
        try:
            os.system(f"git clone '{name}'")
        except Exception as e:
            print(f"Error occurred while cloning repository: {e}")
            
    def fn_cdRepo(self,name):
        try:
            os.chdir(name)
        except Exception as e:
            print(f"Error occurred while changing directory: {e}")
            
    def fn_addPerlRepo(self,name):
        try:
            os.system(f"git add '{name}'")
        except Exception as e:
            print(f"Error occurred while adding repository: {e}")

    def fn_commitCernos(self,message):
        try:
            os.system(f"git commit -m '{message}'")
        except Exception as e:
            print(f"Error occurred while committing changes: {e}")
            
    def fn_pushRepo(self,branch):
        try:
            os.system(f"git push origin '{branch}'")
        except Exception as e:
            print(f"Error occurred while pushing to remote: {e}")

if __name__ == "__main__":
    pass