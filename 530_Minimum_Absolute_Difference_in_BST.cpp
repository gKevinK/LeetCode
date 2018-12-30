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
    int last = INT_MAX, diff = INT_MIN;
    
    void f(TreeNode* r) {
        if (r->left)
            f(r->left);
        if (diff == INT_MIN)
            diff = INT_MAX;
        else
            diff = min(diff, r->val - last);
        last = r->val;
        if (r->right)
            f(r->right);
    }
public:
    int getMinimumDifference(TreeNode* root) {
        last = INT_MAX;
        diff = INT_MIN;
        f(root);
        return diff;
    }
};