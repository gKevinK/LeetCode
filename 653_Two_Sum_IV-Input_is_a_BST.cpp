/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
class Solution {
    bool f(TreeNode* r, unordered_set<int>& s, int k) {
        if (r == NULL) return false;
        if (s.find(k - r->val) != s.end()) return true;
        s.insert(r->val);
        return f(r->left, s, k) || f(r->right, s, k);
    }
public:
    bool findTarget(TreeNode* root, int k) {
        unordered_set<int> s;
        return f(root, s, k);
    }
};