class MyCircularQueue {
    vector<int> v;
    int start = 0, len = 0, capa = 0;
public:
    /** Initialize your data structure here. Set the size of the queue to be k. */
    MyCircularQueue(int k) {
        capa = k;
        v = vector<int>(k);
    }
    
    /** Insert an element into the circular queue. Return true if the operation is successful. */
    bool enQueue(int value) {
        if (len == capa) return false;
        v[(start + len++) % capa] = value;
        return true;
    }
    
    /** Delete an element from the circular queue. Return true if the operation is successful. */
    bool deQueue() {
        if (len == 0) return false;
        start = (start + 1) % capa;
        len--;
        return true;
    }
    
    /** Get the front item from the queue. */
    int Front() {
        return len == 0 ? -1 : v[start];
    }
    
    /** Get the last item from the queue. */
    int Rear() {
        return len == 0 ? -1 : v[(start + len - 1 + capa) % capa];
    }
    
    /** Checks whether the circular queue is empty or not. */
    bool isEmpty() {
        return len == 0;
    }
    
    /** Checks whether the circular queue is full or not. */
    bool isFull() {
        return len == capa;
    }
};

/**
 * Your MyCircularQueue object will be instantiated and called as such:
 * MyCircularQueue* obj = new MyCircularQueue(k);
 * bool param_1 = obj->enQueue(value);
 * bool param_2 = obj->deQueue();
 * int param_3 = obj->Front();
 * int param_4 = obj->Rear();
 * bool param_5 = obj->isEmpty();
 * bool param_6 = obj->isFull();
 */