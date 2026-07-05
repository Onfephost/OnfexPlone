#include <iostream>
#include<string>
#include "lexer.cpp"
using namespace std;

void spellOut(const string& text) {
    for (char c : text) {
        cout << c << " ";
    }
    cout << endl;
}
int main() {
    string var = "a = b";
    Lexer l;
    l.tokenize(var);
    cout << "done";
    return 0;
}