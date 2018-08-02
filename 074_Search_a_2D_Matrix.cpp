class Solution {
public:
    bool searchMatrix(vector<vector<int>>& matrix, int target) {
        if (matrix.empty() || matrix[0].empty()) return false;
        int m = matrix.size(), n = matrix[0].size();
        int lo = 0, hi = m * n;
        while (lo < hi) {
            int mi = (hi - lo) / 2 + lo;
            if (matrix[mi / n][mi % n] < target) lo = mi + 1;
            else hi = mi;
        }
        return lo < m * n && matrix[lo / n][lo % n] == target;
    }
};