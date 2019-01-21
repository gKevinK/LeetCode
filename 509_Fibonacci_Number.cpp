class Solution {
public:
    int fib(int N) {
        if (N == 0) return 0;
        int a = 0, b = 1;
        for (int i = 1; i < N; i++) {
            int t = a + b;
            a = b;
            b = t;
        }
        return b;
    }
};