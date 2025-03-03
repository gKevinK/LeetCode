class Solution {
public:
    int findKthNumber(int m, int n, int k) {
        int lo = 1, hi = m * n;
        while (lo < hi) {
            int mi = (hi - lo + 1) / 2 + lo;
            int less = 0;
            for (int i = 1, j = n; i <= m; ++i) {
                while (j > 0 && i * j >= mi) --j;
                less += j;
            }
            if (less >= k) hi = mi - 1;
            else lo = mi;
        }
        return lo;
    }
};