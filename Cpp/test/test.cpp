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
    Lexer l("a = b");
    l.tokenize();
    return 0;
}