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
    void f(TreeNode* t, string& s) {
        s += to_string(t->val);
        if (t->left == nullptr && t->right == nullptr) return;
        s += '(';
        if (t->left) f(t->left, s);
        s += ')';
        if (t->right) {
            s += '(';
            f(t->right, s);
            s += ')';
        }
    }
public:
    string tree2str(TreeNode* t) {
        string s;
        if (t) f(t, s);
        return s;
    }
};