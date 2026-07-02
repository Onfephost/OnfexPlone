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
        self.vars = {"verzen":"1.0.0"}
        self.classes = {}

    def __renew__(self):
        self.vars["verzen"] = "1.0.0"
        
    def fn_wait(self,arg):
        time.sleep(arg)
        
    def fn_date(self,arg):
        dt = time.gmtime(time.time())
        if __name__ == "__main__":
            print(dt)
        match arg:
            case "moorwaeDvea","monthday":
                return dt.tm_mday
            case "help","heolp":
                helpe()
                return None
            case "oertDvea","yearday":
                return dt.tm_yday
            case "öreo","hour":
                return dt.tm_hour
            case "dophkeo","minute":
                return dt.tm_min
            case "solfess","second" :
                return dt.tm_sec
            case "oert","year":
                return dt.tm_year
            case "moorwae","month":
                return dt.tm_mon
            case "heofteDvea","weekday":
                return dt.tm_wday
            case _:
                raise Exception("Unexcepted time")

if __name__ == "__main__":                
    new = main(None)
    res=new.fn_date("moorwae")
    print(res)
