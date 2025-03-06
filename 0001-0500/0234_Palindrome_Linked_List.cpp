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
    bool isPalindrome(ListNode* head) {
        ListNode * t1 = head, * t2 = NULL, * t3;
        int n = 0;
        while (t1 != NULL) {
            n++;
            t1 = t1->next;
        }
        t1 = head;
        while (n) {
            if (n == 1) {
                t3 = t1;
                t1 = t1->next;
                t3->next = NULL;
                n--;
            } else {
                t3 = t1;
                t1 = t1->next;
                t3->next = t2;
                t2 = t3;
                n -= 2;
            }
        }
        while (t1 != NULL || t2 != NULL) {
            if (t1->val != t2->val)
                return false;
            t1 = t1->next;
            t2 = t2->next;
        }
        return t1 == t2;
    }
};