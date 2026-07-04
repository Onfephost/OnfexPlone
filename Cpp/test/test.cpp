#include <iostream>
#include "lexer.cpp"
using namespace std;

void spellOut(const string& text) {
    for (char c : text) {
        cout << c << " ";
    }
    cout << endl;
}
int main() {
    return 0;
}