#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <openssl/evp.h>

int main() {
	EVP_MD_CTX *ctx = EVP_MD_CTX_create();

	// 8-byte input + room for 1 digit + \0
	int size = 10;
	char *input = malloc(size);
	fgets(input, 9, stdin);

	unsigned char mask = 0xF0;
	unsigned char hash[16];
	for(int i = 1;; ++i) {
		int n = snprintf(input + 8, size - 8, "%d", i);
		if(n >= size - 8) {
			// Not enough space in input buffer. Resize it.
			size = n + 9;
			input = realloc(input, size);
			// Cancel the effect of the loop increment.
			--i;
			continue;
		}

		EVP_DigestInit_ex(ctx, EVP_md5(), NULL);
		EVP_DigestUpdate(ctx, input, size - 1);
		EVP_DigestFinal_ex(ctx, hash, NULL);
		short first16 = *((short*)hash);
		if((first16 == 0) && ((hash[2] & mask) == 0)) {
			printf("%d\n", i);
			if(mask == 0xF0) {
				mask = 0xFF;
			} else {
				break;
			}
		}
	}
	return 0;
}
