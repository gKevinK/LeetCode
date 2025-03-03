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
    bool f(TreeNode* r, int v) {
        if (!r) return true;
        if (r->val != v) return false;
        return f(r->left, v) && f(r->right, v);
    }
public:
    bool isUnivalTree(TreeNode* root) {
        return f(root->left, root->val) && f(root->right, root->val);
    }
};