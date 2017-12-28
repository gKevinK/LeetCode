/**
 * Definition for singly-linked list with a random pointer.
 * struct RandomListNode {
 *     int label;
 *     RandomListNode *next, *random;
 *     RandomListNode(int x) : label(x), next(NULL), random(NULL) {}
 * };
 */
class Solution {
public:
    RandomListNode *copyRandomList(RandomListNode *head) {
        for (RandomListNode * p = head; p; p = p->next->next) {
            RandomListNode * p1 = new RandomListNode(p->label);
            p1->next = p->next;
            p->next = p1;
        }
        for (RandomListNode * p = head; p; p = p->next->next)
            if (p->random)
                p->next->random = p->random->next;
        RandomListNode *head2 = new RandomListNode(0);
        for (RandomListNode * p = head, *p2 = head2; p; p = p->next) {
            p2->next = p->next;
            p2 = p2->next;
            p->next = p2->next;
        }
        RandomListNode *h = head2->next;
        delete head2;
        return h;
    }
};