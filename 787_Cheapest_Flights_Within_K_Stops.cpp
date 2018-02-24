class Solution {
public:
    int findCheapestPrice(int n, vector<vector<int>>& flights, int src, int dst, int K) {
        unordered_map<int, int> m1, m2;
        m1[src] = 0;
        int res = INT_MAX;
        for (int i = 0; i <= K; i++) {
            for (const auto & f : flights) {
                if (m1.find(f[0]) != m1.end() && m1[f[0]] + f[2] <= res) {
                    if (m2.find(f[1]) == m2.end())
                        m2[f[1]] = m1[f[0]] + f[2];
                    else
                        m2[f[1]] = min(m1[f[0]] + f[2], m2[f[1]]);
                }
            }
            m1.clear();
            m1.swap(m2);
            if (m1.find(dst) != m1.end())
                res = min(res, m1[dst]);
        }
        return res == INT_MAX ? -1 : res;
    }
};