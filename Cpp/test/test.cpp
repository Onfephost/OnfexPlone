#include <iostream>
#include "test2.cpp"
using namespace std;
int main() {
    Tank myTank(Tier::VII, "Tiger I", TankType::HEAVY, TankNation::GERMANY);
    myTank.info();
    cout << (5 + 10) << pluse(5, 10) << endl;
    return 0;
}