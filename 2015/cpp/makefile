# examples:
# 	make day01
# 	make day01 test
# 	make day01 run
# 	make day01 day02 run
# 	make clean

CXX ?= /usr/bin/clang++
CXXFLAGS ?= -lssl -lcrypto -Wall -Werror -O3 -std=c++14
INPUT_PATH ?= ../inputs

%: %.cc | bin
	$(CXX) $(CXXFLAGS) $< -o bin/$@

bin:
	mkdir bin

run: $(filter-out run, $(MAKECMDGOALS))
	@for i in $^; do \
		echo "\n$$i\n====="; \
		./bin/$$i <$(INPUT_PATH)/$$i/input.txt; \
	done

test: $(filter-out test, $(MAKECMDGOALS))
	@for i in $^; do \
		echo "\n$$i\n====="; \
		for test_file in $(INPUT_PATH)/$$i/sample*.txt; do \
			echo "$$test_file"; \
			./bin/$$i <"$$test_file"; \
		done \
	done

clean:
	rm -f bin/*
	rmdir bin

.PHONY: clean
