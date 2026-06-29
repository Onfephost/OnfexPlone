import os
import time
from typing import Dict, List, Optional
packs = {}
Vars = {"TV":"1.0.0","CT":"Undef"}
docs = ["Test.txt"]
current_dir = os.path.dirname(os.path.abspath(__file__))

def rex(t,l,to):
    for i in range(len(l)):
        t = t.replace(l[i],to[i])
    return t
def loadPacks(d):
    packs = d
def appendPacks(k,v):
    packs[k] = v
def clearPacks():
    packs = {}
def getInPacks(k):
    return packs[k]

def appendList(text):
    file_path = os.path.join(current_dir, "TermDocList.txt")
    data2 = None
    with open(file_path, "r", encoding="utf-8") as f:
        data2 = f.read()
    docsS = data2.split("/")
    docsS.append(text.strip())
    with open(file_path, "w", encoding="utf-8") as f:
        f.write("/".join(docsS))
def getList():
    file_path = os.path.join(current_dir, "TermDocList.txt")
    with open(file_path, "r", encoding="utf-8") as f:
        return f.read().split("/")
def clearList():
    file_path = os.path.join(current_dir, "TermDocList.txt")
    data2 = None
    with open(file_path, "r", encoding="utf-8") as f:
        data2 = f.read()
    data2 = data2.split("/")
    if len(data2)>0:
        for i in data2:
            if os.path.isfile(current_dir+"/"+i):
                os.remove(current_dir+"/"+i)
            
    with open(file_path, "w", encoding="utf-8") as f:
        f.write("")   
        
def delInList(p):
    file_path = os.path.join(current_dir, "TermDocList.txt")
    data2 = None
    with open(file_path, "r", encoding="utf-8") as f:
        data2 = f.read()
    data2 = data2.split("/")
    if p.strip() in data2:
        os.remove(current_dir+"/"+p)
        data2.remove(p.strip())
    with open(file_path, "w", encoding="utf-8") as f:
        f.write("/".join(data2))
class Term:
    def __init__(self,title:Optional[str]=None,On:bool=False,Ask="$ ",extra:Optional[dict]={}):
        self.title = title
        self.ActivedPacks = {}
        self.TermOn = On
        self.ask = Ask
        self.ac = extra.get("allowingCodes")
    def titleSet(self,titleX):
        print(titleX)
    def startTerm(self):
        self.titleSet(self.title)
        while self.TermOn:
            
            enter = input(self.ask)
            enter = enter.split(" ")
            
            if enter[0] == "exit":
                break
            elif enter[0] == "":
                continue
            else:
                self.call(enter)
    def call(self,e):
        command,value,As = ("1","UnEntered","Undef") 
        try:command = e[0] 
        except: pass
        try:value = e[1] 
        except: pass
        try:command = e[0] 
        except: pass
        try:As= e[2] 
        except: pass
        
        if self.TermOn:
            termList=[
            "",
            "clear/clephnos/id1               :For clear the consol.",
            "cesnos/nano/n/id3                :For editing document.",
            "print/pyrintnos/p                :For printing anything.",
            "input/morfenlnos                 :For asking anything.",
            "wertwnos_listh/show_list/pwd     :For seeing docs.",
            "python/py                        :For run py docs.",
            "delete_doc/detnos_groch/del/id5  :For deleting documents.",
            "clephnos_listh/clear_list/cli... :For clearing document list.",
            "opnos/open/o/id11                :For opening file.",
            ""
            ]
            docy = None
            
            command = command.replace("code","id")
            if value:
                docy = os.path.join(current_dir,value)
            if command[0] == "#":
                pass
            elif command in ["clephnos","clear","c","id1"]:
                os.system("clear")
                print(self.title)
            elif command == ["dowpownos","import","iympord","pip_install","pi","id2"]:
                pass 
            elif command in ["cesnos","nano","n","id3"]:
                if docy:
                    if not os.path.exists(docy):
                        with open(docy,"w") as f:
                            f.write("")
                    if not value in getList():
                        appendList(value)
                try:
                    os.system("nano "+docy)
                except:
                    if docy == None:
                        print("Enter Error: Give a path.")
            elif command in ["python","py","id4"]:
                if value == None:
                	return 'Error'
                if not os.path.exists(docy) or not value in getList():
                    print("The Path is not in the folder(Term)")
                else: 
                    os.system("python "+docy)
            elif command in ["delete_doc","detnos_groch","del","id5"]:
                delInList(value)
            elif command in ["ver","-v","version","id11"]:
                print("Terminal Version:",Vars["TV"])
            elif command in ["pyrintnos","print","p","id6"]:
                print("Out: ",value)
            elif command in ["input","morfenlnos","i","id7"]:
                res = None
                if value:
                    res = input(value+" ")
                else:
                    res = input()
                    
                if As:
                    Vars[As] = res
            elif command in ["help","heolp","?","h?","id8"]:
                print("\n   ".join(termList))
            elif command in ["wertwnos_listh","show_list","pwd","sl","id9"]:
                print("\n".join(getList()))
            elif command in ["clear_list","clephnos_listh","cli","cl","id","id10"]:
                clearList()
            elif command in ["open","opnos","o","id11"]:
                if value == "Term":
                    os.system("clear")
                    self.startTerm()
                    
            else:
                print("Command Error:Undefined Command")
if __name__ == "__main__":
    extra = {"allowingCodes":{"id1":True}}
    a = Term(title="Type \"help\" for showing commands.\n",On=True)
    a.startTerm()