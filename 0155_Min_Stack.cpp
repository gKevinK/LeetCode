class MinStack {
    vector<int> v, s;
public:
    /** initialize your data structure here. */
    MinStack() {
        
    }
    
    void push(int x) {
        v.push_back(x);
        if (s.empty() || x < v[s.back()])
            s.push_back(v.size() - 1);
    }
    
    void pop() {
        v.pop_back();
        if (s.back() == v.size())
            s.pop_back();
    }
    
    int top() {
        return v.back();
    }
    
    int getMin() {
        return v[s.back()];
    }
};

/**
 * Your MinStack object will be instantiated and called as such:
 * MinStack obj = new MinStack();
 * obj.push(x);
 * obj.pop();
 * int param_3 = obj.top();
 * int param_4 = obj.getMin();
 */