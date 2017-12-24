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
    void p(TreeNode * r, vector<int> & v, int d) {
        if (r == NULL) return;
        if (v.size() <= d) v.push_back(r->val);
        else v[d] = r->val;
        p(r->left, v, d+1);
        p(r->right, v, d+1);
    }
public:
    vector<int> rightSideView(TreeNode* root) {
        vector<int> v;
        p(root, v, 0);
        return v;
    }
};