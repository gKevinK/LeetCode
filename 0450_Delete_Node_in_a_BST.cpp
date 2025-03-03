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
    TreeNode* deleteNode(TreeNode* root, int key) {
        if (root == nullptr)
            return nullptr;
        else if (root->val == key) {
            if (root->left && root->right) {
                TreeNode * t = root->left;
                while (t->right)
                    t = t->right;
                root->val = t->val;
                root->left = deleteNode(root->left, root->val);
            } else
                return root->left ? root->left : root->right;
        } else if (root->val > key) {
            root->left = deleteNode(root->left, key);
        } else {
            root->right = deleteNode(root->right, key);
        }
        return root;
    }
};