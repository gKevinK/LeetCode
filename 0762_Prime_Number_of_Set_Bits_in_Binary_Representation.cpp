class Solution {
    int bits(int n, vector<int>& dp) {
        if (n > 0 && dp[n] == 0)
            dp[n] = bits(n >> 1, dp) + n % 2;
        return dp[n];
    }
public:
    int countPrimeSetBits(int L, int R) {
        unordered_set<int> prime = { 2, 3, 5, 7, 11, 13, 17, 19, 23, 29 };
        vector<int> dp(R + 1, 0);
        int count = 0;
        for (int i = L; i <= R; i++) {
            if (prime.find(bits(i, dp)) != prime.end())
                count++;
        }
        return count;
    }
};