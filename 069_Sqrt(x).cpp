class Solution {
public:
    int mySqrt(int x) {
        if (x == 0) return 0;
        double a = x, b = x + 1;
        while (a - b > 0.05 || a - b < -0.05) {
            b = a;
            a = a / 2 + x / a / 2;
        }
        return (int)a;
    }
};