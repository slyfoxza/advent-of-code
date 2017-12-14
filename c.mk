CFLAGS ?= -march=native -O3 -Wall -Wextra
CLEAN_C := clean-c

c: c.c

clean-c:
	$(RM) c
