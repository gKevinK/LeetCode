class Solution {
public:
    int consecutiveNumbersSum(int N) {
        int s = 0;
        for (int i = 1; i <= N; i++) {
            if (N / i - (i - 1) / 2 <= 0)
                break;
            if (N % i * 2 == (i + 1) % 2 * i)
                s++;
        }
        return s;
    }
};