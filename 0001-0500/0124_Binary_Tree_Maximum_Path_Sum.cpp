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
    int maxPathSum(TreeNode* root) {
        int a = 0, b = 0;
        calc(root, a, b);
        return max(a, b);
    }

    void calc(TreeNode* r, int& a, int& b) {
        if (r == NULL) {
            a = b = 0;
        } else if (r -> left == NULL && r -> right == NULL) {
            a = b = r -> val;
        } else {
            int a1 = 0, a2 = 0, b1 = 0, b2 = 0;
            calc(r -> left, a1, b1);
            calc(r -> right, a2, b2);
            a = max(max(r -> val + a1, r -> val + a2), r -> val);
            int nb = (a1 > 0 ? a1 : 0) + (a2 > 0 ? a2 : 0) + r -> val;
            if (r -> left == NULL) {
                b = max(b2, nb);
            } else if (r -> right == NULL) {
                b = max(b1, nb);
            } else {
                b = max(max(b1, b2), nb);
            }
        }
    }
};