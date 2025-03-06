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
    int countNodes(TreeNode* root) {
        if (root == NULL) return 0;
        int l = 0, r = 0;
        TreeNode * lt = root, * rt = root;
        while (lt) {
            lt = lt->left; l++;
        }
        while (rt) {
            rt = rt->right; r++;
        }
        return l == r ? pow(2, l) - 1 : countNodes(root->left) + countNodes(root->right) + 1;
    }
};