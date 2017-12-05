#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
	int *offsets = NULL, nOffsets = 0, szOffsets = 0;
	int tmp;
	while(scanf("%d\n", &tmp) == 1) {
		if(nOffsets == szOffsets) {
			szOffsets += 1100;
			offsets = realloc(offsets, szOffsets * sizeof(*offsets));
		}
		offsets[nOffsets++] = tmp;
	}
	int *offsetsCopy = malloc(nOffsets * sizeof(*offsets));
	memcpy(offsetsCopy, offsets, nOffsets * sizeof(*offsets));

	int steps1 = 0;
	int i = 0;
	while((i >= 0) && (i < nOffsets)) {
		i += offsets[i]++;
		++steps1;
	}

	offsets = offsetsCopy;
	int steps2 = 0;
	i = 0;
	while((i >= 0) && (i < nOffsets)) {
		tmp = offsets[i];
		offsets[i] += (tmp >= 3) ? -1 : 1;
		i += tmp;
		++steps2;
	}

	printf("%d %d\n", steps1, steps2);

	return 0;
}
