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
    TreeNode* f(vector<int>& p, int left, int right) {
        if (left >= right) return nullptr;
        TreeNode* r = new TreeNode(p[left]);
        int i = left + 1;
        while (i < right && p[i] < p[left])
            i++;
        r->left = f(p, left + 1, i);
        r->right = f(p, i, right);
        return r;
    }
public:
    TreeNode* bstFromPreorder(vector<int>& preorder) {
        return f(preorder, 0, preorder.size());
    }
};