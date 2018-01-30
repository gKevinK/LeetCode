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
    ListNode* reverseKGroup(ListNode* head, int k) {
        ListNode head1(0);
        head1.next = head;
        head = &head1;
        while (head) {
            ListNode *p1, *p2 = head, *p3, *p4;
            for (int i = 0; i < k && p2; i++)
                p2 = p2->next;
            if (!p2) break;
            p1 = p3 = p2->next;
            p4 = head->next;
            for (p2 = head->next; p2 && p2 != p3; ) {
                ListNode* tmp = p2->next;
                p2->next = p1;
                p1 = p2;
                p2 = tmp;
            }
            head->next = p1;
            head = p4;
        }
        return head1.next;
    }
};