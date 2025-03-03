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
    TreeNode *n1, *n2, *last;
    void t(TreeNode* r) {
        if (r == NULL) return;
        t(r->left);
        if (n1 == NULL && last && last->val >= r->val)
            n1 = last;
        if (n1 && last->val >= r->val)
            n2 = r;
        last = r;
        t(r->right);
    }
public:
    void recoverTree(TreeNode* root) {
        t(root);
        swap(n1->val, n2->val);
    }
};