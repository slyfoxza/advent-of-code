---
title: Recursive Circus
date: 2017-12-07 16:07:15 -0500
---
# C++
In a fit of madness, I decided to tackle this problem using C++.

[Part 1][cpp1] was relatively simply implemented: I parsed input lines using
some help from `std::regex`, using the node name as the key to a map containing
`node` pointers. Once all nodes have been created and linked to one another, an
arbitrary node is picked out of the map, and the `parent` pointer is traversed
until the root of the tree is reached.

[Part 2][cpp2] implements a recursive solution to find an imbalanced subtree,
and returns the corrected weight. If a subtree is balanced, or a leaf node, it
will return 0.

The recursive algorithm works by calculating the aggregate weight of a node's
children, and adding the node's own weight to determine the total weight of a
subtree. Then, for a given node, the algorithm moves through its children using
a window size of 3, comparing each child subtree's total weight against the
others.

If a mismatch is found, it means the imbalance has been found. The corrected
weight is then a simple modification of the imbalanced child node's weight by
the difference between the equal subtree weights and the mismatched weight.

[cpp1]: https://github.com/slyfoxza/advent-of-code/blob/320c349b8ad30e66b077009f90c378f6ef41db50/2017/07/c%2B%2B.c%2B%2B#L51-L76
[cpp2]: https://github.com/slyfoxza/advent-of-code/blob/320c349b8ad30e66b077009f90c378f6ef41db50/2017/07/c%2B%2B.c%2B%2B#L17-L48
