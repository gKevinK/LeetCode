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
    int getDepth(TreeNode* r) {
        if (r == NULL) return 0;
        return max(getDepth(r->left), getDepth(r->right)) + 1;
    }
    void fill(TreeNode* r, vector<vector<string>>& res, int i, int j, int w) {
        res[i][j] = to_string(r->val);
        if (r->left)  fill(r->left,  res, i + 1, j - w / 2 / 2 - 1, w / 2);
        if (r->right) fill(r->right, res, i + 1, j + w / 2 / 2 + 1, w / 2);
    }
public:
    vector<vector<string>> printTree(TreeNode* root) {
        if (root == NULL) return { { } };
        int depth = getDepth(root), width = 0;
        for (int i = 0; i < depth; i++)
            width = width * 2 + 1;
        vector<vector<string>> res(depth, vector<string>(width, ""));
        fill(root, res, 0, width / 2, width);
        return res;
    }
};