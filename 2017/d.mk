.PHONY: clean-d

d: d.d
	ldc2 -O2 -of=$@ $<
