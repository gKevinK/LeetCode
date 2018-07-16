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
    void f(TreeNode* r, int s, int sum, int & n, unordered_map<int, int>& m) {
        if (r == NULL) return;
        s += r->val;
        if (m.find(s - sum) != m.end()) n += m[s - sum];
        m[s]++;
        f(r->left, s, sum, n, m);
        f(r->right, s, sum, n, m);
        m[s]--;
        if (m[s] == 0)
            m.erase(s);
    }
public:
    int pathSum(TreeNode* root, int sum) {
        unordered_map<int, int> m;
        m[0] = 1;
        int n = 0;
        f(root, 0, sum, n, m);
        return n;
    }
};