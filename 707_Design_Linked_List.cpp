class MyLinkedList {
    struct Node {
        Node * prev, * next;
        int val;
    };
    
    Node * head, * tail;
    int count;
    
    void _insert(Node * p, int val) {
        Node * n = new Node { p, p->next, val };
        p->next->prev = n;
        p->next = n;
        ++count;
    }
public:
    /** Initialize your data structure here. */
    MyLinkedList() {
        head = new Node { nullptr, nullptr, 0 };
        tail = new Node { head, nullptr, 0 };
        head->next = tail;
        count = 0;
    }
    
    /** Get the value of the index-th node in the linked list. If the index is invalid, return -1. */
    int get(int index) {
        if (index < 0 || index >= count) return -1;
        Node * t = head->next;
        for (int i = 0; i < index; ++i)
            t = t->next;
        return t->val;
    }
    
    /** Add a node of value val before the first element of the linked list. After the insertion, the new node will be the first node of the linked list. */
    void addAtHead(int val) {
        _insert(head, val);
    }
    
    /** Append a node of value val to the last element of the linked list. */
    void addAtTail(int val) {
        _insert(tail->prev, val);
    }
    
    /** Add a node of value val before the index-th node in the linked list. If index equals to the length of linked list, the node will be appended to the end of linked list. If index is greater than the length, the node will not be inserted. */
    void addAtIndex(int index, int val) {
        if (index > count) return;
        if (index < 0) _insert(head, val);
        Node * p = head;
        for (int i = 0; i < index; ++i)
            p = p->next;
        _insert(p, val);
    }
    
    /** Delete the index-th node in the linked list, if the index is valid. */
    void deleteAtIndex(int index) {
        if (index < 0 || index >= count) return;
        Node * p = head->next;
        for (int i = 0; i < index; ++i)
            p = p->next;
        p->next->prev = p->prev;
        p->prev->next = p->next;
        delete p;
        --count;
    }
};

/**
 * Your MyLinkedList object will be instantiated and called as such:
 * MyLinkedList* obj = new MyLinkedList();
 * int param_1 = obj->get(index);
 * obj->addAtHead(val);
 * obj->addAtTail(val);
 * obj->addAtIndex(index,val);
 * obj->deleteAtIndex(index);
 */