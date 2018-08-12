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
    ListNode* deleteDuplicates(ListNode* head) {
        if (head == NULL) return head;
        ListNode* c = head;
        while (c -> next != NULL) {
            if (c -> next -> val == c -> val) {
                ListNode* t = c -> next;
                c -> next = c -> next -> next;
                delete t;
            } else {
                c = c -> next;
            }
        }
        return head;
    }
};