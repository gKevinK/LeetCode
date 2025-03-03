class FreqStack {
    unordered_map<int, int> freq;
    unordered_map<int, stack<int>> fstack;
    int mfreq = 0;
public:
    FreqStack() {
        
    }
    
    void push(int x) {
        freq[x]++;
        mfreq = max(mfreq, freq[x]);
        fstack[freq[x]].push(x);
    }
    
    int pop() {
        int r = fstack[mfreq].top();
        freq[r]--;
        fstack[mfreq].pop();
        if (fstack[mfreq].empty()) {
            fstack.erase(mfreq);
            mfreq--;
        }
        return r;
    }
};

/**
 * Your FreqStack object will be instantiated and called as such:
 * FreqStack obj = new FreqStack();
 * obj.push(x);
 * int param_2 = obj.pop();
 */