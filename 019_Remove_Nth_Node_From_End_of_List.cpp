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
    ListNode* removeNthFromEnd(ListNode* head, int n) {
        ListNode *a = head, *b = head, *c;
        for (int i = 1; i < n; i++) {
            b = b -> next;
        }
        while (b -> next != NULL) {
            c = a;
            a = a -> next;
            b = b -> next;
        }
        if (a == head)
            head = head -> next;
        else
            c -> next = c -> next -> next;
        return head;
    }
};