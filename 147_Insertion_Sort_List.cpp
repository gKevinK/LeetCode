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
    ListNode* insertionSortList(ListNode* head) {
        ListNode h(0);
        while (head) {
            ListNode *t1 = &h, *t2;
            for (; t1->next && t1->next->val <= head->val; t1 = t1->next);
            t2 = t1->next;
            t1->next = head;
            head = head->next;
            t1->next->next = t2;
        }
        return h.next;
    }
};