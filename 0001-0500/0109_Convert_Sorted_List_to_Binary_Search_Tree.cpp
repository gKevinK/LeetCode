/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */
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
    TreeNode* f(ListNode* h, ListNode* e) {
        if (h == e) return NULL;
        if (h->next == e) return new TreeNode(h->val);
        ListNode *p1 = h, *p2 = h;
        int i = 0;
        while (p1 != e) {
            p1 = p1->next;
            i++;
            if (i % 2 == 0)
                p2 = p2->next;
        }
        TreeNode * r = new TreeNode(p2->val);
        r->left = f(h, p2);
        r->right = f(p2->next, e);
        return r;
    }
public:
    TreeNode* sortedListToBST(ListNode* head) {
        return f(head, NULL);
    }
};