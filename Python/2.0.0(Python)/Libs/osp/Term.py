import os
from bashCommands import run_command
class Term:
    def __init__(self, isProtected):
        self.isProtected = isProtected
    
    def call(self, command):
        if self.isProtected:
            print(f"Executing command in protected mode: {command}")
            run_command(command)
        else:
            print(f"Executing command: {command}")
            os.system(command)
            

     