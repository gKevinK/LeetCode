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
    unordered_map<TreeNode*, TreeNode*> m1;
    unordered_map<string, TreeNode*> m2;
    unordered_set<TreeNode*> res;
    
    string key(int v, TreeNode* l, TreeNode* r) {
        string k(4 + 8 * 2, 0);
        char * p = &k[0];
        *reinterpret_cast<TreeNode**>(p) = l;
        *reinterpret_cast<TreeNode**>(p + 8) = r;
        *reinterpret_cast<int*>(p + 12) = v;
        return k;
    }

    void f(TreeNode* r) {
        if (r == nullptr) return;
        f(r->left);
        f(r->right);
        string k = key(r->val, m1[r->left], m1[r->right]);
        if (m2.find(k) != m2.end()) {
            res.insert(m2[k]);
            m1[r] = m2[k];
        } else {
            m1[r] = r;
            m2[k] = r;
        }
    }
public:
    vector<TreeNode*> findDuplicateSubtrees(TreeNode* root) {
        m1.clear();
        m2.clear();
        res.clear();
        f(root);
        return vector<TreeNode*>(res.begin(), res.end());
    }
};