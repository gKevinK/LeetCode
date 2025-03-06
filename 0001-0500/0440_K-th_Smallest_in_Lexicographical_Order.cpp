class Solution {
public:
    int findKthNumber(int n, int k) {
        if (k == 0) return 0;
        int r = 1;
        k--;
        while (k) {
            int step = 0;
            long long n1 = r, n2 = r + 1;
            while (n1 <= n) {
                step += min(static_cast<long long>(n) + 1, n2) - n1;
                n1 *= 10;
                n2 *= 10;
            }
            if (k < step) {
                k--;
                r *= 10;
            } else {
                r++;
                k -= step;
            }
        }
        return r;
    }
};