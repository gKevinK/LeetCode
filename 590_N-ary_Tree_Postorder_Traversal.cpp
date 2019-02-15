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
        for (Node* c : root->children)
            f(c, result);
        result.push_back(root->val);
    }
public:
    vector<int> postorder(Node* root) {
        vector<int> result;
        f(root, result);
        return result;
    }
};