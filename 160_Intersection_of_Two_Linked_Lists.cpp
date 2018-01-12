/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */

// class Solution {
// public:
//     ListNode *getIntersectionNode(ListNode *headA, ListNode *headB) {
//         ListNode * p1 = headA, * p2 = headB;
//         while (p1 != p2) {
//             p1 = p1 == NULL ? headA : p1->next;
//             p2 = p2 == NULL ? headB : p2->next;
//         }
//         return p1;
//     }
// };

class Solution {
public:
    ListNode *getIntersectionNode(ListNode *headA, ListNode *headB) {
        ListNode * p1 = headA, * p2 = headB;
        int l1 = 0, l2 = 0;
        while (p1) {
            p1 = p1->next; l1++;
        }
        while (p2) {
            p2 = p2->next; l2++;
        }
        p1 = headA;
        p2 = headB;
        while (l1 > l2) {
            p1 = p1->next; l1--;
        }
        while (l1 < l2) {
            p2 = p2->next; l2--;
        }
        while (p1 && p2 && p1 != p2) {
            p1 = p1->next;
            p2 = p2->next;
        }
        return p1;
    }
};