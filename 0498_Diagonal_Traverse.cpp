class Solution {
public:
    vector<int> findDiagonalOrder(vector<vector<int>>& matrix) {
        if (matrix.empty()) return { };
        int m = matrix.size(), n = matrix[0].size();
        vector<int> r;
        for (int x = 0; x < m + n - 1; x++) {
            if (x % 2 == 0) {
                int i = min(x, m - 1), j = max(0, x - m + 1);
                while (i >= 0 && j < n)
                    r.push_back(matrix[i--][j++]);
            } else {
                int i = max(x - n + 1, 0), j = min(x, n - 1);
                while (i < m && j >= 0)
                    r.push_back(matrix[i++][j--]);
            }
        }
        return r;
    }
};