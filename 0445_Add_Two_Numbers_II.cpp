/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */
class Solution {
    ListNode* f(ListNode* l1, ListNode* l2, int diff) {
        if (l1 == NULL && l2 == NULL) return NULL;
        ListNode* curr, *next;
        if (diff > 0) {
            curr = new ListNode(l1->val);
            next = f(l1->next, l2, diff - 1);
        } else if (diff < 0) {
            curr = new ListNode(l2->val);
            next = f(l1, l2->next, diff + 1);
        } else {
            curr = new ListNode(l1->val + l2->val);
            next = f(l1->next, l2->next, 0);
        }
        curr->next = next;
        if (next != NULL && next->val >= 10) {
            curr->val += next->val / 10;
            next->val %= 10;
        }
        return curr;
    }
public:
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        int len1 = 0, len2 = 0;
        ListNode* t1 = l1, * t2 = l2;
        while (t1) len1++, t1 = t1->next;
        while (t2) len2++, t2 = t2->next;
        ListNode* r = f(l1, l2, len1 - len2);
        if (r && r->val >= 10) {
            ListNode* t = r;
            r = new ListNode(t->val / 10);
            t->val %= 10;
            r->next = t;
        }
        return r;
    }
};