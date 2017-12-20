/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */
class Solution {
private:
    static bool comp(ListNode * & a, ListNode * & b) {
        if (b == NULL)
            return false;
        else if (a == NULL)
            return true;
        else
            return a->val > b->val;
    };
    
public:
    ListNode* mergeKLists(vector<ListNode*>& lists) {
        if (lists.size() == 0) return NULL;
        ListNode *p = NULL, *r = NULL;
        make_heap(lists.begin(), lists.end(), comp);
        while ((*lists.begin()) != NULL) {
            ListNode * tmp = *lists.begin();
            pop_heap(lists.begin(), lists.end(), comp);
            if (p == NULL) {
                p = r = tmp;
            } else {
                r->next = tmp;
                r = tmp;
            }
            *(lists.end() - 1) = tmp->next;
            push_heap(lists.begin(), lists.end(), comp);
        }
        if (r != NULL) r->next = NULL;
        return p;
    }
};