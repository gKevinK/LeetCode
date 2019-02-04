class Solution {
public:
    vector<vector<int>> matrixReshape(vector<vector<int>>& nums, int r, int c) {
        int m = nums.size(), n = m ? nums[0].size() : 0;
        if (m * n != r * c) return nums;
        vector<vector<int>> res(r, vector<int>(c));
        for (int i = 0, j = 0; i < m * n; i++, j++) {
            res[j / c][j % c] = nums[i / n][i % n];
        }
        return res;
    }
};