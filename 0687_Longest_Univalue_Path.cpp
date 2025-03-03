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
        auto p1 = f(r->left);
        auto p2 = f(r->right);
        if (r->left == nullptr || r->left->val != r->val)
            p1.second = 0;
        if (r->right == nullptr || r->right->val != r->val)
            p2.second = 0;
        return { max({ p1.first, p2.first, p1.second + p2.second + 1 }),
                 max(p1.second, p2.second) + 1 };
    }
public:
    int longestUnivaluePath(TreeNode* root) {
        auto p = f(root);
        return max(0, p.first - 1);
    }
};