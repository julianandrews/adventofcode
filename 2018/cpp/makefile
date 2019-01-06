CXX = /usr/bin/clang++
CXXFLAGS ?= -Wall -Werror -O3 -std=c++14

%: %.cc
	mkdir -p bin/
	$(CXX) $(CXXFLAGS) $< -o bin/$@
