#ifndef LEXER_HPP_
#define LEXER_HPP_

#include <iostream>
#include <string>
#include <vector>

enum class Tok {
    Identifier,
    Number,
    End,
};

class Token {
protected:
    Tok tok;
    std::string value;
public:
    Token(Tok tk, const std::string& vl);
};

class Lexer {
public:
    std::vector<Token> lex(const std::string& source);
};

#endif