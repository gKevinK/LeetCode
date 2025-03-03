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
    void f(TreeNode* r, vector<TreeNode*>& v) {
        if (r == NULL) return;
        f(r->left, v);
        v.push_back(r);
        f(r->right, v);
    }
public:
    TreeNode* increasingBST(TreeNode* root) {
        if (root == NULL) return NULL;
        vector<TreeNode*> v;
        f(root, v);
        for (int i = 0; i < v.size(); i++) {
            v[i]->left = NULL;
            v[i]->right = (i + 1 < v.size()) ? v[i + 1] : NULL;
        }
        return v.front();
    }
};