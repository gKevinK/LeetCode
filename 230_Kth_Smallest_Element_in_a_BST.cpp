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
    int f(TreeNode * r, int & k, int & c) {
        if (r == NULL) return 0;
        if (r->left) {
            int t = f(r->left, k, c);
            if (k == c) return t;
        }
        c++;
        if (k == c) return r->val;
        if (r->right) {
            int t = f(r->right, k, c);
            if (k == c) return t;
        }
        return 0;
    }
public:
    int kthSmallest(TreeNode* root, int k) {
        int c = 0;
        return f(root, k, c);
    }
};