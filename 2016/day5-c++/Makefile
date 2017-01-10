CPPFLAGS += -DUSE_OPENSSL
CXXFLAGS += -g
LDADD += $(shell pkg-config --libs openssl)

.PHONY: all clean

all: day5

day5: main.o md5-openssl.o
	$(CXX) -o $@ $+ $(LDADD)

clean:
	$(RM) day5 *.o
