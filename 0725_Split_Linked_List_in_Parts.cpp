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
    vector<ListNode*> splitListToParts(ListNode* root, int k) {
        int n = 0;
        for (ListNode* p = root; p != NULL; p = p->next) n++;
        vector<ListNode*> v;
        for (int i = 0; i < k; i++) {
            ListNode t(0);
            ListNode *p = &t;
            for (int j = 0; j < n / k + (i < n % k); j++) {
                p->next = new ListNode(root->val);
                p = p->next;
                root = root->next;
            }
            v.push_back(t.next);
        }
        return v;
    }
};