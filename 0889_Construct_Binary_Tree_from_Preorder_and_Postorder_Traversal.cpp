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
    TreeNode* f(vector<int>& pre, vector<int>& post, int l1, int len, int l2) {
        if (len <= 0) return NULL;
        TreeNode* node = new TreeNode(pre[l1]);
        if (len > 1) {
            int m = 0;
            while (post[l2 + m] != pre[l1 + 1]) m++;
            m++;
            node->left = f(pre, post, l1 + 1, m, l2);
            node->right = f(pre, post, l1 + m + 1, len - m - 1, l2 + m);
        }
        return node;
    }
public:
    TreeNode* constructFromPrePost(vector<int>& pre, vector<int>& post) {
        return f(pre, post, 0, pre.size(), 0);
    }
};