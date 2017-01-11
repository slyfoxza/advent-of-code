#include <sstream>
#include <stdexcept>
#include <string>

#include "md5-cng.h"

using namespace aoc201605;

namespace {
	void checkNtStatus(const NTSTATUS rc, const std::string& msg) {
		if(rc >= 0x80000000) {
			std::ostringstream ss(msg);
			ss << rc;
			throw std::runtime_error(ss.str());
		}
	}
}

Md5GlobalState::Md5GlobalState() {
	const NTSTATUS rc = BCryptOpenAlgorithmProvider(&handle_,
		BCRYPT_MD5_ALGORITHM, nullptr, BCRYPT_HASH_REUSABLE_FLAG);
	checkNtStatus(rc, "BCryptOpenAlgorithmProvider");
}

Md5GlobalState::~Md5GlobalState() {
	BCryptCloseAlgorithmProvider(handle_, 0);
}

Md5::Md5(const Md5GlobalState& globalState) {
	const NTSTATUS rc = BCryptCreateHash(globalState, &handle_, nullptr, 0,
		nullptr, 0, BCRYPT_HASH_REUSABLE_FLAG);
	checkNtStatus(rc, "BCryptCreateHash");
}

Md5::~Md5() {
	BCryptDestroyHash(handle_);
}

void Md5::hash(const std::string& data) const {
	const NTSTATUS rc = BCryptHashData(handle_,
		const_cast<unsigned char*>(reinterpret_cast<const unsigned char*>(
				data.data())), data.size(), 0);
	checkNtStatus(rc, "BCryptHashData");
}

void Md5::finish(std::array<unsigned char, 16>& output) const {
	const NTSTATUS rc = BCryptFinishHash(handle_, output.data(), output.size(),
		0);
	checkNtStatus(rc, "BCryptFinishHash");
}
