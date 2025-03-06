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
    ListNode* swapPairs(ListNode* head) {
        ListNode** h = &head;
        ListNode* a = head, * b;
        while (a) {
            b = a->next;
            if (b == NULL) break;
            *h = b;
            a->next = b->next;
            b->next = a;
            h = &(a->next);
            a = *h;
        }
        return head;
    }
};