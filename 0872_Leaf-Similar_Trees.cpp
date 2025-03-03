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
    void leafs(TreeNode* r, vector<int>& l) {
        if (r == NULL) return;
        if (r->left == NULL && r->right == NULL)
            l.push_back(r->val);
        leafs(r->left, l);
        leafs(r->right, l);
    }
public:
    bool leafSimilar(TreeNode* root1, TreeNode* root2) {
        vector<int> l1, l2;
        leafs(root1, l1);
        leafs(root2, l2);
        int n = l1.size();
        if (n != l2.size()) return false;
        for (int i = 0; i < n; i++)
            if (l1[i] != l2[i]) return false;
        return true;
    }
};