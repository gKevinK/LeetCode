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
    TreeNode* insertIntoMaxTree(TreeNode* root, int val) {
        if (root == nullptr || val > root->val) {
            auto r = new TreeNode(val);
            r->left = root;
            return r;
        } else {
            root->right = insertIntoMaxTree(root->right, val);
            return root;
        }
    }
};