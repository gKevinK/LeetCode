class Solution {
public:
    int sumSubseqWidths(vector<int>& A) {
        int MOD = 1'000'000'007;
        long r = 0, num = 0, minus = 0;
        sort(A.begin(), A.end());
        int curr = 0;
        for (int i = 1; i <= 20000; ++i) {
            int c = curr;
            while (curr < A.size() && A[curr] == i) curr++;
            c = curr - c;
            if (c == 0) continue;
            int pm = 1;
            for (int j = 0; j < c; ++j) pm = pm * 2 % MOD;
            r = (r + num * (pm - 1) * i - minus * (pm - 1)) % MOD;
            num = (num * pm + pm - 1) % MOD;
            minus = (minus * pm + (pm - 1) * i) % MOD;
        }
        return r;
    }
};