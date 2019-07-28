CXX ?= /usr/bin/clang++
CXXFLAGS ?= -Wall -Werror -O3 -std=c++14
INPUT_PATH ?= ../inputs

%: %.cc | bin
	$(CXX) $(CXXFLAGS) $< -o bin/$@

bin:
	mkdir bin

run: $(filter-out run, $(MAKECMDGOALS))
	@for i in $^; do \
		echo "Running $$i"; \
		./bin/$$i <$(INPUT_PATH)/$$i/input.txt; \
	done

test: $(filter-out test, $(MAKECMDGOALS))
	@for i in $^; do \
		echo "Running $$i"; \
		./bin/$$i <$(INPUT_PATH)/$$i/sample.txt; \
	done

clean:
	rm -f bin/*
	rmdir bin

.PHONY: clean
