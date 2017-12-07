#include <iostream>
#include <regex>
#include <string>
#include <unordered_map>
#include <vector>

struct node {
	std::string name;
	int weight = 0;
	node *parent = nullptr;
	std::vector<node*> children = {};
};

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
	std::cout << root->name << '\n';
	return 0;
}
