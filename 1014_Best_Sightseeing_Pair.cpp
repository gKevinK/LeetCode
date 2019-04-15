class Solution {
public:
    int maxScoreSightseeingPair(vector<int>& A) {
        int n = A.size(), m = A[0], r = -100000;
        for (int i = 1; i < n; ++i) {
            r = max(r, m + A[i] - i);
            m = max(m, A[i] + i);
        }
        return r;
    }
};