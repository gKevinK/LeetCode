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
    ListNode* reverseList(ListNode* head) {
        ListNode h(0);
        while (head) {
            ListNode * t = h.next;
            h.next = head;
            head = head->next;
            h.next->next = t;
        }
        return h.next;
    }
};