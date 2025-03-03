class Solution {
public:
    int divide(int dividend, int divisor) {
        if (divisor == 0 || (dividend == INT_MIN && divisor == -1)) return INT_MAX;
        long long a = labs(dividend);
        long long b = labs(divisor);
        int res = 0;
        long long t = b, m = 1;
        while (a >= (t << 1)) {
            t <<= 1; m <<= 1;
        }
        while (m >= 1) {
            if (a >= t) {
                a -= t;
                res += m;
            }
            t >>= 1;
            m >>= 1;
        }
        return res * (dividend < 0 ^ divisor < 0 ? -1 : 1);
    }
};