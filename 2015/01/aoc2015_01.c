#include <stdio.h>

int main() {
	int accum = 0, c, i = 0, negind = -1;
	while((c = getchar()) != EOF) {
		if(c == '(') accum += 1;
		else if(c == ')') accum -= 1;
		if(++i && (accum < 0) && (negind == -1)) negind = i;
	}
	printf("%d %d\n", accum, negind);
	return 0;
}
