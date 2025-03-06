class Solution {
public:
    double myPow(double x, int n) {
        if (x == 0)
            return 0.0;
        if (n == 0) return 1.0;
        if (n > 0) {
            return pow1(x, n);
        } else {
            return pow2(x, n);
        }
    }
    
    double pow1(double x, int n) {
        if (n == 0) return 1.0;
        double r = pow1(x, n / 2);
        if (n % 2 == 1) return r * r * x;
        else return r * r;
    }
    
    double pow2(double x, int n) {
        if (n == 0) return 1.0;
        double r = pow2(x, n / 2);
        if (n % 2 == -1) return r * r / x;
        else return r * r;
    }
};