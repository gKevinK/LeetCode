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
    unordered_map<int, int> m;
    
    int f(TreeNode* r) {
        if (r == nullptr) return 0;
        int sum = f(r->left) + r->val + f(r->right);
        m[sum]++;
        return sum;
    }
public:
    vector<int> findFrequentTreeSum(TreeNode* root) {
        m.clear();
        f(root);
        vector<int> r;
        int c = 0;
        for (const auto & p : m)
            c = max(c, p.second);
        for (const auto & p : m)
            if (p.second == c)
                r.push_back(p.first);
        return r;
    }
};