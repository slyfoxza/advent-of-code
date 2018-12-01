.PHONY: clean

CFLAGS ?= -O2

c: c.c

c.c: c.lit
	lit -t $<
	lit -w --out-dir ../../docs/2015/01 $<

clean:; $(RM) c c.c
