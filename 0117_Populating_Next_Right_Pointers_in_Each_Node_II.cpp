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
    void connect(TreeLinkNode *root) {
        if (root == NULL) return;
        TreeLinkNode *p0 = root, *p1 = NULL, *p2 = NULL, *p3 = NULL;
        while (true) {
            while (p0 -> left == NULL && p0 -> right == NULL) {
                p0 = p0 -> next;
                if (p0 == NULL)
                    return;
            }
            if (p0 -> left != NULL)
                p2 = p0 -> left;
            else
                p2 = p0 -> right;
            p1 = p0;
            p3 = p2;
            if (p2 == p0 -> left && p1 -> right != NULL) {
                p3 -> next = p1 -> right;
                p3 = p3 -> next;
            }
            while (p1 -> next != NULL) {
                p1 = p1 -> next;
                if (p1 -> left != NULL) {
                    p3 -> next = p1 -> left;
                    p3 = p3 -> next;
                }
                if (p1 -> right != NULL) {
                    p3 -> next = p1 -> right;
                    p3 = p3 -> next;
                }
            }
            p0 = p2;
        }
    }
};