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
    ListNode* reverseBetween(ListNode* head, int m, int n) {
        ListNode h(0);
        h.next = head;
        ListNode *p1 = &h, *p2, *p3;
        for (int i = 1; i < m; i++)
            p1 = p1->next;
        p3 = p1->next;
        for (int i = 0; i <= n - m; i++) {
            ListNode *p4 = p1->next;
            p1->next = p1->next->next;
            p4->next = p2;
            p2 = p4;
        }
        p3->next = p1->next;
        p1->next = p2;
        return h.next;
    }
};