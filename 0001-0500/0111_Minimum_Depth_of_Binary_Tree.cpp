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
    int minDepth(TreeNode* root) {
        if (root == NULL) return 0;
        vector<TreeNode*> r1, r2;
        r1.push_back(root);
        int d = 0;
        for (; !r1.empty(); d++) {
            for (auto & p : r1) {
                if (p->left == NULL && p->right == NULL) return (d + 1);
                if (p->left) r2.push_back(p->left);
                if (p->right) r2.push_back(p->right);
            }
            r1.clear();
            r1.swap(r2);
        }
        return d;
    }
};