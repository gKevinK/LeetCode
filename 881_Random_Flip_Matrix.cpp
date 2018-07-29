class Solution {
    unordered_map<int, int> mp;
    int m, n, idx;
public:
    Solution(int n_rows, int n_cols) {
        m = n_rows;
        n = n_cols;
        reset();
    }
    
    vector<int> flip() {
        int k = rand() % (idx + 1);
        int v = k;
        if (mp.find(k) != mp.end()) v = mp[k];
        mp[k] = (mp.find(idx) != mp.end()) ? mp[idx]  : idx;
        idx--;
        return { v / n, v % n };
    }
    
    void reset() {
        idx = m * n - 1;
        mp.clear();
    }
};

/**
 * Your Solution object will be instantiated and called as such:
 * Solution obj = new Solution(n_rows, n_cols);
 * vector<int> param_1 = obj.flip();
 * obj.reset();
 */