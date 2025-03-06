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
    vector<int> nextLargerNodes(ListNode* head) {
        vector<int> v;
        stack<int> s;
        for (ListNode * h = head; h; h = h->next) {
            v.push_back(h->val);
        }
        for (int i = v.size() - 1; i >= 0; --i) {
            int t = v[i];
            while (!s.empty() && s.top() <= t)
                s.pop();
            v[i] = s.empty() ? 0 : s.top();
            s.push(t);
        }
        return v;
    }
};