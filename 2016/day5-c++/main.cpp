#include <array>
#include <chrono>
#include <future>
#include <iostream>
#include <list>
#include <stdexcept>
#include <string>
#include <thread>
#include <vector>

#include "md5.h"

using Md5Hash = std::array<unsigned char, 16>;

namespace {
	void findCandidates(const std::string& prefix, const unsigned start,
			const unsigned end, const aoc201605::Md5GlobalState& md5GlobalState,
			std::promise<std::vector<unsigned>> promise) {
		std::vector<unsigned> candidates;
		const aoc201605::Md5 md5(md5GlobalState);
		Md5Hash hash;
		std::string string(prefix);
		for(unsigned i = start; (i < end) && (i >= start); ++i) {
			string.append(std::to_string(i));
			md5.hash(string);
			md5.finish(hash);
			if((hash[0] == 0x00) && (hash[1] == 0x00)
					&& ((hash[2] & 0xF0) == 0x00)) {
				candidates.push_back(i);
			}
			string = prefix;
		}
		promise.set_value(candidates);
	}

	const char HEX[] = "0123456789abcdef";
}

int main(int argc, char* argv[]) {
	std::string doorId;
	std::cin >> doorId;
	unsigned nThreads = std::thread::hardware_concurrency();
	if(nThreads == 0) {
		nThreads = 4;
	}
	std::cout << doorId << ": Executing with " << nThreads << " thread(s)\n";
	try {
		aoc201605::Md5GlobalState md5GlobalState;
		aoc201605::Md5 md5(md5GlobalState);
		std::string password1, password2(8, '_');
		unsigned char password2Filled = 0;
		std::list<std::future<std::vector<unsigned>>> futures;
		unsigned nextStart = 0;
		auto startTime = std::chrono::steady_clock::now();
		while((password1.size() < 8) || (password2Filled < 8)) {
			while(futures.size() < nThreads) {
				std::promise<std::vector<unsigned>> promise;
				futures.push_back(promise.get_future());
				unsigned end = nextStart + 0x0FFF;
				std::thread thread(findCandidates, doorId, nextStart, end,
					std::ref(md5GlobalState), std::move(promise));
				nextStart = end + 1;
				thread.detach();
			}
			auto future = std::move(futures.front());
			futures.pop_front();
			future.wait();
			auto candidates = future.get();
			Md5Hash hash;
			for(auto candidate: candidates) {
				std::string string(doorId);
				string.append(std::to_string(candidate));
				md5.hash(string);
				md5.finish(hash);
				if(password1.size() < 8) {
					password1.push_back(HEX[hash[2] & 0x0F]);
				}
				unsigned char position = hash[2] & 0x0F;
				if((position > 7) || (password2[position] != '_')) {
					continue;
				}
				password2[position] = HEX[(hash[3] & 0xF0) >> 4];
				if(++password2Filled == 8) {
					break;
				}
			}
		}
		auto endTime = std::chrono::steady_clock::now();
		std::cout << "Password #1: " << password1 << "\nPassword #2: "
			<< password2 << "\nDuration: "
			<< std::chrono::duration_cast<std::chrono::milliseconds>(
				endTime - startTime).count() << "ms\n";
		return 0;
	} catch(const std::exception& e) {
		std::cerr << "Error: " << e.what() << '\n';
		return 1;
	}
}
