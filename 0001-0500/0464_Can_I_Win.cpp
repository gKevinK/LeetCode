class Solution {
public:
    bool canIWin(int maxChoosableInteger, int desiredTotal) {
        if (desiredTotal <= 1)
            return true;
        int sum = maxChoosableInteger * (1 + maxChoosableInteger) / 2;
        if (sum < desiredTotal)
            return false;
        if (sum == desiredTotal)
            return maxChoosableInteger % 2 == 1;
        std::memset(&dp[0], 0, 1 << maxChoosableInteger);

        return dfs(maxChoosableInteger, desiredTotal, 0) == 1;
    }

private:
    std::array<int8_t, 1 << 20> dp;

    int8_t dfs(int max, int total, int state) {
        if (dp[state] != 0)
            return dp[state];
        if (total <= 0)
            return -1;
        for (int i = 0; i < max; i++) {
            int mask = 1 << i;
            if ((state & mask) == 0 && dfs(max, total - i - 1, state | mask) == -1) {
                dp[state] = 1;
                return 1;
            }
        }
        dp[state] = -1;
        return -1;
    }
};