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
    int getDepth(TreeNode* r) {
        if (r == NULL) return 0;
        int a = getDepth(r -> left);
        if (a == -1) return -1;
        int b = getDepth(r -> right);
        if (b == -1) return -1;
        if (a - b > 1 || b - a > 1) return -1;
        return max(a, b) + 1;
    }

public:
    bool isBalanced(TreeNode* root) {
        return (getDepth(root) != -1);
    }
};