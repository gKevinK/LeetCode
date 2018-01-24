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
    TreeNode* b(vector<int>& in, int inl, int inr, vector<int>& post, int pol, int por) {
        if (inl == inr) return NULL;
        TreeNode * r = new TreeNode(post[por-1]);
        if (inl + 1 == inr) return r;
        int i;
        for (i = inr - inl; post[por - 1] != in[inl + i]; i--);
        r->left = b(in, inl, inl + i, post, pol, pol + i);
        r->right = b(in, inl + i + 1, inr, post, pol + i, por - 1);
        return r;
    }
public:
    TreeNode* buildTree(vector<int>& inorder, vector<int>& postorder) {
        return b(inorder, 0, inorder.size(), postorder, 0, postorder.size());
    }
};