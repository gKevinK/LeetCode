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
    TreeNode* copy(TreeNode* n) {
        // if (n == NULL) return NULL;
        // TreeNode* nn = new TreeNode(0);
        // nn->left = copy(n->left);
        // nn->right = copy(n->right);
        // return nn;
        return n;
    }
    vector<vector<TreeNode*>> cache;
public:
    vector<TreeNode*> allPossibleFBT(int N) {
        if (cache.empty()) cache = vector<vector<TreeNode*>>(21);
        if (cache[N].size() || N % 2 == 0) return cache[N];
        if (N == 1) return { new TreeNode(0) };
        vector<TreeNode*> res;
        for (int i = 1; i < N; i += 2) {
            for (auto & t1 : allPossibleFBT(i)) {
                for (auto & t2 : allPossibleFBT(N - i - 1)) {
                    TreeNode* n = new TreeNode(0);
                    n->left = copy(t1);
                    n->right = copy(t2);
                    res.push_back(n);
                }
            }
        }
        cache[N] = res;
        return res;
    }
};