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
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        int carry = 0, i1 = 0, i2 = 0;
        ListNode* p1 = l1, * p2 = l2, *r, *p3, *p4;
        r = p3 = p4 = new ListNode(0);
        while (p1 != NULL || p2 != NULL) {
            if (p1 == NULL) {
                i1 = 0;
            } else {
                i1 = p1 -> val;
                p1 = p1 -> next;
            }
            if (p2 == NULL) {
                i2 = 0;
            } else {
                i2 = p2 -> val;
                p2 = p2 -> next;
            }
            p3 = p3 -> next;
            if (p3 == NULL) p3 = new ListNode(0);
            p3 -> val = (carry + i1 + i2) % 10;
            p4 -> next = p3;
            carry = (carry + i1 + i2) / 10;
            if (carry) {
                p3 -> next = new ListNode(carry);
            }
            p4 = p3;
        }
        return r -> next;
    }
};