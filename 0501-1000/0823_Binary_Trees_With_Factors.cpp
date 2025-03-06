class Solution {
public:
    int numFactoredBinaryTrees(vector<int>& A) {
        int MOD = 1'000'000'007, n = A.size();
        vector<long> v(n, 1);
        sort(A.begin(), A.end());
        for (int i = 0; i < n; ++i) {
            int j = 0, k = i - 1;
            while (j <= k) {
                long p = static_cast<long>(A[j]) * A[k];
                if (p < A[i]) {
                    ++j;
                } else if (p > A[i]) {
                    --k;
                } else {
                    v[i] = (v[i] + v[j] * v[k] * (j == k ? 1 : 2)) % MOD;
                    ++j; --k;
                }
            }
        }
        int r = 0;
        for (auto & i : v)
            r = (r + i) % MOD;
        return r;
    }
};