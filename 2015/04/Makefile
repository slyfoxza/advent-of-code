CFLAGS ?= -O2 -Wall -Wextra -pipe
LDLIBS := $(shell pkg-config --libs openssl)

aoc2015_04-c: aoc2015_04.c
	$(CC) $(CPPFLAGS) $(CFLAGS) $(LDFLAGS) $< -o $@ $(LDLIBS)
