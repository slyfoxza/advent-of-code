CPPFLAGS += -DUSE_OPENSSL
CXXFLAGS += -std=c++11 -O2
LDFLAGS += -pthread
LDADD += $(shell pkg-config --libs openssl)

.PHONY: all clean

all: day5

day5: main.o md5-openssl.o
	$(CXX) $(LDFLAGS) -o $@ $+ $(LDADD)

clean:
	$(RM) day5 *.o
