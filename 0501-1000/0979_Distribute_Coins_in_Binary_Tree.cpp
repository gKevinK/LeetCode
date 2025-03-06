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
    pair<int, int> f(TreeNode* r) {
        if (r == nullptr) return { 0, 0 };
        auto p1 = f(r->left), p2 = f(r->right);
        int t = r->val - 1 + p1.first + p2.first;
        return { t, p1.second + p2.second + abs(t) };
    }
public:
    int distributeCoins(TreeNode* root) {
        return f(root).second;
    }
};