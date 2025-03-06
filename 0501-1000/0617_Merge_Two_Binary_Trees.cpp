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
private:
    TreeNode* m(TreeNode* t1, TreeNode* t2) {
        if (t1 == NULL) return t2;
        if (t2 == NULL) return t1;
        TreeNode* t = new TreeNode(t1->val + t2->val);
        t->left = m(t1->left, t2->left);
        t->right = m(t1->right, t2->right);
        return t;
    }
public:
    TreeNode* mergeTrees(TreeNode* t1, TreeNode* t2) {
        return m(t1, t2);
    }
};