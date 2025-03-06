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
    void f(TreeNode* r, int& s) {
        if (r == NULL) return;
        f(r->right, s);
        s += r->val;
        r->val = s;
        f(r->left, s);
    }
public:
    TreeNode* convertBST(TreeNode* root) {
        int s = 0;
        f(root, s);
        return root;
    }
};