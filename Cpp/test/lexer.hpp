#ifndef LEXER_HPP
#define LEXER_HPP

#include <string>
#include <vector>

enum class TokenType {
    // Literals
    NUMBER,
    STRING,
    IDENTIFIER,
    
    // Keywords
    IF,
    ELSE,
    WHILE,
    FOR,
    RETURN,
    
    // Operators
    PLUS,
    MINUS,
    STAR,
    SLASH,
    ASSIGN,
    EQ,
    NE,
    LT,
    GT,
    LE,
    GE,
    
    // Delimiters
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    SEMICOLON,
    COMMA,
    
    // Special
    END_OF_FILE,
    UNKNOWN
};

struct Token {
    TokenType type;
    std::string value;
    int line;
    int column;
};

class Lexer {
public:
    explicit Lexer(const std::string& source);
    
    Token nextToken();
    std::vector<Token> tokenize();
    
private:
    std::string source;
    size_t position;
    int line;
    int column;
    
    char current() const;
    char peek(size_t offset = 1) const;
    void advance();
    void skipWhitespace();
    void skipComment();
    
    Token makeToken(TokenType type, const std::string& value);
    Token readNumber();
    Token readString();
    Token readIdentifier();
};

#endif // LEXER_HPP