class Solution {
public:
    int countNumbersWithUniqueDigits(int n) {
        if (n == 0) {
            return 1;
        } else if (n == 1) {
            return 10;
        } else {
            int r = 9;
            for (int i = 1; i < n; ++i) {
                r *= (10 - i);
            }
            return r + countNumbersWithUniqueDigits(n - 1);
        }
    }
};