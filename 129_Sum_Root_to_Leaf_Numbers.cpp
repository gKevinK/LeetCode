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
    void f(TreeNode* r, int c, int & s) {
        if (r == NULL) return;
        int c1 = c * 10 + r->val;
        if (r->left == NULL && r->right == NULL) {
            s += c1;
            return;
        }
        f(r->left, c1, s);
        f(r->right, c1, s);
    }
public:
    int sumNumbers(TreeNode* root) {
        int sum = 0;
        f(root, 0, sum);
        return sum;
    }
};