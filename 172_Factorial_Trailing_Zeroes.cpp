class Solution {
public:
    int trailingZeroes(int n) {
        int r = 0;
        while (n) {
            n = n / 5;
            r += n;
        }
        return r;
    }
};