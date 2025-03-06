class Solution {
public:
    int clumsy(int N) {
        int op = 0, r1 = 0, r2 = -N;
        for (int i = N - 1; i > 0; --i) {
            if (op == 0)
                r2 *= i;
            else if (op == 1)
                r2 /= i;
            else if (op == 2) {
                r1 -= r2;
                r1 += i;
                r2 = 0;
            } else
                r2 = i;
            op = (op + 1) % 4;
        }
        return r1 - r2;
    }
};