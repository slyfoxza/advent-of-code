#pragma once
#include <array>
#include <string>

#include <Windows.h>
#include <BCrypt.h>

namespace aoc201605 {
	class Md5GlobalState {
		public:
		Md5GlobalState();
		Md5GlobalState(const Md5GlobalState&) = delete;
		Md5GlobalState(Md5GlobalState&&) = delete;
		~Md5GlobalState();
		Md5GlobalState& operator=(const Md5GlobalState&) = delete;
		Md5GlobalState& operator=(Md5GlobalState&&) = delete;

		operator BCRYPT_ALG_HANDLE() const { return handle_; }

		private:
		BCRYPT_ALG_HANDLE handle_;
	};

	class Md5 {
		public:
		Md5(const Md5GlobalState& globalState);
		Md5(const Md5&) = delete;
		Md5(Md5&&) = delete;
		~Md5();
		Md5& operator=(const Md5&) = delete;
		Md5& operator=(Md5&&) = delete;

		void hash(const std::string& data);
		void finish(std::array<unsigned char, 16>& output);

		private:
		BCRYPT_HASH_HANDLE handle_;
	};
}
