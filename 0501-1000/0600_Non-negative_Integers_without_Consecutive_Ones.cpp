class Solution {
public:
    int findIntegers(int num) {
        static vector<int> dp;
        if (dp.empty()) {
            dp = { 1, 2 };
            for (int i = 2; i < 32; i++)
                dp.push_back(dp[i - 1] + dp[i - 2]);
        }
        int curr = 0, res = 1;
        while ((1 << curr) <= num)
            curr++;
        for (int i = curr - 1; i >= 0; i--) {
            if (num & (1 << i)) {
                res += dp[i];
                if (num & (1 << (i + 1))) {
                    res--;
                    break;
                }
            }
        }
        return res;
    }
};