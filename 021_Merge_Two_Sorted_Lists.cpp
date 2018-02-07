/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */
class Solution {
public:
    ListNode* mergeTwoLists(ListNode* l1, ListNode* l2) {
        ListNode h(0);
        ListNode * c = &h;
        while (l1 != NULL || l2 != NULL) {
            if (l1 && l2 && l1->val < l2->val || l2 == NULL) {
                c->next = l1;
                l1 = l1->next;
            } else {
                c->next = l2;
                l2 = l2->next;
            }
            c = c->next;
        }
        return h.next;
    }
};