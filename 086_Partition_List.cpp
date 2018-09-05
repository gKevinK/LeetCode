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
    ListNode* partition(ListNode* head, int x) {
        ListNode small(0), large(0);
        ListNode * sn = &small, * ln = &large;
        while (head) {
            if (head->val < x) {
                sn->next = head;
                head = head->next;
                sn = sn->next;
                sn->next = NULL;
            } else {
                ln->next = head;
                head = head->next;
                ln = ln->next;
                ln->next = NULL;
            }
        }
        sn->next = large.next;
        return small.next;
    }
};