# lexer.py
import re
from dataclasses import dataclass

class Token:
    def __init__(self, type, value, line, col, pos):
        self.type = type
        self.value = value
        self.line = line
        self.col = col
        self.pos = pos

TYPES = {
    "intg","flotg","strg","boletg","listh","dicth","aphe",
    "typct_taphlot","typct_neom","veot",
    "kalf","karchen","peontderen",
}

KEYWORDS = {
"kalfen":"KALFEN","frounct":"FRCT","frutupheFrounct":"PFRCT","frutuphenos":"FUTUREC",
"ifnt":"IF","elsnt":"ELSE","efnt":"ELIF",
"peontDelnos":"PTRDEL","forp":"FOR","perl":"PERL","eower":"WHILE",
"keryenos":"FORPPERL","maphnos":"MAPHPERL","asken":"AS",
"serl":"SERL","inkleodnos":"IMPORT",
"kalfenCernos":"CLASSCRE","oed":"YIELD",
}


unary = {"brof":"AND","neat":"NOT","ophe":"OR","asp":"IS","mot":"MOD","intf":"IN"}
TOKEN_SPEC = [
    ("NUMBER",   r"-?\d+\.\d+|-?\d+"),("STRING",   r'"[^"]*"'),
    ("CHAR",     r"'[^']*'"),("BOOL",     r"(trunth|franth)"),("NULL",     r"(noph)"),
    ("EQEQ",     r"=="),("EQUAL",    r"="),

    ("PLUS",     r"\+"),("MINUS",    r"-"),("STAR",     r"\*"),("SLASH",    r"/"),("UP",    r"\^"),
    ("EQLT",     r"<="),("EQGT",     r">="),("LT",       r"<"),("GT",       r">"),("REF",       r"&"),

    ("LPAREN",   r"\("),("RPAREN",   r"\)"),
    
    ("LBRACE",   r"\{"),("RBRACE",   r"\}"),

    ("LBRACKET", r"\["),("RBRACKET", r"\]"),

    ("COMMA",    r","),("COLON",    r":"),("SEMI",     r";"),("POINT",    r"\."),

    ("IDENTIFEN",r"[a-zA-Z_][a-zA-Z0-9_öüİÖÜ]*"),
    ("LABEL",    r'#[^"]*#'),("SKIP",     r"[ \t\n]+"),
]
trns = {
    "IDENTIFEN":"texth",
    "POINT":"detot","COMMA":"vetot",
    "LPAREN":"lofPartot","RPAREN":"roghPartot",
    "LBRACE":"lofSupartot","RBRACE":"roghSupartot",
    "LBRACKET":"lofEpartot","RBRACKET":"roghEpartot",
    "COLON":"fedetot","SEMI":"vedetot","UP":"ipherLitot",
    "GT":"roghLitot","LT":"lofLitot",
    "KALFEN":"kalfen","FRCT":"frounct",
    "IF":"ifnt","ELSE":"elsnt","ELIF":"efnt",
    "PTRDEL":"peontDelnos",
    "MEHEN":"mehen","FOR":"forp","PERL":"perl","SERL":"serl",
    "FORPPERL":"keryenos","MAPHPERL":"maphnos","AS":"asken",
    "TYPE":"typect","EQUAL":"qenev(=)","EQEQ":"duphAfonQenev",
    "EQLT":"kanevOphQenev","EQGT":"banevOphQenev",
    "SLASH":"slatot","PLUS":"adfoss","MINUS":"edfoss","YIELD":"oed",
    "STRING":"sterg","NULL":"nophnev","NUMBER":"neom","STAR":"keush","CHAR":"karch"
}
def trans(a):
    return trns.get(a,"texth")

TOKEN_REGEX = "|".join(f"(?P<{n}>{r})" for n, r in TOKEN_SPEC)
def lex(code: str):
    tokens = []
    line = 1
    col = 0    
    for m in re.finditer(TOKEN_REGEX, code):
        kind = m.lastgroup;value = m.group()
        start_pos = m.start()
        end_pos = m.end()
        # satır ve sütun takibi
        newlines = value.count("\n")
        if newlines > 0:line += newlines;col = len(value) - value.rfind("\n") - 1
        else:col += len(value)
        if kind in ("SKIP","LABEL"):
            continue
        # TYPE ayrımı
        if kind == "IDENTIFEN" and value in TYPES:kind = "TYPE"
        if kind == "IDENTIFEN" and value in KEYWORDS:kind = KEYWORDS[value]
        if kind == "IDENTIFEN" and value in unary:kind = unary[value]
        tokens.append(Token(kind, value, line, col - len(value), (start_pos, end_pos)))
    return tokens