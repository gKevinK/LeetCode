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
        ListNode h(0);
        h.next = head;
        head = &h;
        while (head->next) {
            if (head->next->next && head->next->val == head->next->next->val) {
                int v = head->next->val;
                while (head->next && head->next->val == v) {
                    ListNode * t = head->next;
                    head->next = head->next->next;
                    delete t;
                }
            } else
                head = head->next;
        }
        return h.next;
    }
};