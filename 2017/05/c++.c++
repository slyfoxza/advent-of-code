#include <iostream>
#include <vector>

template<typename F> int jump(std::vector<int> &offsets, const F &f) {
	const size_t s = offsets.size();
	int i, n;
	for(i = 0, n = 0; (i >= 0) && (static_cast<size_t>(i) < s); ++n) {
		f(offsets, i);
	}
	return n;
}

int main() {
	std::vector<int> offsets1;
	offsets1.reserve(1093);
	while(true) {
		int offset;
		std::cin >> offset;
		if(std::cin.eof()) break;
		offsets1.push_back(offset);
	}
	std::vector<int> offsets2(offsets1);

	std::cout << jump(offsets1, [](auto &o, int &i) { i += o[i]++; }) << ' '
		<< jump(offsets2, [](auto &o, int &i) {
			const int tmp = o[i];
			o[i] += (tmp >= 3) ? -1 : 1;
			i += tmp;
		}) << '\n';

	return 0;
}
