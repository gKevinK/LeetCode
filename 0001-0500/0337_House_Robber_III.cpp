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
    int max(int a, int b) {
        return (a > b)? a: b;
    }

    void A(TreeNode* r, int* a, int* b) {
        int a1 = 0, a2 = 0, b1 = 0, b2 = 0;

        if (r -> left == NULL && r -> right == NULL) {
            *a = r -> val;
            *b = 0;
            return;
        }
        if (r -> left != NULL) {
            A(r -> left, &a1, &a2);
        }
        if (r -> right != NULL) {
            A(r ->right, &b1, &b2);
        }
        *a = r -> val + a2 + b2;
        *b = max(a1, a2) + max(b1, b2);
    }
    
    int rob(TreeNode* root) {
        int a = 0, b = 0;
        if (root != NULL) A(root, &a, &b);
        return (a > b)? a: b;
    }
};