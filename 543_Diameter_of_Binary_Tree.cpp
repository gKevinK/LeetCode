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
private:
    void c(TreeNode* r, int & d, int & p) {
        if (r == NULL) {
            d = 0; p = 0; return;
        }
        int d1, p1, d2, p2;
        c(r->left, d1, p1);
        c(r->right, d2, p2);
        d = max(d1, d2) + 1;
        p = max(max(p1, p2), d1 + d2);
    }
    
public:
    int diameterOfBinaryTree(TreeNode* root) {
        int d, p;
        c(root, d, p);
        return max(d-1, p);
    }
};