CXXFLAGS ?= -march=native -O3 -Wall -Wextra
CLEAN_CPP := clean-cpp

cpp: cpp.cpp

clean-cpp:
	$(RM) cpp
