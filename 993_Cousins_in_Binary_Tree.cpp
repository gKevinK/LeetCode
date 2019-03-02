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
    int dx, dy, px, py, _x, _y;
    void f(TreeNode* r, int d, int p) {
        if (r == nullptr) return;
        if (r->val == _x) {
            dx = d;
            px = p;
        } else if (r->val == _y) {
            dy = d;
            py = p;
        }
        f(r->left, d + 1, r->val);
        f(r->right, d + 1, r->val);
    }
public:
    bool isCousins(TreeNode* root, int x, int y) {
        dx = dy = px = py = -1;
        _x = x;
        _y = y;
        f(root, 0, -1);
        return dx == dy && px != py;
    }
};