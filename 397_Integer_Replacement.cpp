class Solution {
public:
    int integerReplacement(int n) {
        int r = 0;
        if (n == INT_MAX) return 32;
        while (n > 1) {
            if ((n & 1) == 0) {
                n >>= 1;
            } else if ((n & 2) == 0 || n == 3) {
                n >>= 1;
                r++;
            } else {
                int m = 1;
                while (m & n) {
                    n &= ~m;
                    m <<= 1;
                }
                n |= m;
            }
            r++;
        }
        return r;
    }
};