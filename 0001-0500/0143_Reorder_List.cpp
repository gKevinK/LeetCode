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
    void reorderList(ListNode* head) {
        if (head == NULL) return;
        ListNode *p1 = head, *p2 = head, *p3 = head;
        while (p1) {
            p1 = p1->next;
            if (p1) p1 = p1->next;
            p3 = p2;
            p2 = p2->next;
        }
        p3->next = NULL;
        while (p2) {
            ListNode * t = p2->next;
            p2->next = p1;
            p1 = p2;
            p2 = t;
        }
        p2 = head;
        while (p1 && p2) {
            ListNode * t = p2->next;
            p2->next = p1;
            p1 = p1->next;
            p2->next->next = t;
            p2 = t;
        }
    }
};