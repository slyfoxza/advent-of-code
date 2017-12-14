DFLAGS ?= -O3
CLEAN_D := clean-d

d: d.d
	ldc2 $(DFLAGS) -of=$@ $<

clean-d:
	$(RM) d d.o
