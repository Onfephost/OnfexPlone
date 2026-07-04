from cache.ast_nodes import *
from cache.Exceptions import *
import os
import cache.lexer as lex

def rex(a,l1,l2):
    rs = a
    for a,b in zip(l1,l2):rs = rs.replace(a,b)
    return rs
    
class Parser:
    def __init__(self, tokens):
        self.tokens = tokens
        self.pos = 0

    # ---------------- helpers ----------------
    def current(self):
        if self.pos >= len(self.tokens):return None
        return self.tokens[self.pos]

    def peek(self, n=1):
        pos = self.pos + n
        if pos >= len(self.tokens):return None
        return self.tokens[pos]
    
    def peeks(self,*args):
        l = []
        for i in args:
            res = self.peek(args.index(i)+1)
            if res:l.append(res.type == i)
            else:l.append(False)
        return all(l)

    def eat(self, type_):
        self.pos -= 1;last = self.current()
        self.pos += 1;tok = self.current()
        if not tok:raiseE(tok,"Syntax Error",f"Expected \"{type_}\" but got EOF")
        if tok.type != type_:            
            raiseE(tok,"Sentex Errn", f"{lex.trans(type_)} aps wraithvognosan apht {lex.trans(tok.type)} asp gephvognosan")
        self.pos += 1
        return tok

    # ---------------- program ----------------
    def parse(self):       
        nodes = []
        while self.current():
            nodes.append(self.statement())
        return Block(self.tokens[0], nodes)

    # ---------------- statement ----------------
    def statement(self):
        tok = self.current()
        if tok.value == "mehen":
            tk = self.eat("IDENTIFEN")
            b = self.block()
            return Mehen(tk,b)
        if tok.value == "brontnos":TRK = self.eat("IDENTIFEN");return Break(TRK)
        if tok.value == "krotnos":TRK = self.eat("IDENTIFEN");return Continue(TRK)
        if tok.type == "IDENTIFEN" and self.peeks("POINT","IDENTIFEN","EQUAL"):return self.memberAsg()
        if tok.value == "retrunos":return self.ret()
        if tok.value == "mot":
            return self.modimp()
        if tok.type == "FOR":return self.forp()
        if tok.value == "inkleodnos" and self.peeks("IDENTIFEN","AS","IDENTIFEN"):return self.importAs()
        if tok.type == "SWITCH":
            return self.switch()
        if tok.value == "inkleodnos":return self.imp()
        if tok.type == "FRCT":return self.func()
        if tok.type == "PFRCT":return self.pfunc()
        if tok.type == "KALFEN":return self.parse_kalfen()
        if tok.type == "IF":return self.if_stmt()
        if tok.value == "delnosIyndexe":return self.delete()
        if tok.type == "YIELD":return self.yld()
        if tok.type == "WHILE":return self.whil()
        if tok.type == "PTRDEL":return self.deleteptr()
        if tok.value == "wrossnosMot":return self.wrossnosMot()
        if tok.value == "wrossnosLribe":return self.wrossnosLribe()
        res =  self.expr()
        self.eat("SEMI")
        return res

    # ---------------- main ----------------
    def parse_kalfen(self):
        tok=self.eat("KALFEN")
        name = self.eat("IDENTIFEN").value
        i = None
        if self.current().type == "COLON":
            self.eat("COLON")
            i = self.eat("IDENTIFEN")
        body = self.block()
        return KalfenNode(tok,i,name, body)
        
    def ret(self):
        TRK = self.eat("IDENTIFEN");
        self.eat("LT")
        args = self.expr()
        self.eat("SEMI")
        return Return(TRK, args)
        
    def switch(self):
        print("amk2")
        ptok=self.eat("SWITCH")
        bodies = []
        values = []
        arg = self.enter1()
        self.eat("LBRACE")
        while self.current() and self.current().type == "CASE":
            self.eat("CASE")
            values.append(self.enter1())
            bodies.append(self.block())
        self.eat("RBRACE")
        df = Block(ptok,[])
        if self.current() and self.current().value == "defeal":
            self.eat("IDENTIFEN")
            df = self.block()
        return Switch(ptok,arg,values,bodies,df)

    def enters(self):
        self.eat("LPAREN");args = []
        if self.current().type != "RPAREN":
            args.append(self.expr())
            while self.current().type == "COMMA":self.eat("COMMA");args.append(self.expr())
        self.eat("RPAREN")
        return args
        
    def inters(self):
        args = []
        if self.current().type != "COMMA":
            args.append(self.expr())
            while self.current().type == "COMMA":
                self.eat("COMMA")
                args.append(self.expr())
        return args
    
        
    def enter1(self):
        self.eat("LPAREN");arg = None
        if self.current().type != "RPAREN":
            arg = self.expr()
        self.eat("RPAREN")
        return arg
    
    def wrossnosMot(self):
        self.eat("IDENTIFEN")
        tok = self.eat("IDENTIFEN")
        self.eat("EQUAL")
        self.eat("GT")
        name = self.eat("IDENTIFEN")
        self.eat("SEMI")
        return TypingModul(tok, tok, name)
     
    def wrossnosLribe(self):
        self.eat("IDENTIFEN")
        tok = self.eat("IDENTIFEN")
        self.eat("EQUAL")
        self.eat("GT")
        name = self.eat("IDENTIFEN")
        self.eat("SEMI")
        return TypingLib(tok, tok, name)

    def memberAsg(self):
        print("memberAsg")
        obj_tok = self.eat("IDENTIFEN"); obj = Variable(obj_tok, obj_tok.value)
        ptok = self.eat("POINT"); name_tok = self.eat("IDENTIFEN")
        self.eat("EQUAL"); value = self.expr(); self.eat("SEMI")
        member = MemberAccess(ptok, obj, name_tok)
        return Assign(obj_tok, member, None, value)
        
    def forp(self):
        tok = self.eat("FOR");
        fs = self.inters()
        self.eat("IN");
        e = self.enter1()
        sc = self.block()
        return ForpNode(tok, fs,e, sc)
    
    def whil(self):
        tok = self.eat("WHILE")
        cnd = self.enter1()
        self.eat("PERL")
        b = self.block()
        return While(tok,cnd,b)
        
    def importAs(self):
        tok = self.eat("IMPORT")
        lib = self.eat("IDENTIFEN")
        tok = self.eat("AS")
        As = self.eat("IDENTIFEN").value
        self.eat("SEMI")
        return ImportAs(tok, lib, As)
                
    def imp(self):
        tok = self.eat("IMPORT");
        lib = self.eat("IDENTIFEN");
        self.eat("SEMI")
        return ImportAs(tok, lib,lib)
    
    def modimp(self):
        tok = self.eat("MOD");
        print(233)
        lib = self.eat("IDENTIFEN")
        if self.current().type== "SLASH":
            print(2)
            while self.current().type == "SLASH":
                self.eat("SLASH")
                res = self.eat("IDENTIFEN").value
                lib.value = lib.value +"/"+ res
        self.eat("SEMI")
        return ModulImport(tok, lib,lib)

    # ---------------- block ----------------
    def block(self):
        tok = self.eat("LBRACE");nodes = []
        if self.current():            
            while self.current().type != "RBRACE":nodes.append(self.statement())
            self.eat("RBRACE")
        return Block(tok, nodes)

    # ---------------- function ----------------
    def func(self):
        tok = self.eat("FRCT");name = self.eat("IDENTIFEN").value
        self.eat("LPAREN");params = [];varagr = None
        setparams ={}
        sp = False
        if self.current().type != "RPAREN":
        # ilk param
            if self.current().type == "STAR":self.eat("STAR");varagr = self.eat("IDENTIFEN").value
            else:
                 params.append(self.eat("IDENTIFEN").value)
                 setparams = {params[-1]: None}
                 if self.current().type == "EQUAL":
                    sp = True
                    self.eat("EQUAL")
                    setparams[params[-1]] = self.expr()
        # devamı
            while self.current().type == "COMMA":
                self.eat("COMMA")
                if self.current().type == "STAR":
                    self.eat("STAR");varagr = self.eat("IDENTIFEN").value
                    break
                params.append(self.eat("IDENTIFEN").value)
                setparams[params[-1]] = None
                if self.current().type == "EQUAL":
                    sp = True
                    self.eat("EQUAL")
                    setparams[params[-1]] = self.expr()
                elif sp:
                    raiseE(self.current(), "Syntax Error", "Positional argument follows keyword argument")
        self.eat("RPAREN")
        body = self.block()
        return Func(tok, name, params, varagr, body, setparams)

    def pfunc(self):
        tok = self.eat("PFRCT")        
        probs = self.dict_literal()
        name = self.eat("IDENTIFEN").value
        self.eat("LPAREN")
        params = []
        setparams = {}
        sp = False
        if self.current().type != "RPAREN":
        # ilk param
            params.append(self.eat("IDENTIFEN").value)
            setparams = {params[-1]: None}
            if self.current().type == "EQUAL":
                sp = True
                self.eat("EQUAL")
                setparams = {params[-1]: self.expr()}
        # devamı
            while self.current().type == "COMMA":
                self.eat("COMMA")
                if self.current().type == "STAR":
                    self.eat("STAR");varagr = self.eat("IDENTIFEN").value;break
                params.append(self.eat("IDENTIFEN").value)
                setparams[params[-1]] = None
                if self.current().type == "EQUAL":
                    sp = True
                    self.eat("EQUAL")
                    setparams[params[-1]] = self.expr()
                elif sp:
                    raiseE(self.current(), "Syntax Error", "Positional argument follows keyword argument")
        self.eat("RPAREN")
        
        body = self.block()
        return FutureFunc(tok,probs, name, params,body,setparams)
        
    
    # ---------------- delete ----------------
    def delete(self):
        tok = self.eat("IDENTIFEN")
        target = self.expr()
        self.eat("SEMI")
        return IndexDelete(tok, target.target, target.index)
        
    def deleteptr(self):
        tok = self.eat("PTRDEL")
        target = self.expr()
        self.eat("SEMI")
        return PointerDel(tok, target)
        
    def yld(self):
        tok = self.eat("YIELD");target = self.enters();self.eat("SEMI")
        return Yield(tok, target)
    
    # ---------------- assign ----------------

    # ---------------- if ----------------
    def if_stmt(self):
        tok = self.eat("IF")        
        cond = self.enter1()
        body = self.block()
        elif_bodies = []
        cur_cond = None
        cur_body = None
        else_body = None
        if self.current() and self.current().type == "ELSE" and self.peek() and self.peek().type == "IF":
            while self.current() and self.current().type == "ELSE" and self.peek() and self.peek().type == "IF":
                tk = self.eat("ELSE")
                self.eat("IF")
                cur_cond = self.enter1()
                cur_body = self.block()
                elif_bodies.append(Elif(tk,cur_cond,cur_body))
        if self.current() and self.current().type == "ELSE":
            self.eat("ELSE");else_body = self.block()
        if len(elif_bodies) == 0:elif_bodies = None
        return If(tok, cond, body,elif_bodies, else_body)

    # ---------------- expression ----------------
    def expr(self):
        left = self.chain(self.primary())
        # assignment Variable, MemberAssign, DataAttr
        if self.current() and self.current().type == "EQUAL":
            if isinstance(left,(Variable, MemberAccess, DataAttr,IndexAccess,ModulVariable,LibVariable)):
                tok = self.eat("EQUAL");value = self.expr()
                if isinstance(left, IndexAccess):
                    return Assign(left,left,None,value)
                return Assign(tok, left, None, value)
            else:raiseE(self.current(), "Syntax Error", "Cannot assign to this expression")
        return self.binary_op(left)
        
    def binary_op(self, left):
        while self.current() and self.current().type in ("PLUS","MINUS","STAR","SLASH","GT",
        "LT","EQEQ","EQGT","EQLT","AND","OR","IS","MOD","IN","UP"):
            op = self.eat(self.current().type)
            right = self.chain(self.primary())
            left = BinOp(left, op.type, right,op)
        return left

    # ---------------- primary ----------------
    def primary(self):
        tok = self.current()
        if tok.type == "LPAREN":
            return self.enter1()
        if tok.type == "NUMBER":
            tok = self.eat("NUMBER");val = eval(tok.value)
            return Float(tok,val) if isinstance(val,float) else Int(tok,val)
        if tok.type == "STRING":
            tok = self.eat("STRING")
            res = rex(tok.value[1:-1],["{mr}","{t}"],["\n","\t"])
            return String(tok,res)
        if tok.type == "BOOL":
            tok = self.eat("BOOL");return Bool(tok,tok.value == "trunth")
        if tok.type == "NULL":
            tok = self.eat("NULL");return Null(tok)
        if tok.type == "NOT":
            op = self.eat("NOT");expr = self.expr()
            return UnaryOp(op, expr)
        
        if tok.type == "IDENTIFEN":
            tok = self.eat("IDENTIFEN")
            name = tok.value
            node = Variable(tok, name)
            if tok.value == "peontOft":
                ex = self.expr()
                return PointerGet(tok,ex)            
            
            if self.current() and self.current().type == "LPAREN":
                args = self.enters()
                bb = DictLiteral(tok,[(Int(tok,0),Null(tok))])
                if self.current().type == "LBRACE":
                    bb = self.dict_literal()
                node = Call(tok, node, args,bb)
            
            # Lib call öncelikli kontrol
            if self.current() and self.current().type == "COLON" and self.peek() and self.peek().type == "COLON":
                self.eat("COLON");self.eat("COLON")
                func_name = self.eat("IDENTIFEN")
                if not self.current().type == "LPAREN":
                    return LibVariable(tok, name, func_name)
                args = self.enters()
                dct = DictLiteral(tok,[(Int(tok,0),Null(tok))])
                if self.current() and self.current().type == "LBRACE":
                    dct = self.dict_literal()
                return LibCall(tok, name, func_name, args,dct)
                
            if self.current() and self.current().type == "MINUS" and self.peek() and self.peek().type == "GT":
                self.eat("MINUS");self.eat("GT")
                func_name = self.eat("IDENTIFEN")
                if not self.current().type == "LPAREN":
                    node = ModulVariable(tok, name, func_name)
                else:
                    node = ModulVariable(tok, name, func_name)
                    args = self.enters()
                    return Call(tok, node, args,None)
 
            return self.chain(node)
        if tok.type == "LBRACKET":return self.list_literal()
        if tok.type == "LBRACE":return self.dict_literal()
        if tok.type == "TYPE":
            res = self.eat("TYPE")            
            return Type(res,res.value)
        raiseE(tok, "Syntax Error", f"Invalid expression {lex.trans(tok.type)}")

    # ---------------- chain ----------------
    def chain(self, node):
        while self.current() and self.current().type in ("POINT", "LBRACKET"):
            if self.current().type == "POINT":
                ptok = self.eat("POINT");ntok = self.eat("IDENTIFEN");name = ntok.value
                # obj.lib::func() kontrolü
                if self.current() and self.current().type == "COLON" and self.peek() and self.peek().type == "COLON":
                    self.eat("COLON");self.eat("COLON")
                    func = self.eat("IDENTIFEN")
                    args = self.enters()
                    dc =DictLiteral(ptok,[(Int(ptok,0),Null(ptok))])
                    if self.current() and self.current().type == "LBRACE":
                        dc = self.dict_literal()
                    node = LibMethodCall(ptok,name,node,func,args,dc)
                # normal method call
                elif self.current() and self.current().type == "LPAREN":
                    args = self.enters()
                    dc = DictLiteral(ptok,[(Int(ptok,0),Null(ptok))])
                    if self.current() and self.current().type == "LBRACE":
                        dc = self.dict_literal()
                    node = MethodCall(ptok, node, name, args,dc)
                # normal attribute
                else:
                    node = MemberAccess(ptok, node, ntok)

            elif self.current().type == "LBRACKET":
                btok = self.eat("LBRACKET")
                idx = self.expr()
                self.eat("RBRACKET")
                node = IndexAccess(btok, node, idx)
        return node

    # ---------------- list ----------------
    def list_literal(self):
        tok = self.eat("LBRACKET");items = []
        if self.current().type != "RBRACKET":
            items.append(self.expr())
            while self.current().type == "COMMA":
                self.eat("COMMA");items.append(self.expr())
        self.eat("RBRACKET")
        return ListLiteral(tok, items)

    # ---------------- dict ----------------
    def dict_literal(self):
        tok = self.eat("LBRACE");pairs = []
        while self.current().type != "RBRACE":
            key = self.expr();self.eat("COLON")
            val = self.expr();pairs.append((key, val))
            if self.current().type == "COMMA":self.eat("COMMA")
        self.eat("RBRACE")
        return DictLiteral(tok, pairs)