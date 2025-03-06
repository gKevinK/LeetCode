class KthLargest {
    priority_queue<int, vector<int>, greater<int>> pq;
public:
    KthLargest(int k, vector<int>& nums) {
        for (int & i : nums) {
            if (pq.size() < k)
                pq.push(i);
            else
                add(i);
        }
        if (pq.size() < k)
            pq.push(INT_MIN);
    }
    
    int add(int val) {
        if (val > pq.top()) {
            pq.pop();
            pq.push(val);
        }
        return pq.top();
    }
};

/**
 * Your KthLargest object will be instantiated and called as such:
 * KthLargest* obj = new KthLargest(k, nums);
 * int param_1 = obj->add(val);
 */