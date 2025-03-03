/*
// Definition for a Node.
class Node {
public:
    int val = NULL;
    Node* prev = NULL;
    Node* next = NULL;
    Node* child = NULL;

    Node() {}

    Node(int _val, Node* _prev, Node* _next, Node* _child) {
        val = _val;
        prev = _prev;
        next = _next;
        child = _child;
    }
};
*/
class Solution {
public:
    Node* flatten(Node* head) {
        if (head == NULL) return NULL;
        vector<Node *> v = { head };
        Node h(0, NULL, NULL, NULL);
        Node *t = & h;
        while (!v.empty()) {
            Node * t2 = v.back();
            v.back() = t2->next;
            if (t2->child)
                v.push_back(t2->child);
            t->next = t2;
            t2->prev = t;
            t = t2;
            t->next = NULL;
            t->child = NULL;
            while (!v.empty() && v.back() == NULL)
                v.pop_back();
        }
        h.next->prev = NULL;
        return h.next;
    }
};