#pragma once
#include <array>
#include <memory>
#include <string>

#include <openssl/evp.h>

namespace aoc201605 {
	/* OpenSSL only has local state. */
	class Md5GlobalState {};
	class Md5 {
		public:
		Md5(const Md5GlobalState&);

		void hash(const std::string& data);
		void finish(std::array<unsigned char, 16>& output);

		private:
		const std::unique_ptr<EVP_MD_CTX, void(*)(EVP_MD_CTX*)> context_;
	};
}
