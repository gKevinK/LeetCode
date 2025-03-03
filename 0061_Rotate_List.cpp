class Solution {
public:
    ListNode* rotateRight(ListNode* head, int k) {
        if (k == 0 || head == NULL) return head;
        int t = 0;
        ListNode* tmp = head;
        while (tmp != NULL) {
            tmp = tmp -> next;
            t++;
        }
        k = k % t;
        ListNode* a = head, * b = head;
        for (int i = 0; i < k; ++i) {
            a = a -> next;
            if (a == NULL) return head;
        }
        if (a == NULL) return head;
        while (a -> next != NULL) {
            a = a -> next;
            b = b -> next;
        }
        a -> next = head;
        head = b -> next;
        b -> next = NULL;
        return head;
    }
};