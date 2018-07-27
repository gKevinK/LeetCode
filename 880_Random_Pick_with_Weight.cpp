class Solution {
    vector<int> v;
public:
    Solution(vector<int> w) {
        v = w;
        int s = 0;
        for (int & i : v) {
            s += i;
            i = s;
        }
    }
    
    int pickIndex() {
        int lo = 0, hi = v.size() - 1, t = rand() % v.back() + 1;
        while (lo < hi) {
            int mi = (hi - lo) / 2 + lo;
            // cout << lo << " " << mi << " " << hi << endl;
            if (v[mi] < t) lo = mi + 1;
            else if (v[mi] > t) hi = mi;
            else return mi;
        }
        return lo;
    }
};

/**
 * Your Solution object will be instantiated and called as such:
 * Solution obj = new Solution(w);
 * int param_1 = obj.pickIndex();
 */