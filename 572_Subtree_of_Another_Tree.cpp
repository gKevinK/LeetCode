/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */

// class Solution {
//     bool f(TreeNode* s, TreeNode* t) {
//         if (s == NULL && t == NULL) return true;
//         if (s == NULL || t == NULL) return false;
//         if (s->val != t->val) return false;
//         return f(s->left, t->left) && f(s->right, t->right);
//     }
// public:
//     bool isSubtree(TreeNode* s, TreeNode* t) {
//         if (s == NULL) return false;
//         if (s->val == t->val && f(s, t)) return true;
//         return isSubtree(s->left, t) || isSubtree(s->right, t);
//     }
// };

class Solution {
    bool isSame(TreeNode* s, TreeNode* t) {
        if (s == NULL && t == NULL) return true;
        if (s == NULL || t == NULL) return false;
        if (s->val != t->val) return false;
        return isSame(s->left, t->left) && isSame(s->right, t->right);
    }
    int depth(TreeNode* s) {
        if (s == NULL) return 0;
        else return max(depth(s->left), depth(s->right)) + 1;
    }
    int f(TreeNode* s, TreeNode* t, int d) {
        if (s == NULL) return 0;
        int a = f(s->left, t, d), b = f(s->right, t, d);
        if (a == -1 || b == -1) return -1;
        int d1 = max(a, b) + 1;
        if (s->val == t->val && d1 == d && isSame(s, t)) return -1;
        return d1;
    }
public:
    bool isSubtree(TreeNode* s, TreeNode* t) {
        if (s == NULL) return false;
        int d = depth(t);
        return f(s, t, d) == -1;
    }
};