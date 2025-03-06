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
    int numComponents(ListNode* head, vector<int>& G) {
        int c = 0;
        unordered_set<int> s(G.begin(), G.end());
        while (head) {
            if (s.count(head->val) && (head->next == NULL || !s.count(head->next->val)))
                c++;
            head = head->next;
        }
        return c;
    }
};