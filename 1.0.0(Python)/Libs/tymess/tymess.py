import time

def helpe():
    print("""
    moorwaeDvea = monthday
    oertDvea = yearday
    öreo = hour
    dophkeo = minute
    solfess = second
    oert = year
    moorwae = month
    heofteDvea = week day
    """)

class main:
    def __init__(self,node):
        self.node = node
        self.funcs = {
        "wraithnos":self.fn_wait,
        "deat":self.fn_date,
        }
        self.metodes = {}
        self.vars = {}
        self.classes = {}
    def fn_wait(self,arg):
        time.sleep(arg)
        
    def fn_date(self,arg):
        dt = time.gmtime(time.time())
        if __name__ == "__main__":
            print(dt)
        match arg:
            case "moorwaeDvea":
                return dt.tm_mday
            case "help","heolp":
                helpe()
                return None
            case "oertDvea":
                return dt.tm_yday
            case "öreo":
                return dt.tm_hour
            case "dophkeo":
                return dt.tm_min
            case "solfess":
                return dt.tm_sec
            case "oert":
                return dt.tm_year
            case "moorwae":
                return dt.tm_mon
            case "heofteDvea":
                return dt.tm_wday
            case _:
                raise Exception("Unexcepted time")

if __name__ == "__main__":                
    new = tymess()
    res=new.fn_date("moorwae")
    print(res)
