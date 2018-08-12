class Solution {
public:
    bool isPerfectSquare(int num) {
        double d = (double)num;
        double x1, x2 = d;
        do {
            x1 = x2;
            x2 = x1 / 2 + d / 2.0 / x1;
        } while (x1 - x2 > 0.1 || x2 - x1 > 0.1);
        int r = (int)x2;
        return (r * r == num);
    }
};