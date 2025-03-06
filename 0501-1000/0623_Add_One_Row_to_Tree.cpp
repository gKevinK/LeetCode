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
    void f(TreeNode* r, int v, int d) {
        if (d == 0) {
            auto t = new TreeNode(v);
            t->left = r->left;
            r->left = t;
            t = new TreeNode(v);
            t->right = r->right;
            r->right = t;
        } else {
            if (r->left) f(r->left, v, d - 1);
            if (r->right) f(r->right, v, d - 1);
        }
    }
public:
    TreeNode* addOneRow(TreeNode* root, int v, int d) {
        if (d == 1) {
            auto t = new TreeNode(v);
            t->left = root;
            return t;
        }
        f(root, v, d - 2);
        return root;
    }
};