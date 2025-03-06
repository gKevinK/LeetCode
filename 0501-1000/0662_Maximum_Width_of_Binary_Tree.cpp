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
    int widthOfBinaryTree(TreeNode* root) {
        if (root == nullptr) return 0;
        vector<pair<TreeNode*, int>> v1 = { { root, 0 } }, v2;
        int r = 0;
        while (!v1.empty()) {
            r = max(r, v1.back().second - v1.front().second + 1);
            int l = -1;
            for (auto & p : v1) {
                if (p.first->left) {
                    v2.push_back({ p.first->left, l == -1 ? 0 : (p.second - l) * 2 });
                    if (l == -1) l = p.second;
                }
                if (p.first->right) {
                    v2.push_back({ p.first->right, l == -1 ? 1 : (p.second - l) * 2 + 1 });
                    if (l == -1) l = p.second;
                }
            }
            v1.clear();
            v1.swap(v2);
        }
        return r;
    }
};