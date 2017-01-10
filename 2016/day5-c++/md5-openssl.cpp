#include <stdexcept>

#include "md5-openssl.h"

using namespace aoc201605;

namespace {
	void initializeMd5Digest(EVP_MD_CTX* context) {
		if(EVP_DigestInit_ex(context, EVP_md5(), nullptr) == 0) {
			throw std::runtime_error("Failed to initialize MD5 digest");
		}
	}
}

Md5::Md5(const Md5GlobalState&)
		: context_(EVP_MD_CTX_create(), EVP_MD_CTX_destroy) {
	if(context_ == nullptr) {
		throw std::runtime_error("EVP_MD_CTX_create() == nullptr");
	}
	initializeMd5Digest(context_.get());
}

void Md5::hash(const std::string& data) {
	if(EVP_DigestUpdate(context_.get(), data.data(), data.size()) == 0) {
		throw std::runtime_error("Failed updating digest with data");
	}
}

void Md5::finish(std::array<unsigned char, 16>& output) {
	if(EVP_DigestFinal_ex(context_.get(), output.data(), nullptr) == 0) {
		throw std::runtime_error("Failed to finalize digest");
	}
	initializeMd5Digest(context_.get());
}
