class Solution {
public:
    int maximumSwap(int num) {
        for (int a = 1e8; a >= 10; a /= 10) {
            if (num % a == num)
                continue;
            int d1 = num / a % 10, d2 = -1, b;
            for (int b2 = a / 10; b2; b2 /= 10) {
                int d22 = num / b2 % 10;
                if (d2 <= d22) {
                    d2 = d22;
                    b = b2;
                }
            }
            if (d1 < d2)
                return num + (d2 - d1) * (a - b);
        }
        return num;
    }
};