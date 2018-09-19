/*
// Definition for a Node.
class Node {
public:
    int val = NULL;
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
    vector<vector<int>> levelOrder(Node* root) {
        vector<vector<int>> r;
        vector<Node *> v1, v2;
        if (root) v1.push_back(root);
        while (!v1.empty()) {
            r.push_back({});
            for (auto & n : v1) {
                r.back().push_back(n->val);
                for (auto & c : n->children)
                    if (c)
                        v2.push_back(c);
            }
            v1.clear();
            v1.swap(v2);
        }
        return r;
    }
};