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

    auto d = rcalc::gCalc(); d.ver = 1;
    rcalc::printVer(d);
    d.data.push_back(2);
    d.hm.push_back("hello");

    std::cout << "I have a c++ " << d.data[0] << std::endl;
    std::cout << "I have a c++ " << d.hm[0] << std::endl;

    
}
