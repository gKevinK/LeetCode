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
    void f(TreeNode* r, int x, int y, map<int, map<int, set<int>>>& m) {
        if (r == nullptr) return;
        m[x][y].insert(r->val);
        f(r->left, x - 1, y + 1, m);
        f(r->right, x + 1, y + 1, m);
    }
public:
    vector<vector<int>> verticalTraversal(TreeNode* root) {
        map<int, map<int, set<int>>> m;
        f(root, 0, 0, m);
        vector<vector<int>> res;
        for (auto & p : m) {
            res.push_back({});
            for (auto & p2 : p.second) for (auto & i : p2.second)
                res.back().push_back(i);
        }
        return res;
    }
};