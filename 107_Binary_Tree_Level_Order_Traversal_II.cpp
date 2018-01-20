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
    void f(TreeNode* r, vector<vector<int>> & v, int l) {
        if (r == NULL) return;
        if (v.size() == l) v.push_back(vector<int>());
        v[l].push_back(r->val);
        f(r->left, v, l + 1);
        f(r->right, v, l + 1);
    }
public:
    vector<vector<int>> levelOrderBottom(TreeNode* root) {
        vector<vector<int>> v;
        f(root, v, 0);
        return vector<vector<int>>(v.rbegin(), v.rend());
    }
};