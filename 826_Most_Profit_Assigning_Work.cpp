class Solution {
public:
    int maxProfitAssignment(vector<int>& difficulty, vector<int>& profit, vector<int>& worker) {
        unordered_map<int, int> m;
        for (int i = 0; i < difficulty.size(); i++)
            m[difficulty[i]] = max(m[difficulty[i]], profit[i]);
        sort(difficulty.begin(), difficulty.end());
        int tm = m[difficulty[0]];
        for (int i = 0; i < difficulty.size(); i++) {
            tm = max(tm, m[difficulty[i]]);
            profit[i] = tm;
        }
        int s = 0;
        for (int & w : worker) {
            if (w < difficulty.front())
                continue;
            int lo = 0, hi = profit.size(), mi;
            while (lo < hi) {
                mi = (hi - lo) / 2 + lo;
                if (difficulty[mi] <= w)
                    lo = mi + 1;
                else
                    hi = mi;
            }
            s += profit[lo - 1];
        }
        return s;
    }
};