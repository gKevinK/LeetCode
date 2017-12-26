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
    vector<vector<int>> zigzagLevelOrder(TreeNode* root) {
        vector<TreeNode*> r1, r2;
        vector<vector<int>> r;
        r1.push_back(root);
        for (int d = 0; !r1.empty(); d++) {
            vector<int> v;
            if (d % 2) {
                for (int i = r1.size() - 1; i >= 0; i--) {
                    if (r1[i] != NULL) {
                        v.push_back(r1[i]->val);
                        r2.push_back(r1[i]->right);
                        r2.push_back(r1[i]->left);
                    }
                }
            } else {
                for (int i = r1.size() - 1; i >= 0; i--) {
                    if (r1[i] != NULL) {
                        v.push_back(r1[i]->val);
                        r2.push_back(r1[i]->left);
                        r2.push_back(r1[i]->right);
                    }
                }
            }
            if (!v.empty())
                r.push_back(std::move(v));
            r1.clear();
            r1.swap(r2);
        }
        return r;
    }
};