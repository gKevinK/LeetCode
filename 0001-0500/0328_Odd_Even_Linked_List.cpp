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
    ListNode* oddEvenList(ListNode* head) {
        ListNode ho(0), he(0);
        ListNode *t1 = &ho, *t2 = &he;
        int n = 0;
        while (head) {
            n++;
            t1->next = head;
            t1 = t2;
            t2 = head;
            head = head->next;
            t2->next = NULL;
        }
        if (n % 2)
            t2->next = he.next;
        else
            t1->next = he.next;
        return ho.next;
    }
};