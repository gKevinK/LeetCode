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
public:
    int rangeSumBST(TreeNode* root, int L, int R) {
        int r = 0;
        if (root == nullptr) return 0;
        if (root->val >= L && root->val <= R)
            r += root->val;
        if (root->val < R)
            r += rangeSumBST(root->right, L, R);
        if (root->val > L)
            r += rangeSumBST(root->left, L, R);
        return r;
    }
};