#include <iostream>
#include "test2.cpp"
using namespace std;

void spellOut(const string& text) {
    for (char c : text) {
        cout << c << " ";
    }
    cout << endl;
}
int main() {
    Tank myTank(Tier::VII, "Tiger I", TankType::HEAVY, TankNation::GERMANY);
    myTank.info();
    cout << (5 + 10) << pluse(5, 10) << endl;
    string text = "Hello, World!";
    spellOut(text);
    return 0;
}