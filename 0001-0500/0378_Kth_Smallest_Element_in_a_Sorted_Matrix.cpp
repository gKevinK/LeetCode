class Solution {
public:
    int kthSmallest(vector<vector<int>>& matrix, int k) {
        int n = matrix.size(), lo = matrix[0][0], hi = matrix[n-1][n-1];
        while (lo < hi) {
            int count = 0, mi = lo + (hi - lo) / 2;
            for (int i = 0, j = n - 1; i < n && j >= 0; i++) {
                while (j >= 0 && matrix[i][j] > mi) j--;
                count += j + 1;
            }
            if (count < k) lo = mi + 1;
            else hi = mi;
        }
        return lo;
    }
};