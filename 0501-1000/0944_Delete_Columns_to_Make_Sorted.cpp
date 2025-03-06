class Solution {
public:
    int minDeletionSize(vector<string>& A) {
        int r = 0, m = A.size(), n = A[0].size();
        for (int j = 0; j < n; j++)
            for (int i = 1; i < m; i++)
                if (A[i][j] < A[i-1][j]) {
                    r++;
                    break;
                }
        return r;
    }
};