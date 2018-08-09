class Solution {
    int gcd(int A, int B) {
        if (A < B) return gcd(B, A);
        if (A % B == 0) return B;
        return gcd(B, A % B);
    }
public:
    int nthMagicalNumber(int N, int A, int B) {
        int mod = 1e9 + 7, cd = gcd(A, B), lcm = A / cd * B, cpp = lcm / A + lcm / B - 1;
        int lcms = N / cpp, rst = N % cpp;
        double n = rst / (1. / A + 1. / B);
        int v = min(ceil(n / A) * A, ceil(n / B) * B);
        return ((long long)lcms * lcm + v) % mod;
    }
};