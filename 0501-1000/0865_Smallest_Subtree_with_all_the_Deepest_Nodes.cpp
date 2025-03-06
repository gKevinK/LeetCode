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
    int height(TreeNode* r) {
        if (r == NULL) return 0;
        return max(height(r->left), height(r->right)) + 1;
    }
public:
    TreeNode* subtreeWithAllDeepest(TreeNode* root) {
        TreeNode * r = root;
        while (r) {
            int hl = height(r->left), hr = height(r->right);
            if (hl == hr) return r;
            r = (hl > hr) ? r->left : r->right;
        }
        return NULL;
    }
};