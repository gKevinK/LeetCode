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
    struct R {
        int left;
        int right;
        int res;
    };
    
    R f(TreeNode* r) {
        if (r->left == NULL && r->right == NULL)
            return { r->val, r->val, INT_MAX };
        else if (r->left == NULL) {
            R rr = f(r->right);
            return { r->val, rr.right, min(rr.res, rr.left - r->val) };
        } else if (r->right == NULL) {
            R rl = f(r->left);
            return { rl.left, r->val, min(rl.res, r->val - rl.right) };
        } else {
            R rl = f(r->left);
            R rr = f(r->right);
            return { rl.left, rr.right, min(min(rl.res, r->val - rl.right), min(rr.res, rr.left - r->val)) };
        }
    }
public:
    int minDiffInBST(TreeNode* root) {
        R r = f(root);
        return r.res;
    }
};