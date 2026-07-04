#include <iostream>
#include <cctype>
#include <string>
#include <vector>
using namespace std;

class Lexer {
public:
    enum class TokenType {
        IDENTIFIER,
        NUMBER,
        EQUAL,
        PLUS,
        SYMBOL
    };

    struct Token {
        TokenType type;
        string value;
    };

    vector<Token> toks;

    vector<Token> tokenize(const string& input) {
        vector<Token> tokens;
        size_t i = 0;

        while (i < input.size()) {
            char c = input[i];

            if (isspace(static_cast<unsigned char>(c))) {
                ++i;
                continue;
            }

            if (isalpha(static_cast<unsigned char>(c)) || c == '_') {
                string value;
                while (i < input.size() && (isalnum(static_cast<unsigned char>(input[i])) || input[i] == '_')) {
                    value += input[i++];
                }
                tokens.push_back({TokenType::IDENTIFIER, value});
            } else if (isdigit(static_cast<unsigned char>(c))) {
                string value;
                while (i < input.size() && isdigit(static_cast<unsigned char>(input[i]))) {
                    value += input[i++];
                }
                tokens.push_back({TokenType::NUMBER, value});
            } else if (c == '=') {
                tokens.push_back({TokenType::EQUAL, string(1, c)});
                ++i;
            } else if (c == '+') {
                tokens.push_back({TokenType::PLUS, string(1, c)});
                ++i;
            }
        }
        toks = tokens;
        return tokens;
    }

    void printTokens(const vector<Token>& tokens) {
        for (const Token& token : tokens) {
            string typeName;
            if (token.type == TokenType::IDENTIFIER) {
                typeName = "IDENT";
            } else if (token.type == TokenType::NUMBER) {
                typeName = "NUMBER";
            } else if (token.type == TokenType::EQUAL) {
                typeName = "EQUAL";
            } else if (token.type == TokenType::PLUS) {
                typeName = "PLUS";  
            } else {
                typeName = "SYMBOL";
            }

            cout << typeName << ": " << token.value << endl;
        }
    }
};

