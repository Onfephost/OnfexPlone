#include <iostream>
#include "lexer.hpp"

int main() {
    Lexer l;
    std::vector<Token> toks = l.lex("abc");
    std::cout << "done" << std::endl;
    return 0;
}


