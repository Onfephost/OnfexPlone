#include <iostream>
using namespace std;

int pluse(int a, int b) {
    return a + b;
}

enum class TankType {
    LIGHT,
    MEDIUM,
    HEAVY
};
enum class TankNation {
    USA,
    GERMANY,
    RUSSIA
};
string tankTypeToString(TankType type) {
    switch (type) {
        case TankType::LIGHT: return "LIGHT";
        case TankType::MEDIUM: return "MEDIUM";
        case TankType::HEAVY: return "HEAVY";
        default: return "UNKNOWN";
    }
}
string tankNationToString(TankNation nation) {
    switch (nation) {
        case TankNation::USA: return "USA";
        case TankNation::GERMANY: return "GERMANY";
        case TankNation::RUSSIA: return "RUSSIA";
        default: return "UNKNOWN";
    }
}
enum class Tier{
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
    VIII,
    IX,
    X
};

class Tank {
public:
    Tier tier;
    string name;
    TankType type;
    TankNation nation;
    Tank(Tier t, string n, TankType ty, TankNation na){
        tier = t;
        name = n;
        type = ty;
        nation = na;
    }
    void info() {
        cout << "Tank Name: " << name << endl;
        cout << "Tier: " << static_cast<int>(tier) + 1 << endl;
        cout << "Type: " << tankTypeToString(type) << endl;
        cout << "Nation: " << tankNationToString(nation) << endl;
    }
};