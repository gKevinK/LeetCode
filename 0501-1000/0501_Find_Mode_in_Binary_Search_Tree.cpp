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
    int value, count = 0, countm = 0;
    vector<int> result;
    
    void t() {
        if (count == countm)
            result.push_back(value);
        else if (count > countm) {
            result.clear();
            result.push_back(value);
            countm = count;
        }
    }
    
    void f(TreeNode* r) {
        if (r->left) f(r->left);
        if (value == r->val) {
            count++;
        } else {
            t();
            value = r->val;
            count = 1;
        }
        if (r->right) f(r->right);
    }
    
public:
    vector<int> findMode(TreeNode* root) {
        value = INT_MIN;
        count = 0;
        countm = 0;
        result.clear();
        if (root) {
            f(root);
            t();
        }
        return result;
    }
};