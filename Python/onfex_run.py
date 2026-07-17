import argparse
import time
import sys
import os
import pickle as pk

def resolve_path(file):
    # önce direkt var mı?
    if os.path.exists(file):
        return os.path.abspath(file)

    # yoksa workspace içinde ara
    for root in ["/public","sdcard","/public/onfex_aertnen"]:
        for path, dirs, files in os.walk(root):
            if file in files:
                return os.path.join(path, file)
    return None

def main():
    version = "1.6.2"
    sys.path.append(f"/storage/emulated/0/OnfexPlone/Python/{version}(Python)/")
    from cache.document import OnfexPloneDoph as OPD
    parser = argparse.ArgumentParser(
        prog="onfex",
        description="Onfex Compiler And Interpreter"
    )

    sub = parser.add_subparsers(dest="command")

    run = sub.add_parser("run", help="Compile and run")
    run.add_argument("doc", help="document name")

    build = sub.add_parser("build", help="Compile")
    build.add_argument("doc", help="document name")
    versionP = sub.add_parser("v", help="Show version")
    
    args = parser.parse_args()

    if args.command == "run":
        name = args.doc
        path = resolve_path(name).replace("/"+name,"")
        dc = OPD(path,name,None,False)
        dc.run()

    elif args.command == "build":
        name = args.doc
        path = resolve_path(name).replace("/"+name,"")
        dc = OPD(path,name,None,False)
        res = dc.binCernos()
        with open(path+"/__onfexcache__/"+name+"c","wb") as f:
            pk.dump(res,f)
        print(f"Onfex document was built as {name+'c'}")
    elif args.command == "v":
        print(f"Onfex {version}")
    else:
        parser.print_help()

if __name__ == "__main__":
    main()
    