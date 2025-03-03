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
    void f(Node* root, vector<int>& result) {
        if (root == nullptr) return;
        result.push_back(root->val);
        for (Node* c : root->children)
            f(c, result);
    }
public:
    vector<int> preorder(Node* root) {
        vector<int> result;
        f(root, result);
        return result;
    }
};