.PHONY: clean-c++

c++: c++.c++
	$(CXX) -Wall -Wextra -O2 -o $@ $<

clean-c++:
	$(RM) c++
