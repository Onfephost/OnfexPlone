#pragma once

#include <cstdint>
#include <iostream>
#include <memory>
#include <optional>
#include <sstream>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <utility>
#include <vector>

namespace onfex {

struct Value {
    enum class Type { Null, Number, String, Bool } type = Type::Null;
    double number = 0.0;
    std::string text;
    bool boolean = false;

    static Value make_number(double value);
    static Value make_string(const std::string& value);
    static Value make_bool(bool value);
    static Value make_null();
    std::string to_string() const;
    bool is_truthy() const;
};

struct Expr;
struct Stmt;
using ExprPtr = std::shared_ptr<Expr>;
using StmtPtr = std::shared_ptr<Stmt>;

struct Expr {
    enum class Kind { Number, String, Bool, Variable, Unary, Binary, Call, Group } kind;
    std::string text;
    double number = 0.0;
    bool boolean = false;
    std::string op;
    ExprPtr left;
    ExprPtr right;
    std::vector<ExprPtr> args;
    std::string name;
};

struct Stmt {
    enum class Kind { Program, Assign, If, Return, Print, Function, Block, ExprStmt } kind;
    std::string name;
    std::vector<std::string> params;
    std::vector<StmtPtr> body;
    std::vector<StmtPtr> else_body;
    std::vector<StmtPtr> statements;
    ExprPtr expr;
    ExprPtr condition;
};

struct FunctionDef {
    std::vector<std::string> params;
    std::vector<StmtPtr> body;
};

class Lexer {
public:
    explicit Lexer(std::string source);
    struct Token {
        enum class Type {
            End,
            Identifier,
            Number,
            String,
            Plus,
            Minus,
            Star,
            Slash,
            Equal,
            EqualEqual,
            BangEqual,
            Less,
            LessEqual,
            Greater,
            GreaterEqual,
            LParen,
            RParen,
            LBrace,
            RBrace,
            Comma,
            SemiColon,
            And,
            Or
        } type = Type::End;
        std::string text;
        double number = 0.0;
    };

    Token next_token();
    Token peek() const;

private:
    void skip_whitespace();
    std::string source_;
    std::size_t index_ = 0;
    Token current_token_;
};

class Parser {
public:
    explicit Parser(Lexer lexer);
    StmtPtr parse_program();

private:
    StmtPtr parse_statement();
    StmtPtr parse_block();
    StmtPtr parse_function();
    StmtPtr parse_if();
    StmtPtr parse_assign();
    StmtPtr parse_print();
    StmtPtr parse_return();
    StmtPtr parse_expression_statement();
    ExprPtr parse_expression();
    ExprPtr parse_or();
    ExprPtr parse_and();
    ExprPtr parse_equality();
    ExprPtr parse_comparison();
    ExprPtr parse_additive();
    ExprPtr parse_multiplicative();
    ExprPtr parse_unary();
    ExprPtr parse_primary();
    std::vector<ExprPtr> parse_arguments();
    void consume(Lexer::Token::Type type, const std::string& message);
    bool check(Lexer::Token::Type type) const;
    bool match(Lexer::Token::Type type);
    Lexer::Token current() const;
    Lexer::Token advance();
    Lexer lexer_;
    Lexer::Token current_token_;
};

class Interpreter {
public:
    Value evaluate(const ExprPtr& expr);
    void execute(const StmtPtr& stmt);
    void run_program(const StmtPtr& program);
    static void run_file(const std::string& path);

private:
    void execute_block(const std::vector<StmtPtr>& statements, bool new_scope);
    Value evaluate_call(const std::string& name, const std::vector<ExprPtr>& args);
    void set_variable(const std::string& name, const Value& value);
    Value get_variable(const std::string& name) const;
    std::vector<std::unordered_map<std::string, Value>> scopes_;
    std::unordered_map<std::string, std::shared_ptr<FunctionDef>> functions_;
    std::optional<Value> return_value_;
};

void run_file(const std::string& path);

}  // namespace onfex
