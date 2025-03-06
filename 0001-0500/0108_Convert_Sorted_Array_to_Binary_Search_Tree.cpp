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
private:
    TreeNode* s(vector<int>& nums, int l, int r) {
        if (r - l == 0) return NULL;
        if (r - l == 1) return new TreeNode(nums[l]);
        int c2 = (r - l - 1)/ 2;
        int c1 = (r - l - 1) - c2;
        TreeNode *n = new TreeNode(nums[l+c1]);
        n->left = s(nums, l, l+c1);
        n->right = s(nums, l+c1+1, r);
        return n;
    }

public:
    TreeNode* sortedArrayToBST(vector<int>& nums) {
        return s(nums, 0, nums.size());
    }
};