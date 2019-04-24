class Solution {
public:
    bool queryString(string S, int N) {
        int n = S.size();
        if (N > n * n) return false;
        vector<bool> r(N, false);
        for (int i = 0; i < n; ++i) {
            int d = 0;
            for (int j = i; j < n && d < N; ++j) {
                d = d * 2 + (S[j] - '0');
                if (d > 0 && d <= N)
                    r[d - 1] = true;
            }
        }
        for (bool b : r)
            if (b == false)
                return false;
        return true;
    }
};