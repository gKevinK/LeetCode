/**
 * Definition for binary tree with next pointer.
 * struct TreeLinkNode {
 *  int val;
 *  TreeLinkNode *left, *right, *next;
 *  TreeLinkNode(int x) : val(x), left(NULL), right(NULL), next(NULL) {}
 * };
 */
class Solution {
public:
    void connect2(TreeLinkNode *left, TreeLinkNode *right) {
        if (left -> right == NULL) return;
        left -> right -> next = right -> left;
        connect2(left -> right, right -> left);
    }
    void connect(TreeLinkNode *root) {
        if (root == NULL) return;
        if (root -> left == NULL) return;
        root -> left -> next = root -> right;
        connect(root -> left);
        connect(root -> right);
        connect2(root -> left, root -> right);
    }
};