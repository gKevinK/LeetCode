class Solution {
    int n;
    vector<int> v;
    default_random_engine e;
    uniform_int_distribution<int> d;
    
public:
    Solution(int N, vector<int>& blacklist) {
        n = N - blacklist.size();
        v = vector<int>(blacklist);
        sort(v.begin(), v.end());
        for (int i = 1, c = 1; i < v.size(); ++i) {
            v[i] -= c++;
        }

        d = uniform_int_distribution<int>(0, n - 1);
    }
    
    int pick() {
        int r = d(e);
        auto p = upper_bound(v.begin(), v.end(), r);
        return r + distance(v.begin(), p);
    }
};

/**
 * Your Solution object will be instantiated and called as such:
 * Solution* obj = new Solution(N, blacklist);
 * int param_1 = obj->pick();
 */