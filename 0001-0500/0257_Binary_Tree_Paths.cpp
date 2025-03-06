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
    void f(TreeNode* root, vector<string> & v, string & s) {
        if (root == NULL) return;
        string s2 = s + "->" + to_string(root->val);
        if (root->left == NULL && root->right == NULL) {
            v.push_back(s2);
            return;
        }
        f(root->left, v, s2);
        f(root->right, v, s2);
    }
public:
    vector<string> binaryTreePaths(TreeNode* root) {
        vector<string> v;
        if (root == NULL) return v;
        string s2 = to_string(root->val);
        if (root->left == NULL && root->right == NULL)
            v.push_back(s2);
        f(root->left, v, s2);
        f(root->right, v, s2);
        return v;
    }
};