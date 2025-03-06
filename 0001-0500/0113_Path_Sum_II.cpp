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
    void f(TreeNode* r, int sum, vector<vector<int>>& m, vector<int>& s) {
        if (r == NULL) return;
        // if (sum == 0)
        if (r->left == NULL && r->right == NULL && sum == r->val) {
            s.push_back(r->val);
            m.push_back(s);
            s.pop_back();
        }
        s.push_back(r->val);
        f(r->left, sum - r->val, m, s);
        f(r->right, sum - r->val, m, s);
        s.pop_back();
    }
public:
    vector<vector<int>> pathSum(TreeNode* root, int sum) {
        vector<vector<int>> m;
        vector<int> s;
        f(root, sum, m, s);
        return m;
    }
};