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
    pair<int, int> f(TreeNode* r) {
        if (r == nullptr) return { 0, 0 };
        pair<int, int> p = { 0, r->val };
        if (r->left) {
            auto p1 = f(r->left);
            if (p1.first > p.first)
                p = p1;
        }
        if (r->right) {
            auto p2 = f(r->right);
            if (p2.first > p.first)
                p = p2;
        }
        p.first++;
        return p;
    }
    
public:
    int findBottomLeftValue(TreeNode* root) {
        return f(root).second;
    }
};