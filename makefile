

RUSTBIND = -I./rcalc/target/cxxbridge/rcalc/src/ -I./rcalc/target/cxxbridge/
RUSTBINDLIB = -L rcalc/target/release -l rcalc 

all: 
	make -C rcalc
	g++ -std=c++17 calc.cpp -o calc $(RUSTBIND) $(RUSTBINDLIB)

clean: 
	rm calc
	make -C rcalc clean
