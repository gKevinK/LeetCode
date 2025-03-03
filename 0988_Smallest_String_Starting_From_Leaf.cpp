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
    string res, tmp;
    int comp(string & a, string & b) {
        int i = a.size(), j = b.size();
        while (i || j) {
            if (i == 0) return 1;
            if (j == 0) return -1;
            if (a[--i] < b[--j]) return 1;
            if (a[i] > b[j]) return -1;
        }
        return 0;
    }
    void f(TreeNode* r) {
        tmp.push_back('a' + r->val);
        if (r->left == nullptr && r->right == nullptr)
            if (comp(tmp, res) > 0)
                res = tmp;
        if (r->left)
            f(r->left);
        if (r->right)
            f(r->right);
        tmp.pop_back();
    }
public:
    string smallestFromLeaf(TreeNode* root) {
        res = "~";
        tmp = "";
        f(root);
        return string(res.rbegin(), res.rend());
    }
};