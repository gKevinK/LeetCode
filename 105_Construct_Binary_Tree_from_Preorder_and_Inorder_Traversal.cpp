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
    TreeNode* b(vector<int>& pre, int prel, int prer, vector<int>& in, int inl, int inr) {
        if (prel == prer) return NULL;
        TreeNode * r = new TreeNode(pre[prel]);
        if (prel + 1 == prer) return r;
        int i;
        for (i = 0; pre[prel] != in[inl + i]; i++);
        r->left = b(pre, prel + 1, prel + i + 1, in, inl, inl + i);
        r->right = b(pre, prel + i + 1, prer, in, inl + i + 1, inr);
        return r;
    }
public:
    TreeNode* buildTree(vector<int>& preorder, vector<int>& inorder) {
        return b(preorder, 0, preorder.size(), inorder, 0, inorder.size());
    }
};