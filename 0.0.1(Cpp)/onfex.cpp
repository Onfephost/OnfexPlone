#include "onfex.hpp"

#include <fstream>
#include <functional>
#include <stdexcept>

namespace onfex {
namespace {

std::string read_file(const std::string& path) {
    std::ifstream input(path);
    if (!input) {
        throw std::runtime_error("Unable to read file: " + path);
    }
    std::ostringstream buffer;
    buffer << input.rdbuf();
    return buffer.str();
}

Value make_binary_result(const std::string& op, const Value& left, const Value& right) {
    if (left.type != Value::Type::Number || right.type != Value::Type::Number) {
        throw std::runtime_error("Binary operations require numbers");
    }
    double a = left.number;
    double b = right.number;
    if (op == "+") return Value::make_number(a + b);
    if (op == "-") return Value::make_number(a - b);
    if (op == "*") return Value::make_number(a * b);
    if (op == "/") {
        if (b == 0.0) throw std::runtime_error("Division by zero");
        return Value::make_number(a / b);
    }
    if (op == "<") return Value::make_bool(a < b);
    if (op == "<=") return Value::make_bool(a <= b);
    if (op == ">") return Value::make_bool(a > b);
    if (op == ">=") return Value::make_bool(a >= b);
    if (op == "==") return Value::make_bool(a == b);
    if (op == "!=") return Value::make_bool(a != b);
    throw std::runtime_error("Unsupported operator: " + op);
}

}  // namespace

Value Value::make_number(double value) {
    return Value{Type::Number, value, "", false};
}

Value Value::make_string(const std::string& value) {
    return Value{Type::String, 0.0, value, false};
}

Value Value::make_bool(bool value) {
    return Value{Type::Bool, 0.0, "", value};
}

Value Value::make_null() {
    return Value{};
}

std::string Value::to_string() const {
    switch (type) {
        case Type::Null: return "noph";
        case Type::Number: return std::to_string(number);
        case Type::String: return text;
        case Type::Bool: return boolean ? "trunth" : "franth";
    }
    return "";
}

bool Value::is_truthy() const {
    if (type == Type::Bool) return boolean;
    if (type == Type::Number) return number != 0.0;
    if (type == Type::String) return !text.empty();
    return false;
}

Lexer::Lexer(std::string source) : source_(std::move(source)) {
    current_token_ = next_token();
}

Lexer::Token Lexer::next_token() {
    skip_whitespace();
    if (index_ >= source_.size()) {
        return Token{Token::Type::End};
    }
    char ch = source_[index_++];
    if (std::isalpha(static_cast<unsigned char>(ch)) || ch == '_') {
        std::string text;
        text.push_back(ch);
        while (index_ < source_.size() && (std::isalnum(static_cast<unsigned char>(source_[index_])) || source_[index_] == '_')) {
            text.push_back(source_[index_++]);
        }
        if (text == "valt" || text == "let") return Token{Token::Type::Identifier, text};
        if (text == "frounct" || text == "frutupheFrounct") return Token{Token::Type::Identifier, text};
        if (text == "mehen") return Token{Token::Type::Identifier, text};
        if (text == "ifnt") return Token{Token::Type::Identifier, text};
        if (text == "elsnt") return Token{Token::Type::Identifier, text};
        if (text == "retrunos") return Token{Token::Type::Identifier, text};
        if (text == "pyrintnos") return Token{Token::Type::Identifier, text};
        if (text == "trunth") return Token{Token::Type::Identifier, text};
        if (text == "franth") return Token{Token::Type::Identifier, text};
        if (text == "noph") return Token{Token::Type::Identifier, text};
        return Token{Token::Type::Identifier, text};
    }
    if (std::isdigit(static_cast<unsigned char>(ch))) {
        std::string text;
        text.push_back(ch);
        while (index_ < source_.size() && std::isdigit(static_cast<unsigned char>(source_[index_]))) {
            text.push_back(source_[index_++]);
        }
        if (index_ < source_.size() && source_[index_] == '.') {
            text.push_back(source_[index_++]);
            while (index_ < source_.size() && std::isdigit(static_cast<unsigned char>(source_[index_]))) {
                text.push_back(source_[index_++]);
            }
        }
        return Token{Token::Type::Number, text, std::stod(text)};
    }
    if (ch == '"') {
        std::string text;
        while (index_ < source_.size() && source_[index_] != '"') {
            text.push_back(source_[index_++]);
        }
        if (index_ < source_.size()) ++index_;
        return Token{Token::Type::String, text};
    }
    switch (ch) {
        case '+': return Token{Token::Type::Plus, "+"};
        case '-': return Token{Token::Type::Minus, "-"};
        case '*': return Token{Token::Type::Star, "*"};
        case '/': return Token{Token::Type::Slash, "/"};
        case '=': return Token{Token::Type::Equal, "="};
        case '!': return Token{Token::Type::BangEqual, "!"};
        case '<': return Token{Token::Type::Less, "<"};
        case '>': return Token{Token::Type::Greater, ">"};
        case '(': return Token{Token::Type::LParen, "("};
        case ')': return Token{Token::Type::RParen, ")"};
        case '{': return Token{Token::Type::LBrace, "{"};
        case '}': return Token{Token::Type::RBrace, "}"};
        case ',': return Token{Token::Type::Comma, ","};
        case ';': return Token{Token::Type::SemiColon, ";"};
        default: throw std::runtime_error(std::string("Unexpected character: ") + ch);
    }
}

void Lexer::skip_whitespace() {
    while (index_ < source_.size() && std::isspace(static_cast<unsigned char>(source_[index_]))) ++index_;
}

Lexer::Token Lexer::peek() const { return current_token_; }

Parser::Parser(Lexer lexer) : lexer_(std::move(lexer)) {
    current_token_ = lexer_.peek();
}

Lexer::Token Parser::current() const { return current_token_; }

Lexer::Token Parser::advance() {
    Lexer::Token token = current_token_;
    current_token_ = lexer_.next_token();
    return token;
}

bool Parser::check(Lexer::Token::Type type) const { return current_token_.type == type; }

bool Parser::match(Lexer::Token::Type type) {
    if (check(type)) {
        advance();
        return true;
    }
    return false;
}

void Parser::consume(Lexer::Token::Type type, const std::string& message) {
    if (!match(type)) {
        throw std::runtime_error(message);
    }
}

StmtPtr Parser::parse_program() {
    std::vector<StmtPtr> statements;
    while (!check(Lexer::Token::Type::End)) {
        if (check(Lexer::Token::Type::SemiColon)) {
            advance();
            continue;
        }
        statements.push_back(parse_statement());
    }
    auto program = std::make_shared<Stmt>();
    program->kind = Stmt::Kind::Program;
    program->statements = std::move(statements);
    return program;
}

StmtPtr Parser::parse_statement() {
    if (check(Lexer::Token::Type::Identifier) && current_token_.text == "frounct") {
        return parse_function();
    }
    if (check(Lexer::Token::Type::Identifier) && current_token_.text == "frutupheFrounct") {
        return parse_function();
    }
    if (check(Lexer::Token::Type::Identifier) && current_token_.text == "mehen") {
        advance();
        auto block = std::make_shared<Stmt>();
        block->kind = Stmt::Kind::Block;
        block->body = parse_block()->body;
        return block;
    }
    if (check(Lexer::Token::Type::Identifier) && current_token_.text == "ifnt") {
        return parse_if();
    }
    if (check(Lexer::Token::Type::Identifier) && current_token_.text == "pyrintnos") {
        return parse_print();
    }
    if (check(Lexer::Token::Type::Identifier) && current_token_.text == "retrunos") {
        return parse_return();
    }
    if (check(Lexer::Token::Type::Identifier) && current_token_.text == "valt") {
        return parse_assign();
    }
    if (check(Lexer::Token::Type::Identifier) && current_token_.text == "let") {
        return parse_assign();
    }
    return parse_expression_statement();
}

StmtPtr Parser::parse_block() {
    consume(Lexer::Token::Type::LBrace, "Expected '{'");
    std::vector<StmtPtr> statements;
    while (!check(Lexer::Token::Type::RBrace) && !check(Lexer::Token::Type::End)) {
        if (check(Lexer::Token::Type::SemiColon)) {
            advance();
            continue;
        }
        statements.push_back(parse_statement());
    }
    consume(Lexer::Token::Type::RBrace, "Expected '}'");
    auto block = std::make_shared<Stmt>();
    block->kind = Stmt::Kind::Block;
    block->body = std::move(statements);
    return block;
}

StmtPtr Parser::parse_function() {
    advance();
    auto function = std::make_shared<Stmt>();
    function->kind = Stmt::Kind::Function;
    if (!check(Lexer::Token::Type::Identifier)) {
        throw std::runtime_error("Expected function name");
    }
    function->name = advance().text;
    consume(Lexer::Token::Type::LParen, "Expected '(' after function name");
    while (!check(Lexer::Token::Type::RParen)) {
        if (!check(Lexer::Token::Type::Identifier)) {
            throw std::runtime_error("Expected parameter name");
        }
        function->params.push_back(advance().text);
        if (!match(Lexer::Token::Type::Comma)) break;
    }
    consume(Lexer::Token::Type::RParen, "Expected ')' after parameters");
    function->body = parse_block()->body;
    return function;
}

StmtPtr Parser::parse_if() {
    advance();
    consume(Lexer::Token::Type::LParen, "Expected '(' after if");
    auto stmt = std::make_shared<Stmt>();
    stmt->kind = Stmt::Kind::If;
    stmt->condition = parse_expression();
    consume(Lexer::Token::Type::RParen, "Expected ')' after if condition");
    stmt->body = parse_block()->body;
    if (check(Lexer::Token::Type::Identifier) && current_token_.text == "elsnt") {
        advance();
        stmt->else_body = parse_block()->body;
    }
    return stmt;
}

StmtPtr Parser::parse_assign() {
    auto stmt = std::make_shared<Stmt>();
    stmt->kind = Stmt::Kind::Assign;
    advance();
    if (!check(Lexer::Token::Type::Identifier)) {
        throw std::runtime_error("Expected variable name");
    }
    stmt->name = advance().text;
    consume(Lexer::Token::Type::Equal, "Expected '='");
    stmt->expr = parse_expression();
    if (match(Lexer::Token::Type::SemiColon)) {
        // optional
    }
    return stmt;
}

StmtPtr Parser::parse_print() {
    advance();
    auto stmt = std::make_shared<Stmt>();
    stmt->kind = Stmt::Kind::Print;
    stmt->expr = parse_expression();
    if (match(Lexer::Token::Type::SemiColon)) {
        // optional
    }
    return stmt;
}

StmtPtr Parser::parse_return() {
    advance();
    auto stmt = std::make_shared<Stmt>();
    stmt->kind = Stmt::Kind::Return;
    stmt->expr = parse_expression();
    if (match(Lexer::Token::Type::SemiColon)) {
        // optional
    }
    return stmt;
}

StmtPtr Parser::parse_expression_statement() {
    auto stmt = std::make_shared<Stmt>();
    stmt->kind = Stmt::Kind::ExprStmt;
    stmt->expr = parse_expression();
    if (match(Lexer::Token::Type::SemiColon)) {
        // optional
    }
    return stmt;
}

ExprPtr Parser::parse_expression() { return parse_or(); }

ExprPtr Parser::parse_or() {
    auto expr = parse_and();
    while (check(Lexer::Token::Type::Identifier) && current_token_.text == "or") {
        advance();
        auto right = parse_and();
        auto node = std::make_shared<Expr>();
        node->kind = Expr::Kind::Binary;
        node->op = "or";
        node->left = expr;
        node->right = right;
        expr = node;
    }
    return expr;
}

ExprPtr Parser::parse_and() {
    auto expr = parse_equality();
    while (check(Lexer::Token::Type::Identifier) && current_token_.text == "and") {
        advance();
        auto right = parse_equality();
        auto node = std::make_shared<Expr>();
        node->kind = Expr::Kind::Binary;
        node->op = "and";
        node->left = expr;
        node->right = right;
        expr = node;
    }
    return expr;
}

ExprPtr Parser::parse_equality() {
    auto expr = parse_comparison();
    while (check(Lexer::Token::Type::EqualEqual) || check(Lexer::Token::Type::BangEqual)) {
        std::string op = advance().text;
        auto right = parse_comparison();
        auto node = std::make_shared<Expr>();
        node->kind = Expr::Kind::Binary;
        node->op = op;
        node->left = expr;
        node->right = right;
        expr = node;
    }
    return expr;
}

ExprPtr Parser::parse_comparison() {
    auto expr = parse_additive();
    while (check(Lexer::Token::Type::Less) || check(Lexer::Token::Type::LessEqual) || check(Lexer::Token::Type::Greater) || check(Lexer::Token::Type::GreaterEqual)) {
        std::string op = advance().text;
        auto right = parse_additive();
        auto node = std::make_shared<Expr>();
        node->kind = Expr::Kind::Binary;
        node->op = op;
        node->left = expr;
        node->right = right;
        expr = node;
    }
    return expr;
}

ExprPtr Parser::parse_additive() {
    auto expr = parse_multiplicative();
    while (check(Lexer::Token::Type::Plus) || check(Lexer::Token::Type::Minus)) {
        std::string op = advance().text;
        auto right = parse_multiplicative();
        auto node = std::make_shared<Expr>();
        node->kind = Expr::Kind::Binary;
        node->op = op;
        node->left = expr;
        node->right = right;
        expr = node;
    }
    return expr;
}

ExprPtr Parser::parse_multiplicative() {
    auto expr = parse_unary();
    while (check(Lexer::Token::Type::Star) || check(Lexer::Token::Type::Slash)) {
        std::string op = advance().text;
        auto right = parse_unary();
        auto node = std::make_shared<Expr>();
        node->kind = Expr::Kind::Binary;
        node->op = op;
        node->left = expr;
        node->right = right;
        expr = node;
    }
    return expr;
}

ExprPtr Parser::parse_unary() {
    if (match(Lexer::Token::Type::Minus)) {
        auto node = std::make_shared<Expr>();
        node->kind = Expr::Kind::Unary;
        node->op = "-";
        node->right = parse_unary();
        return node;
    }
    return parse_primary();
}

ExprPtr Parser::parse_primary() {
    if (check(Lexer::Token::Type::Number)) {
        auto node = std::make_shared<Expr>();
        node->kind = Expr::Kind::Number;
        node->number = advance().number;
        return node;
    }
    if (check(Lexer::Token::Type::String)) {
        auto node = std::make_shared<Expr>();
        node->kind = Expr::Kind::String;
        node->text = advance().text;
        return node;
    }
    if (check(Lexer::Token::Type::Identifier) && current_token_.text == "trunth") {
        advance();
        auto node = std::make_shared<Expr>();
        node->kind = Expr::Kind::Bool;
        node->boolean = true;
        return node;
    }
    if (check(Lexer::Token::Type::Identifier) && current_token_.text == "franth") {
        advance();
        auto node = std::make_shared<Expr>();
        node->kind = Expr::Kind::Bool;
        node->boolean = false;
        return node;
    }
    if (check(Lexer::Token::Type::Identifier) && current_token_.text == "noph") {
        advance();
        auto node = std::make_shared<Expr>();
        node->kind = Expr::Kind::Number;
        node->number = 0.0;
        return node;
    }
    if (check(Lexer::Token::Type::Identifier)) {
        std::string name = advance().text;
        if (check(Lexer::Token::Type::LParen)) {
            advance();
            auto node = std::make_shared<Expr>();
            node->kind = Expr::Kind::Call;
            node->name = name;
            node->args = parse_arguments();
            return node;
        }
        auto node = std::make_shared<Expr>();
        node->kind = Expr::Kind::Variable;
        node->name = name;
        return node;
    }
    if (match(Lexer::Token::Type::LParen)) {
        auto expr = parse_expression();
        consume(Lexer::Token::Type::RParen, "Expected ')' after expression");
        auto node = std::make_shared<Expr>();
        node->kind = Expr::Kind::Group;
        node->left = expr;
        return node;
    }
    throw std::runtime_error("Unexpected token while parsing expression");
}

std::vector<ExprPtr> Parser::parse_arguments() {
    std::vector<ExprPtr> args;
    if (check(Lexer::Token::Type::RParen)) {
        advance();
        return args;
    }
    while (true) {
        args.push_back(parse_expression());
        if (!match(Lexer::Token::Type::Comma)) break;
    }
    consume(Lexer::Token::Type::RParen, "Expected ')' after arguments");
    return args;
}

Value Interpreter::evaluate(const ExprPtr& expr) {
    if (!expr) return Value::make_null();
    switch (expr->kind) {
        case Expr::Kind::Number: return Value::make_number(expr->number);
        case Expr::Kind::String: return Value::make_string(expr->text);
        case Expr::Kind::Bool: return Value::make_bool(expr->boolean);
        case Expr::Kind::Variable: return get_variable(expr->name);
        case Expr::Kind::Unary:
            if (expr->op == "-") {
                Value v = evaluate(expr->right);
                if (v.type != Value::Type::Number) throw std::runtime_error("Unary minus requires a number");
                return Value::make_number(-v.number);
            }
            throw std::runtime_error("Unsupported unary operator");
        case Expr::Kind::Binary: {
            if (expr->op == "and") {
                bool left = evaluate(expr->left).is_truthy();
                return Value::make_bool(left && evaluate(expr->right).is_truthy());
            }
            if (expr->op == "or") {
                bool left = evaluate(expr->left).is_truthy();
                return Value::make_bool(left || evaluate(expr->right).is_truthy());
            }
            Value left = evaluate(expr->left);
            Value right = evaluate(expr->right);
            return make_binary_result(expr->op, left, right);
        }
        case Expr::Kind::Call: return evaluate_call(expr->name, expr->args);
        case Expr::Kind::Group: return evaluate(expr->left);
    }
    return Value::make_null();
}

Value Interpreter::evaluate_call(const std::string& name, const std::vector<ExprPtr>& args) {
    if (functions_.count(name) == 0) {
        throw std::runtime_error("Undefined function: " + name);
    }
    const auto& function = functions_.at(name);
    if (function->params.size() != args.size()) {
        throw std::runtime_error("Wrong argument count for function: " + name);
    }
    scopes_.emplace_back();
    for (std::size_t i = 0; i < function->params.size(); ++i) {
        set_variable(function->params[i], evaluate(args[i]));
    }
    return_value_.reset();
    execute_block(function->body, false);
    Value result = return_value_.value_or(Value::make_null());
    scopes_.pop_back();
    return result;
}

void Interpreter::execute(const StmtPtr& stmt) {
    if (!stmt) return;
    switch (stmt->kind) {
        case Stmt::Kind::Program:
            for (const auto& item : stmt->statements) execute(item);
            break;
        case Stmt::Kind::Block:
            execute_block(stmt->body, true);
            break;
        case Stmt::Kind::Assign:
            set_variable(stmt->name, evaluate(stmt->expr));
            break;
        case Stmt::Kind::If: {
            if (evaluate(stmt->condition).is_truthy()) {
                execute_block(stmt->body, true);
            } else if (!stmt->else_body.empty()) {
                execute_block(stmt->else_body, true);
            }
            break;
        }
        case Stmt::Kind::Return:
            return_value_ = evaluate(stmt->expr);
            break;
        case Stmt::Kind::Print:
            std::cout << evaluate(stmt->expr).to_string() << std::endl;
            break;
        case Stmt::Kind::Function:
            functions_[stmt->name] = std::make_shared<FunctionDef>(FunctionDef{stmt->params, stmt->body});
            break;
        case Stmt::Kind::ExprStmt:
            evaluate(stmt->expr);
            break;
    }
}

void Interpreter::execute_block(const std::vector<StmtPtr>& statements, bool new_scope) {
    if (new_scope) scopes_.emplace_back();
    for (const auto& stmt : statements) {
        execute(stmt);
        if (return_value_.has_value()) break;
    }
    if (new_scope) scopes_.pop_back();
}

void Interpreter::set_variable(const std::string& name, const Value& value) {
    if (scopes_.empty()) {
        scopes_.emplace_back();
    }
    scopes_.back()[name] = value;
}

Value Interpreter::get_variable(const std::string& name) const {
    for (auto it = scopes_.rbegin(); it != scopes_.rend(); ++it) {
        auto found = it->find(name);
        if (found != it->end()) return found->second;
    }
    throw std::runtime_error("Undefined variable: " + name);
}

void Interpreter::run_program(const StmtPtr& program) {
    scopes_.clear();
    scopes_.emplace_back();
    execute(program);
}

void Interpreter::run_file(const std::string& path) {
    Lexer lexer(read_file(path));
    Parser parser(std::move(lexer));
    auto program = parser.parse_program();
    Interpreter interpreter;
    interpreter.run_program(program);
}

void run_file(const std::string& path) {
    Interpreter::run_file(path);
}

}  // namespace onfex
