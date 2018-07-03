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
    bool find(TreeNode* r, TreeNode* t, vector<TreeNode*>& v) {
        if (r == NULL) return false;
        v.push_back(r);
        if (r == t) return true;
        if (find(r->left, t, v)) return true;
        if (find(r->right, t, v)) return true;
        v.pop_back();
        return false;
    }
    void add(TreeNode* t, int k, vector<int>& r, TreeNode* b = NULL) {
        if (t == NULL) return;
        if (k == 0) {
            r.push_back(t->val);
            return;
        } else {
            if (t->left != b) add(t->left, k - 1, r);
            if (t->right != b)add(t->right, k - 1, r);
        }
    }
public:
    vector<int> distanceK(TreeNode* root, TreeNode* target, int K) {
        vector<TreeNode*> v;
        vector<int> r;
        TreeNode* l = NULL;
        find(root, target, v);
        for (int i = K; !v.empty() && i >= 0; i--) {
            add(v.back(), i, r, l);
            l = v.back();
            v.pop_back();
        }
        return r;
    }
};