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
public:
    TreeNode* constructMaximumBinaryTree(vector<int>& nums) {
        deque<TreeNode*> s;
        for (int & i : nums) {
            TreeNode* n = new TreeNode(i);
            while (!s.empty() && s.back()->val < i) {
                n->left = s.back();
                s.pop_back();
            }
            if (!s.empty())
                s.back()->right = n;
            s.push_back(n);
        }
        return s.front();
    }
};