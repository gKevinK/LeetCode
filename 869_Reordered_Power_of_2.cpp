class Solution {
    long long c(int N) {
        long long b = 0;
        for (; N > 0; N /= 10) {
            b += (int)pow(10, N % 10);
        }
        return b;
    }
public:
    bool reorderedPowerOf2(int N) {
        long long b = c(N);
        for (int i = 0; i < 31; i++) {
            if (b == c(pow(2, i))) return true;
        }
        return false;
    }
};