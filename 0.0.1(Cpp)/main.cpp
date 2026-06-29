#include "onfex.hpp"

int main(int argc, char** argv) {
    std::string path = (argc > 1) ? argv[1] : "main.onfex";
    onfex::run_file(path);
    return 0;
}
