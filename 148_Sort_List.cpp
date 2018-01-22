/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */
class Solution {
    ListNode* s(ListNode* b, ListNode* e) {
        if (b == e || b->next == e) return b;
        ListNode h(0);
        ListNode *t1 = b, *t2 = b, *l, *r, *t = &h;
        int n;
        while (t1 != e) {
            t1 = t1->next;
            n++;
            if (n % 2 == 0)
                t2 = t2->next;
        }
        l = s(b, t2);
        r = s(t2, e);
        while (l != t2 || r != e) {
            ListNode ** c;
            if (l == t2 || (r != e && l->val > r->val)) {
                c = &r;
            } else {
                c = &l;
            }
            ListNode *tmp = t->next;
            t->next = *c;
            *c = (*c)->next;
            t->next->next = tmp;
            t = t->next;
        }
        t->next = e;
        return h.next;
    }
    
public:
    ListNode* sortList(ListNode* head) {
        return s(head, NULL);
    }
};