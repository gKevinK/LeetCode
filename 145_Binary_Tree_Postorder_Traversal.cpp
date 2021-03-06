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
    void f(TreeNode* r, vector<int>& v) {
        if (r == NULL) return;
        f(r->left, v);
        f(r->right, v);
        v.push_back(r->val);
    }
public:
    vector<int> postorderTraversal(TreeNode* root) {
        vector<int> v;
        f(root, v);
        return v;
    }
};