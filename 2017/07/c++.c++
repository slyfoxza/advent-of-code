#include <algorithm>
#include <iostream>
#include <numeric>
#include <regex>
#include <string>
#include <unordered_map>
#include <vector>

struct node {
	std::string name;
	int weight = 0;
	int total_weight = 0;
	node *parent = nullptr;
	std::vector<node*> children = {};
};

int balance(node *node) {
	if(node->children.empty()) {
		node->total_weight = node->weight;
		return 0;
	}
	for(auto i = node->children.begin(); i != node->children.end(); ++i) {
		const int b = balance(*i);
		if(b != 0) {
			return b;
		}
	}
	int children_weight = std::accumulate(node->children.cbegin(), node->children.cend(), 0,
			[](int a, auto *n) { return a + n->total_weight; });
	node->total_weight = node->weight + children_weight;
	if(node->children.size() > 2) {
		for(auto i = node->children.begin(); i != node->children.end() - 2; ++i) {
			int a = (*i)->total_weight;
			int b = (*(i + 1))->total_weight;
			int c = (*(i + 2))->total_weight;
			if((a == b) && (b != c)) {
				return (*(i + 2))->weight + (a - b);
			} else if((a == c) && (b != c)) {
				return (*(i + 1))->weight + (a - b);
			} else if((b == c) && (a != b)) {
				return (*i)->weight + (b - a);
			} else {
				continue;
			}
		}
	}
	return 0;
}

int main() {
	std::unordered_map<std::string, node*> node_map;
	std::regex regex("^(\\w+) \\((\\d+)\\)(?: -> ((?:\\w+(?:, )?)+))?");
	std::regex child_regex("(\\w+)(?:, )?");
	for(std::string line; std::getline(std::cin, line);) {
		std::smatch match;
		if(!std::regex_match(line, match, regex)) continue;
		node*& node = node_map[match[1]];
		if(node == nullptr) {
			node = new ::node { match[1] };
		}
		node->weight = std::stoi(match[2]);
		const auto children = match[3].str();
		for(std::sregex_iterator i(children.begin(), children.end(), child_regex);
				i != std::sregex_iterator();
				++i) {
			const auto m = *i;
			::node*& child = node_map[m[1]];
			if(child == nullptr) {
				child = new ::node { m[1] };
			}
			child->parent = node;
			node->children.push_back(child);
		}
	}
	node* root = (*node_map.cbegin()).second;
	while(root->parent != nullptr) root = root->parent;
	std::cout << root->name << ' ' << balance(root) << '\n';
	return 0;
}
