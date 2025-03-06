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
    bool check(TreeNode* r, int s, int l, bool le, bool ri) {
        if (r == NULL) return true;
        if (!le  && r -> val <= s) return false;
        if (!ri && r -> val >= l) return false;
        return check(r -> left, s, r -> val, le, false) && check(r -> right, r -> val, l, false, ri);
    }
public:
    bool isValidBST(TreeNode* root) {
        return check(root, 0, 0, true, true);
    }
};