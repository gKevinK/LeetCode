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
    int f(TreeNode* r, int & s) {
        if (r == NULL) return 0;
        int t = 0, s1 = 0, s2 = 0;
        t += f(r->left, s1);
        t += f(r->right, s2);
        t += abs(s1 - s2);
        s = r->val + s1 + s2;
        return t;
    }
public:
    int findTilt(TreeNode* root) {
        int s = 0;
        return f(root, s);
    }
};