class Solution {
public:
    bool searchMatrix(vector<vector<int>>& matrix, int target) {
        if (matrix.empty() || matrix[0].empty()) return false;
        int m = matrix.size(), n = matrix[0].size(), i = 0, j = n - 1;
        while (i < m && j >= 0) {
            while (i + 1 < m && matrix[i][j] < target) i++;
            if (matrix[i][j] < target) return false;
            while (j > 0 && matrix[i][j] > target) j--;
            if (matrix[i][j] > target) return false;
            if (matrix[i][j] == target) return true;
        }
        return false;
    }
};