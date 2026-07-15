
import os
import string

def getCharPack(ask):
    char = None
    if ask in ("abc","default","english"):
        char = "abcdefghijklmnoprstuvyzwxq" + "ABCDEFGHIJKLMNOPRSTUVYZWXQ"
        
    elif ask in ("abcß","deutsch","german"):
        char = "abcdefghijklmnoprstuüvyzwxqßä" + "ABCDEFGHIJKLMNOPRSTUVYZWXQßÄ"
        
    elif ask == "utf-8":
        char = open("./RealOnfexCompiler/Libs/karchenter/utf8.txt","r").read()
        
    elif ask == "abcç" or ask == "turkish":
        char = "abcçdefgğhıijklmnoöprsştuüvyzwxq" + "ABCÇDEFGĞHIİJKLMNOÖPRSŞTUÜVYZWXQ"
        
    elif ask == "numbers":
        char = "0123456789"
        
    elif ask == "sembols":
        char = string.punctuation   
    return char
def getCharPackType():
    return "utf-8"
    
def get(ask):
    return getCharPack(ask)
    
if __name__ == "__main__":
    with open("./RealOnfexCompiler/Libs/karchenter/utf8.txt","w") as f:
        f.write(";:;".join([chr(i) for i in range(0x110000)][0:50000]))

