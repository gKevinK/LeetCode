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
    int findSecondMinimumValue(TreeNode* root) {
        if (root == NULL || root->left == NULL) return -1;
        int la = root->left->val, lb = findSecondMinimumValue(root->left);
        int ra = root->right->val, rb = findSecondMinimumValue(root->right);
        if (la < ra) return lb == -1 ? ra : min(ra, lb);
        else if (la > ra) return rb == -1 ? la : min(la, rb);
        else if (lb != -1 && rb != -1) return min(lb, rb);
        else if (lb == -1) return rb;
        else return lb;
    }
};