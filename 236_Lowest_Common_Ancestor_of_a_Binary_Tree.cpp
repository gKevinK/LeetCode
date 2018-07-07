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
    vector<TreeNode*> vp, vq;
    void s(TreeNode* r, TreeNode* p, TreeNode* q, vector<TreeNode*>& v) {
        if (r == NULL) return;
        v.push_back(r);
        if (r == p) vp = v;
        if (r == q) vq = v;
        s(r->left, p, q, v);
        s(r->right, p, q, v);
        v.pop_back();
    }
public:
    TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
        vector<TreeNode*> v;
        s(root, p, q, v);
        for (int i = 0; i < min(vp.size(), vq.size()); i++)
            if (i + 1 == vp.size() || i + 1 == vq.size() || vp[i + 1] != vq[i + 1])
                return vp[i];
        return NULL;
    }
};