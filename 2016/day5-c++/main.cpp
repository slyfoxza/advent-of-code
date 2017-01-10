#include <array>
#include <iostream>
#include <string>

#include "md5.h"

int main(int argc, char* argv[]) {
	aoc201605::Md5GlobalState md5GlobalState;
	aoc201605::Md5 md5(md5GlobalState);
	std::array<unsigned char, 16> hash;
	md5.hash(std::string("The quick brown fox jumps over the lazy dog"));
	md5.finish(hash);
	std::array<unsigned char, 16> TQBF_HASH { 0x9e, 0x10, 0x7d, 0x9d, 0x37,
			0x2b, 0xb6, 0x82, 0x6b, 0xd8, 0x1d, 0x35, 0x42, 0xa4, 0x19, 0xd6 };
	if(hash != TQBF_HASH) {
		std::cerr << "Hash of TQBF string did not match expected value\n";
		return 1;
	}
	md5.finish(hash);
	std::array<unsigned char, 16> NULL_HASH { 0xd4, 0x1d, 0x8c, 0xd9, 0x8f,
			0x00, 0xb2, 0x04, 0xe9, 0x80, 0x09, 0x98, 0xec, 0xf8, 0x42, 0x7e };
	if(hash != NULL_HASH) {
		std::cerr << "Hash of empty string did not match expected value\n";
		return 1;
	}
	return 0;
}
