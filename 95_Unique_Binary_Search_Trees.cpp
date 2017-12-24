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
    vector<TreeNode*> g(int l, int r) {
        vector<TreeNode*> v;
        if (l > r) return v;
        if (l == r) {
            v.push_back(new TreeNode(l)); return v;
        }
        for (int i = l; i <= r; i++) {
            vector<TreeNode*> v1, v2;
            v1 = g(l, i - 1);
            v2 = g(i + 1, r);
            if (v1.size() == 0) v1.push_back(NULL);
            if (v2.size() == 0) v2.push_back(NULL);
            for (auto & p1 : v1) {
                for (auto & p2 : v2) {
                    TreeNode* tn = new TreeNode(i);
                    tn->left = p1;
                    tn->right = p2;
                    v.push_back(tn);
                }
            }
        }
        return v;
    }
public:
    vector<TreeNode*> generateTrees(int n) {
        return g(1, n);
    }
};