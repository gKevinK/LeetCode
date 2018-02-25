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
    bool hasCycle(ListNode *head) {
        ListNode *p1 = head, *p2 = head;
        while (true) {
            if (p1 == NULL) return false;
            p1 = p1->next;
            if (p1 == NULL) return false;
            p1 = p1->next;
            p2 = p2->next;
            if (p1 == p2) return true;
        }
    }
};