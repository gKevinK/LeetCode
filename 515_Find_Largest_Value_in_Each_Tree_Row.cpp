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
    vector<int> largestValues(TreeNode* root) {
        vector<int> r;
        vector<TreeNode*> row1, row2;
        if (root) row1.push_back(root);
        while (!row1.empty()) {
            int m = INT_MIN;
            for (TreeNode * p : row1) {
                m = max(m, p->val);
                if (p->left) row2.push_back(p->left);
                if (p->right) row2.push_back(p->right);
            }
            r.push_back(m);
            row1.clear();
            row1.swap(row2);
        }
        return r;
    }
};