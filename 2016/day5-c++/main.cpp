#include <array>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <vector>

#include <Windows.h>
#include <BCrypt.h>

namespace {
	void checkNtStatus(const NTSTATUS rc, const std::string& msg) {
		if(rc >= 0x80000000) {
			std::ostringstream ss(msg);
			ss << rc;
			throw std::runtime_error(ss.str());
		}
	}

	class BCryptAlgHandle {
		public:
		BCryptAlgHandle(LPCWSTR pszAlgId, LPCWSTR pszImplementation,
			DWORD dwFlags);
		~BCryptAlgHandle() {
			BCryptCloseAlgorithmProvider(handle_, 0);
		}
		operator BCRYPT_ALG_HANDLE() const { return handle_; }

		private:
		BCRYPT_ALG_HANDLE handle_;
	};

	class BCryptHashHandle {
		public:
		BCryptHashHandle(const BCryptAlgHandle& hAlgorithm, DWORD dwFlags);
		~BCryptHashHandle() {
			BCryptDestroyHash(handle_);
		}
		void hashData(PUCHAR pbInput, ULONG cbInput);
		void hashData(const std::string& input) {
			std::vector<unsigned char> v(input.cbegin(), input.cend());
			hashData(v.data(), v.size());
		}
		template<size_t N> void finishHash(
			std::array<unsigned char, N>& output) {
			const NTSTATUS rc = BCryptFinishHash(handle_, output.data(),
				output.size(), 0);
			checkNtStatus(rc, "BCryptFinishHash");
		}

		private:
		BCRYPT_HASH_HANDLE handle_;
	};
}

BCryptAlgHandle::BCryptAlgHandle(LPCWSTR pszAlgId, LPCWSTR pszImplementation,
		DWORD dwFlags) {
	const NTSTATUS rc = BCryptOpenAlgorithmProvider(&handle_, pszAlgId,
		pszImplementation, dwFlags);
	checkNtStatus(rc, "BCryptOpenAlgorithmProvider");
}

BCryptHashHandle::BCryptHashHandle(const BCryptAlgHandle& hAlgorithm,
		DWORD dwFlags) {
	const NTSTATUS rc = BCryptCreateHash(hAlgorithm, &handle_, nullptr, 0,
		nullptr, 0, dwFlags);
	checkNtStatus(rc, "BCryptCreateHash");
}

void BCryptHashHandle::hashData(PUCHAR pbInput, ULONG cbInput) {
	const NTSTATUS rc = BCryptHashData(handle_, pbInput, cbInput, 0);
	checkNtStatus(rc, "BCryptHashData");
}

int main(int argc, char* argv[]) {
	BCryptAlgHandle algHandle(BCRYPT_MD5_ALGORITHM, nullptr,
		BCRYPT_HASH_REUSABLE_FLAG);
	BCryptHashHandle hashHandle(algHandle, BCRYPT_HASH_REUSABLE_FLAG);
	std::array<unsigned char, 16> hash;
	hashHandle.hashData(std::string(
			"The quick brown fox jumps over the lazy dog"));
	hashHandle.finishHash(hash);
	std::array<unsigned char, 16> TQBF_HASH { 0x9e, 0x10, 0x7d, 0x9d, 0x37,
			0x2b, 0xb6, 0x82, 0x6b, 0xd8, 0x1d, 0x35, 0x42, 0xa4, 0x19, 0xd6 };
	if(hash != TQBF_HASH) {
		std::cerr << "Hash of TQBF string did not match expected value\n";
		return 1;
	}
	hashHandle.finishHash(hash);
	std::array<unsigned char, 16> NULL_HASH { 0xd4, 0x1d, 0x8c, 0xd9, 0x8f,
			0x00, 0xb2, 0x04, 0xe9, 0x80, 0x09, 0x98, 0xec, 0xf8, 0x42, 0x7e };
	if(hash != NULL_HASH) {
		std::cerr << "Hash of empty string did not match expected value\n";
		return 1;
	}
	return 0;
}
