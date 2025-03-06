class Solution {
public:
    int reverse(int x) {
        if (x == INT_MIN) return 0;
        if (x < 0) return -reverse(-x);
        int t = 0;
        while (x > 0) {
            if (t == 214748364 && x % 10 > 7 || t > 214748364) return 0;
            t = t * 10 + x % 10;
            x /= 10;
        }
        return t;
    }
};