class Solution {
public:
    vector<int> largestDivisibleSubset(vector<int>& nums) {
        int n = nums.size(), head = n - 1;
        sort(nums.begin(), nums.end());
        vector<int> dp(n, 1), ne(n, -1);
        for (int i = n - 2; i >= 0; i--) {
            for (int j = i + 1; j < n; j++) {
                if (nums[j] % nums[i] == 0 && dp[i] <= dp[j]) {
                    dp[i] = dp[j] + 1;
                    ne[i] = j;
                    if (dp[i] > dp[head])
                        head = i;
                }
            }
        }
        vector<int> res;
        while (head >= 0) {
            res.push_back(nums[head]);
            head = ne[head];
        }
        return res;
    }
};