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
        if (head == NULL || head->next == NULL) return head;
        ListNode h(0);
        h.next = head;
        while (head) {
            if (head->next && head->next->val < head->val) {
                ListNode *t1, *t2;
                for (t1 = &h; t1->next && t1->next->val < head->next->val; t1 = t1->next);
                t2 = t1->next;
                t1->next = head->next;
                head->next = head->next->next;
                t1->next->next = t2;
            } else {
                head = head->next;
            }
        }
        return h.next;
    }
};