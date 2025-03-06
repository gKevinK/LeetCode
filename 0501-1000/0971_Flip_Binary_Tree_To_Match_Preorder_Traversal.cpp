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
    bool f(TreeNode* root, vector<int>& v, int i1, int i2, vector<int>& res) {
        if (root == nullptr && i1 == i2) return true;
        if (root == nullptr || i1 == i2 || root->val != v[i1]) return false;
        if (root->left == nullptr || root->right == nullptr)
            return f(root->left ? root->left : root->right, v, i1 + 1, i2, res);
        if (v[i1 + 1] != root->left->val) {
            int m1 = i1 + 2;
            for (; m1 < i2; m1++)
                if (v[m1] == root->left->val)
                    break;
            if (m1 == i2) return false;
            res.push_back(root->val);
            return f(root->left, v, m1, i2, res) && f(root->right, v, i1 + 1, m1, res);
        } else {
            int m2 = i1 + 2;
            for (; m2 < i2; m2++)
                if (v[m2] == root->right->val)
                    break;
            if (m2 == i2) return false;
            return f(root->left, v, i1 + 1, m2, res) && f(root->right, v, m2, i2, res);
        }
    }
public:
    vector<int> flipMatchVoyage(TreeNode* root, vector<int>& voyage) {
        vector<int> res;
        if (!f(root, voyage, 0, voyage.size(), res))
            res = { -1 };
        return res;
    }
};