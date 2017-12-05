.PHONY: clean-c

c: c.c
	$(CC) -Wall -Wextra -O2 -o $@ $<

clean-c:
	$(RM) c
