class Solution {
public:
    int networkDelayTime(vector<vector<int>>& times, int N, int K) {
        vector<int> t(N + 1, INT_MAX);
        t[K] = 0;
        for (int i = 0; i < N; i++) {
            for (auto & e : times) {
                if (t[e[0]] < INT_MAX) {
                    t[e[1]] = min(t[e[1]], t[e[0]] + e[2]);
                }
            }
        }
        int m = 0;
        for (int i = 1; i <= N; i++) {
            m = max(m, t[i]);
        }
        return m == INT_MAX ? -1 : m;
    }
};