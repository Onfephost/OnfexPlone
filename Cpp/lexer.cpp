#include "lexer.hpp"

Token::Token(Tok tk, const std::string& vl)
    : tok(tk), value(vl)
{
}

std::vector<Token> Lexer::lex(const std::string& source)
{
    std::vector<Token> tokens;
    if (!source.empty()) {
        tokens.emplace_back(Tok::Identifier, source);
    }
    tokens.emplace_back(Tok::End, "EOF");
    return tokens;
}


