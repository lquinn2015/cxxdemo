#include <iostream>
#include "rust/cxx.h"
#include "lib.rs.h"

using namespace rcalc;

int main() {
    auto a = 5;
    auto b = 6;
    auto c = rcalc::grab_calc();
    std::cout << "Adding 6+5=" << rcalc::add(a,b) << std::endl;
    std::cout << "6*6=" << c->mul(6,b) << std::endl;
}
