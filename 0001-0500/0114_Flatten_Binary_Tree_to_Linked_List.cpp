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
    TreeNode * f(TreeNode* r) {
        if (r == NULL) return NULL;
        TreeNode* c = r;
        if (r->left) {
            c = f(r->left);
            c->right = r->right;
        } else
            r->left = r->right;
        if (r->right) {
            c = f(r->right);
        }
        r->right = r->left;
        r->left = NULL;
        return c;
    }
public:
    void flatten(TreeNode* root) {
        f(root);
    }
};