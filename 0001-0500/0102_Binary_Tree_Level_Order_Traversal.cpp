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
    vector<vector<int>> levelOrder(TreeNode* root) {
        vector<TreeNode*> r1, r2;
        vector<vector<int>> r;
        r1.push_back(root);
        while (!r1.empty()) {
            vector<int> v;
            for (TreeNode * p : r1) {
                if (p != NULL) {
                    v.push_back(p->val);
                    r2.push_back(p->left);
                    r2.push_back(p->right);
                }
            }
            if (!v.empty())
                r.push_back(v);
            r1.clear();
            r1.swap(r2);
        }
        return r;
    }
};