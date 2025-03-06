class Solution {
public:
    int maxCoins(vector<int>& nums) {
        if (nums.empty()) return 0;
        nums.insert(nums.begin(), 1);
        nums.push_back(1);
        int n = nums.size();
        vector<vector<int>> mat(n, vector<int>(n, 0));
        for (int d = 1; d < n - 1; d++) {
            for (int i = 1; i + d < n; i++) {
                int j = i + d;
                for (int k = 0; k < d; k++)
                    mat[i][j] = max(mat[i][j], mat[i][i + k] + nums[i - 1] * nums[i + k] * nums[j] + mat[i + k + 1][j]);
            }
        }
        return mat[1][n-1];
    }
};