/*
// Definition for a Node.
class Node {
public:
    int val;
    vector<Node*> children;

    Node() {}

    Node(int _val, vector<Node*> _children) {
        val = _val;
        children = _children;
    }
};
*/
class Solution {
public:
    int maxDepth(Node* root) {
        if (root == nullptr) return 0;
        int m = 0;
        for (auto p : root->children)
            m = max(m, maxDepth(p));
        return m + 1;
    }
};